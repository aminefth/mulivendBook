package services

import (
	"database/sql"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/redis/go-redis/v9"
	"vendor-service/internal/models"
)

type AnalyticsService struct {
	db    *sql.DB
	redis *redis.Client
}

func NewAnalyticsService(db *sql.DB, redis *redis.Client) *AnalyticsService {
	return &AnalyticsService{
		db:    db,
		redis: redis,
	}
}

func (s *AnalyticsService) GetVendorAnalytics(vendorID uuid.UUID, startDate, endDate time.Time) ([]models.VendorAnalytics, error) {
	query := `
		SELECT vendor_id, date, total_views, total_orders, total_revenue,
			   total_commission, avg_order_value, conversion_rate, created_at
		FROM analytics.vendor_metrics
		WHERE vendor_id = $1 AND date >= $2 AND date <= $3
		ORDER BY date DESC
	`

	rows, err := s.db.Query(query, vendorID, startDate, endDate)
	if err != nil {
		return nil, fmt.Errorf("failed to get vendor analytics: %w", err)
	}
	defer rows.Close()

	var analytics []models.VendorAnalytics
	for rows.Next() {
		var metric models.VendorAnalytics
		err := rows.Scan(
			&metric.VendorID, &metric.Date, &metric.TotalViews, &metric.TotalOrders,
			&metric.TotalRevenue, &metric.TotalCommission, &metric.AvgOrderValue,
			&metric.ConversionRate, &metric.CreatedAt,
		)
		if err != nil {
			return nil, fmt.Errorf("failed to scan analytics row: %w", err)
		}
		analytics = append(analytics, metric)
	}

	return analytics, nil
}

func (s *AnalyticsService) GetVendorDashboard(vendorID uuid.UUID) (*models.VendorDashboard, error) {
	dashboard := &models.VendorDashboard{}

	// Get basic metrics from vendor table
	vendorQuery := `
		SELECT rating, total_sales, total_orders
		FROM marketplace.vendors
		WHERE id = $1
	`

	err := s.db.QueryRow(vendorQuery, vendorID).Scan(
		&dashboard.Rating,
		&dashboard.TotalRevenue,
		&dashboard.TotalOrders,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to get vendor metrics: %w", err)
	}

	// Get product count
	productQuery := `
		SELECT COUNT(*) FROM marketplace.products 
		WHERE vendor_id = $1 AND status = 'active'
	`
	s.db.QueryRow(productQuery, vendorID).Scan(&dashboard.TotalProducts)

	// Get pending payouts
	payoutQuery := `
		SELECT COALESCE(SUM(amount), 0) FROM marketplace.vendor_payouts 
		WHERE vendor_id = $1 AND status = 'pending'
	`
	s.db.QueryRow(payoutQuery, vendorID).Scan(&dashboard.PendingPayouts)

	// Get revenue chart data (last 30 days)
	chartQuery := `
		SELECT date, total_revenue
		FROM analytics.vendor_metrics
		WHERE vendor_id = $1 AND date >= CURRENT_DATE - INTERVAL '30 days'
		ORDER BY date ASC
	`

	rows, err := s.db.Query(chartQuery, vendorID)
	if err == nil {
		defer rows.Close()
		for rows.Next() {
			var chartData models.ChartData
			var date time.Time
			rows.Scan(&date, &chartData.Value)
			chartData.Date = date.Format("2006-01-02")
			dashboard.RevenueChart = append(dashboard.RevenueChart, chartData)
		}
	}

	// Get orders chart data (last 30 days)
	ordersChartQuery := `
		SELECT date, total_orders
		FROM analytics.vendor_metrics
		WHERE vendor_id = $1 AND date >= CURRENT_DATE - INTERVAL '30 days'
		ORDER BY date ASC
	`

	rows, err = s.db.Query(ordersChartQuery, vendorID)
	if err == nil {
		defer rows.Close()
		for rows.Next() {
			var chartData models.ChartData
			var date time.Time
			rows.Scan(&date, &chartData.Value)
			chartData.Date = date.Format("2006-01-02")
			dashboard.OrdersChart = append(dashboard.OrdersChart, chartData)
		}
	}

	// Get recent orders (placeholder - would need order service integration)
	dashboard.RecentOrders = []interface{}{}

	// Get top products (placeholder - would need catalog service integration)
	dashboard.TopProducts = []interface{}{}

	return dashboard, nil
}

func (s *AnalyticsService) GetOverviewAnalytics() (map[string]interface{}, error) {
	overview := make(map[string]interface{})

	// Total vendors
	var totalVendors int64
	s.db.QueryRow("SELECT COUNT(*) FROM marketplace.vendors").Scan(&totalVendors)
	overview["total_vendors"] = totalVendors

	// Active vendors
	var activeVendors int64
	s.db.QueryRow("SELECT COUNT(*) FROM marketplace.vendors WHERE status = 'active'").Scan(&activeVendors)
	overview["active_vendors"] = activeVendors

	// Pending vendors
	var pendingVendors int64
	s.db.QueryRow("SELECT COUNT(*) FROM marketplace.vendors WHERE status = 'pending'").Scan(&pendingVendors)
	overview["pending_vendors"] = pendingVendors

	// Total revenue
	var totalRevenue float64
	s.db.QueryRow("SELECT COALESCE(SUM(total_sales), 0) FROM marketplace.vendors").Scan(&totalRevenue)
	overview["total_revenue"] = totalRevenue

	// Total orders
	var totalOrders int64
	s.db.QueryRow("SELECT COALESCE(SUM(total_orders), 0) FROM marketplace.vendors").Scan(&totalOrders)
	overview["total_orders"] = totalOrders

	// Average commission rate
	var avgCommission float64
	s.db.QueryRow("SELECT COALESCE(AVG(commission_rate), 0) FROM marketplace.vendors WHERE status = 'active'").Scan(&avgCommission)
	overview["avg_commission_rate"] = avgCommission

	// Growth metrics (last 30 days)
	var newVendorsThisMonth int64
	s.db.QueryRow(`
		SELECT COUNT(*) FROM marketplace.vendors 
		WHERE created_at >= CURRENT_DATE - INTERVAL '30 days'
	`).Scan(&newVendorsThisMonth)
	overview["new_vendors_this_month"] = newVendorsThisMonth

	return overview, nil
}

func (s *AnalyticsService) RecordVendorView(vendorID uuid.UUID) error {
	// Increment view count in Redis
	key := fmt.Sprintf("vendor_views:%s:%s", vendorID.String(), time.Now().Format("2006-01-02"))
	return s.redis.Incr(nil, key).Err()
}

func (s *AnalyticsService) FlushDailyMetrics() error {
	// This would be called by a cron job to flush Redis counters to PostgreSQL
	// Implementation would:
	// 1. Get all Redis keys for the previous day
	// 2. Aggregate the data
	// 3. Insert/update records in analytics.vendor_metrics
	// 4. Clean up Redis keys

	// Placeholder implementation
	return nil
}
