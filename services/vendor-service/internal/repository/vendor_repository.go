package repository

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/google/uuid"
	"vendor-service/internal/models"
)

type VendorRepository struct {
	db *sql.DB
}

func NewVendorRepository(db *sql.DB) *VendorRepository {
	return &VendorRepository{db: db}
}

func (r *VendorRepository) Create(vendor *models.Vendor) error {
	addressJSON, _ := json.Marshal(vendor.Address)
	bankDetailsJSON, _ := json.Marshal(vendor.BankDetails)
	verificationDocsJSON, _ := json.Marshal(vendor.VerificationDocuments)
	settingsJSON, _ := json.Marshal(vendor.Settings)

	query := `
		INSERT INTO marketplace.vendors (
			id, user_id, company_name, business_type, tax_id, registration_number,
			status, commission_rate, address, bank_details, verification_documents, settings
		) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
		RETURNING created_at, updated_at
	`

	err := r.db.QueryRow(
		query,
		vendor.ID, vendor.UserID, vendor.CompanyName, vendor.BusinessType,
		vendor.TaxID, vendor.RegistrationNumber, vendor.Status, vendor.CommissionRate,
		addressJSON, bankDetailsJSON, verificationDocsJSON, settingsJSON,
	).Scan(&vendor.CreatedAt, &vendor.UpdatedAt)

	return err
}

func (r *VendorRepository) GetByID(id uuid.UUID) (*models.Vendor, error) {
	vendor := &models.Vendor{}
	var addressJSON, bankDetailsJSON, verificationDocsJSON, settingsJSON []byte

	query := `
		SELECT id, user_id, company_name, business_type, tax_id, registration_number,
			   status, commission_rate, rating, total_sales, total_orders,
			   address, bank_details, verification_documents, settings,
			   created_at, updated_at
		FROM marketplace.vendors
		WHERE id = $1
	`

	err := r.db.QueryRow(query, id).Scan(
		&vendor.ID, &vendor.UserID, &vendor.CompanyName, &vendor.BusinessType,
		&vendor.TaxID, &vendor.RegistrationNumber, &vendor.Status, &vendor.CommissionRate,
		&vendor.Rating, &vendor.TotalSales, &vendor.TotalOrders,
		&addressJSON, &bankDetailsJSON, &verificationDocsJSON, &settingsJSON,
		&vendor.CreatedAt, &vendor.UpdatedAt,
	)

	if err != nil {
		return nil, err
	}

	// Unmarshal JSON fields
	json.Unmarshal(addressJSON, &vendor.Address)
	json.Unmarshal(bankDetailsJSON, &vendor.BankDetails)
	json.Unmarshal(verificationDocsJSON, &vendor.VerificationDocuments)
	json.Unmarshal(settingsJSON, &vendor.Settings)

	return vendor, nil
}

func (r *VendorRepository) GetByUserID(userID uuid.UUID) (*models.Vendor, error) {
	vendor := &models.Vendor{}
	var addressJSON, bankDetailsJSON, verificationDocsJSON, settingsJSON []byte

	query := `
		SELECT id, user_id, company_name, business_type, tax_id, registration_number,
			   status, commission_rate, rating, total_sales, total_orders,
			   address, bank_details, verification_documents, settings,
			   created_at, updated_at
		FROM marketplace.vendors
		WHERE user_id = $1
	`

	err := r.db.QueryRow(query, userID).Scan(
		&vendor.ID, &vendor.UserID, &vendor.CompanyName, &vendor.BusinessType,
		&vendor.TaxID, &vendor.RegistrationNumber, &vendor.Status, &vendor.CommissionRate,
		&vendor.Rating, &vendor.TotalSales, &vendor.TotalOrders,
		&addressJSON, &bankDetailsJSON, &verificationDocsJSON, &settingsJSON,
		&vendor.CreatedAt, &vendor.UpdatedAt,
	)

	if err != nil {
		return nil, err
	}

	// Unmarshal JSON fields
	json.Unmarshal(addressJSON, &vendor.Address)
	json.Unmarshal(bankDetailsJSON, &vendor.BankDetails)
	json.Unmarshal(verificationDocsJSON, &vendor.VerificationDocuments)
	json.Unmarshal(settingsJSON, &vendor.Settings)

	return vendor, nil
}

func (r *VendorRepository) Update(vendor *models.Vendor) error {
	addressJSON, _ := json.Marshal(vendor.Address)
	bankDetailsJSON, _ := json.Marshal(vendor.BankDetails)
	verificationDocsJSON, _ := json.Marshal(vendor.VerificationDocuments)
	settingsJSON, _ := json.Marshal(vendor.Settings)

	query := `
		UPDATE marketplace.vendors
		SET company_name = $2, business_type = $3, tax_id = $4, registration_number = $5,
			status = $6, commission_rate = $7, rating = $8, total_sales = $9, total_orders = $10,
			address = $11, bank_details = $12, verification_documents = $13, settings = $14,
			updated_at = CURRENT_TIMESTAMP
		WHERE id = $1
		RETURNING updated_at
	`

	err := r.db.QueryRow(
		query,
		vendor.ID, vendor.CompanyName, vendor.BusinessType, vendor.TaxID, vendor.RegistrationNumber,
		vendor.Status, vendor.CommissionRate, vendor.Rating, vendor.TotalSales, vendor.TotalOrders,
		addressJSON, bankDetailsJSON, verificationDocsJSON, settingsJSON,
	).Scan(&vendor.UpdatedAt)

	return err
}

func (r *VendorRepository) List(page, limit int, status *models.VendorStatus, search *string) ([]models.Vendor, int64, error) {
	offset := (page - 1) * limit
	
	// Build WHERE clause
	var conditions []string
	var args []interface{}
	argIndex := 1

	if status != nil {
		conditions = append(conditions, fmt.Sprintf("status = $%d", argIndex))
		args = append(args, *status)
		argIndex++
	}

	if search != nil && *search != "" {
		conditions = append(conditions, fmt.Sprintf("(company_name ILIKE $%d OR tax_id ILIKE $%d)", argIndex, argIndex))
		args = append(args, "%"+*search+"%")
		argIndex++
	}

	whereClause := ""
	if len(conditions) > 0 {
		whereClause = "WHERE " + strings.Join(conditions, " AND ")
	}

	// Get total count
	countQuery := fmt.Sprintf("SELECT COUNT(*) FROM marketplace.vendors %s", whereClause)
	var total int64
	err := r.db.QueryRow(countQuery, args...).Scan(&total)
	if err != nil {
		return nil, 0, err
	}

	// Get vendors
	query := fmt.Sprintf(`
		SELECT id, user_id, company_name, business_type, tax_id, registration_number,
			   status, commission_rate, rating, total_sales, total_orders,
			   address, bank_details, verification_documents, settings,
			   created_at, updated_at
		FROM marketplace.vendors
		%s
		ORDER BY created_at DESC
		LIMIT $%d OFFSET $%d
	`, whereClause, argIndex, argIndex+1)

	args = append(args, limit, offset)

	rows, err := r.db.Query(query, args...)
	if err != nil {
		return nil, 0, err
	}
	defer rows.Close()

	var vendors []models.Vendor
	for rows.Next() {
		vendor := models.Vendor{}
		var addressJSON, bankDetailsJSON, verificationDocsJSON, settingsJSON []byte

		err := rows.Scan(
			&vendor.ID, &vendor.UserID, &vendor.CompanyName, &vendor.BusinessType,
			&vendor.TaxID, &vendor.RegistrationNumber, &vendor.Status, &vendor.CommissionRate,
			&vendor.Rating, &vendor.TotalSales, &vendor.TotalOrders,
			&addressJSON, &bankDetailsJSON, &verificationDocsJSON, &settingsJSON,
			&vendor.CreatedAt, &vendor.UpdatedAt,
		)
		if err != nil {
			return nil, 0, err
		}

		// Unmarshal JSON fields
		json.Unmarshal(addressJSON, &vendor.Address)
		json.Unmarshal(bankDetailsJSON, &vendor.BankDetails)
		json.Unmarshal(verificationDocsJSON, &vendor.VerificationDocuments)
		json.Unmarshal(settingsJSON, &vendor.Settings)

		vendors = append(vendors, vendor)
	}

	return vendors, total, nil
}

func (r *VendorRepository) UpdateStatus(id uuid.UUID, status models.VendorStatus) error {
	query := `
		UPDATE marketplace.vendors
		SET status = $2, updated_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`

	_, err := r.db.Exec(query, id, status)
	return err
}

func (r *VendorRepository) GetAnalytics(vendorID uuid.UUID, startDate, endDate time.Time) ([]models.VendorAnalytics, error) {
	query := `
		SELECT vendor_id, date, total_views, total_orders, total_revenue,
			   total_commission, avg_order_value, conversion_rate, created_at
		FROM analytics.vendor_metrics
		WHERE vendor_id = $1 AND date >= $2 AND date <= $3
		ORDER BY date DESC
	`

	rows, err := r.db.Query(query, vendorID, startDate, endDate)
	if err != nil {
		return nil, err
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
			return nil, err
		}
		analytics = append(analytics, metric)
	}

	return analytics, nil
}

func (r *VendorRepository) GetDashboardData(vendorID uuid.UUID) (*models.VendorDashboard, error) {
	dashboard := &models.VendorDashboard{}

	// Get basic metrics
	query := `
		SELECT 
			COALESCE(SUM(total_revenue), 0) as total_revenue,
			COALESCE(SUM(total_orders), 0) as total_orders,
			COALESCE(AVG(rating), 0) as rating
		FROM analytics.vendor_metrics vm
		JOIN marketplace.vendors v ON v.id = vm.vendor_id
		WHERE vm.vendor_id = $1
	`

	err := r.db.QueryRow(query, vendorID).Scan(
		&dashboard.TotalRevenue,
		&dashboard.TotalOrders,
		&dashboard.Rating,
	)
	if err != nil {
		return nil, err
	}

	// Get product count
	productQuery := `
		SELECT COUNT(*) FROM marketplace.products WHERE vendor_id = $1 AND status = 'active'
	`
	r.db.QueryRow(productQuery, vendorID).Scan(&dashboard.TotalProducts)

	// Get pending payouts
	payoutQuery := `
		SELECT COALESCE(SUM(amount), 0) FROM marketplace.vendor_payouts 
		WHERE vendor_id = $1 AND status = 'pending'
	`
	r.db.QueryRow(payoutQuery, vendorID).Scan(&dashboard.PendingPayouts)

	// Get revenue chart data (last 30 days)
	chartQuery := `
		SELECT date, total_revenue
		FROM analytics.vendor_metrics
		WHERE vendor_id = $1 AND date >= CURRENT_DATE - INTERVAL '30 days'
		ORDER BY date ASC
	`

	rows, err := r.db.Query(chartQuery, vendorID)
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

	return dashboard, nil
}
