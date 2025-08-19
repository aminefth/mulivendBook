package models

import (
	"database/sql/driver"
	"encoding/json"
	"time"

	"github.com/google/uuid"
)

type VendorStatus string

const (
	VendorStatusPending   VendorStatus = "pending"
	VendorStatusActive    VendorStatus = "active"
	VendorStatusSuspended VendorStatus = "suspended"
	VendorStatusRejected  VendorStatus = "rejected"
)

type BusinessType string

const (
	BusinessTypeIndividual  BusinessType = "individual"
	BusinessTypeCompany     BusinessType = "company"
	BusinessTypeCooperative BusinessType = "cooperative"
)

type JSONB map[string]interface{}

func (j JSONB) Value() (driver.Value, error) {
	return json.Marshal(j)
}

func (j *JSONB) Scan(value interface{}) error {
	if value == nil {
		*j = make(JSONB)
		return nil
	}
	
	bytes, ok := value.([]byte)
	if !ok {
		return nil
	}
	
	return json.Unmarshal(bytes, j)
}

type Vendor struct {
	ID                     uuid.UUID    `json:"id" db:"id"`
	UserID                 uuid.UUID    `json:"user_id" db:"user_id"`
	CompanyName            string       `json:"company_name" db:"company_name"`
	BusinessType           BusinessType `json:"business_type" db:"business_type"`
	TaxID                  *string      `json:"tax_id" db:"tax_id"`
	RegistrationNumber     *string      `json:"registration_number" db:"registration_number"`
	Status                 VendorStatus `json:"status" db:"status"`
	CommissionRate         float64      `json:"commission_rate" db:"commission_rate"`
	Rating                 float64      `json:"rating" db:"rating"`
	TotalSales             float64      `json:"total_sales" db:"total_sales"`
	TotalOrders            int          `json:"total_orders" db:"total_orders"`
	Address                JSONB        `json:"address" db:"address"`
	BankDetails            JSONB        `json:"bank_details" db:"bank_details"`
	VerificationDocuments  JSONB        `json:"verification_documents" db:"verification_documents"`
	Settings               JSONB        `json:"settings" db:"settings"`
	CreatedAt              time.Time    `json:"created_at" db:"created_at"`
	UpdatedAt              time.Time    `json:"updated_at" db:"updated_at"`
}

type VendorRegistrationRequest struct {
	UserID             uuid.UUID    `json:"user_id" validate:"required"`
	CompanyName        string       `json:"company_name" validate:"required,min=2,max=255"`
	BusinessType       BusinessType `json:"business_type" validate:"required,oneof=individual company cooperative"`
	TaxID              *string      `json:"tax_id" validate:"omitempty,min=5,max=50"`
	RegistrationNumber *string      `json:"registration_number" validate:"omitempty,max=100"`
	Address            Address      `json:"address" validate:"required"`
	BankDetails        BankDetails  `json:"bank_details" validate:"required"`
}

type Address struct {
	Street     string `json:"street" validate:"required"`
	City       string `json:"city" validate:"required"`
	State      string `json:"state" validate:"required"`
	PostalCode string `json:"postal_code" validate:"required"`
	Country    string `json:"country" validate:"required"`
}

type BankDetails struct {
	AccountName   string `json:"account_name" validate:"required"`
	AccountNumber string `json:"account_number" validate:"required"`
	BankName      string `json:"bank_name" validate:"required"`
	BankCode      string `json:"bank_code" validate:"required"`
	IBAN          string `json:"iban" validate:"omitempty"`
	SwiftCode     string `json:"swift_code" validate:"omitempty"`
}

type VendorUpdateRequest struct {
	CompanyName        *string      `json:"company_name" validate:"omitempty,min=2,max=255"`
	BusinessType       *BusinessType `json:"business_type" validate:"omitempty,oneof=individual company cooperative"`
	TaxID              *string      `json:"tax_id" validate:"omitempty,min=5,max=50"`
	RegistrationNumber *string      `json:"registration_number" validate:"omitempty,max=100"`
	Address            *Address     `json:"address"`
	BankDetails        *BankDetails `json:"bank_details"`
	Settings           JSONB        `json:"settings"`
}

type VendorListResponse struct {
	Vendors    []Vendor `json:"vendors"`
	Total      int64    `json:"total"`
	Page       int      `json:"page"`
	Limit      int      `json:"limit"`
	TotalPages int      `json:"total_pages"`
}

type VendorAnalytics struct {
	VendorID        uuid.UUID `json:"vendor_id" db:"vendor_id"`
	Date            time.Time `json:"date" db:"date"`
	TotalViews      int       `json:"total_views" db:"total_views"`
	TotalOrders     int       `json:"total_orders" db:"total_orders"`
	TotalRevenue    float64   `json:"total_revenue" db:"total_revenue"`
	TotalCommission float64   `json:"total_commission" db:"total_commission"`
	AvgOrderValue   float64   `json:"avg_order_value" db:"avg_order_value"`
	ConversionRate  float64   `json:"conversion_rate" db:"conversion_rate"`
	CreatedAt       time.Time `json:"created_at" db:"created_at"`
}

type VendorDashboard struct {
	TotalRevenue     float64           `json:"total_revenue"`
	TotalOrders      int               `json:"total_orders"`
	TotalProducts    int               `json:"total_products"`
	PendingPayouts   float64           `json:"pending_payouts"`
	Rating           float64           `json:"rating"`
	RecentOrders     []interface{}     `json:"recent_orders"`
	TopProducts      []interface{}     `json:"top_products"`
	RevenueChart     []ChartData       `json:"revenue_chart"`
	OrdersChart      []ChartData       `json:"orders_chart"`
}

type ChartData struct {
	Date  string  `json:"date"`
	Value float64 `json:"value"`
}
