package services

import (
	"encoding/json"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/redis/go-redis/v9"
	"vendor-service/internal/models"
	"vendor-service/internal/repository"
)

type VendorService struct {
	repo  *repository.VendorRepository
	redis *redis.Client
}

func NewVendorService(repo *repository.VendorRepository, redis *redis.Client) *VendorService {
	return &VendorService{
		repo:  repo,
		redis: redis,
	}
}

func (s *VendorService) RegisterVendor(req *models.VendorRegistrationRequest) (*models.Vendor, error) {
	// Check if vendor already exists for this user
	existingVendor, err := s.repo.GetByUserID(req.UserID)
	if err == nil && existingVendor != nil {
		return nil, fmt.Errorf("vendor already exists for this user")
	}

	// Create new vendor
	vendor := &models.Vendor{
		ID:             uuid.New(),
		UserID:         req.UserID,
		CompanyName:    req.CompanyName,
		BusinessType:   req.BusinessType,
		TaxID:          req.TaxID,
		RegistrationNumber: req.RegistrationNumber,
		Status:         models.VendorStatusPending,
		CommissionRate: 15.0, // Default commission rate
		Rating:         0.0,
		TotalSales:     0.0,
		TotalOrders:    0,
		Address:        models.JSONB(map[string]interface{}{
			"street":      req.Address.Street,
			"city":        req.Address.City,
			"state":       req.Address.State,
			"postal_code": req.Address.PostalCode,
			"country":     req.Address.Country,
		}),
		BankDetails: models.JSONB(map[string]interface{}{
			"account_name":   req.BankDetails.AccountName,
			"account_number": req.BankDetails.AccountNumber,
			"bank_name":      req.BankDetails.BankName,
			"bank_code":      req.BankDetails.BankCode,
			"iban":           req.BankDetails.IBAN,
			"swift_code":     req.BankDetails.SwiftCode,
		}),
		VerificationDocuments: models.JSONB{},
		Settings:              models.JSONB{},
	}

	err = s.repo.Create(vendor)
	if err != nil {
		return nil, fmt.Errorf("failed to create vendor: %w", err)
	}

	// Cache vendor data
	s.cacheVendor(vendor)

	return vendor, nil
}

func (s *VendorService) GetVendor(id uuid.UUID) (*models.Vendor, error) {
	// Try to get from cache first
	if vendor := s.getCachedVendor(id); vendor != nil {
		return vendor, nil
	}

	// Get from database
	vendor, err := s.repo.GetByID(id)
	if err != nil {
		return nil, fmt.Errorf("vendor not found: %w", err)
	}

	// Cache the result
	s.cacheVendor(vendor)

	return vendor, nil
}

func (s *VendorService) GetVendorByUserID(userID uuid.UUID) (*models.Vendor, error) {
	vendor, err := s.repo.GetByUserID(userID)
	if err != nil {
		return nil, fmt.Errorf("vendor not found: %w", err)
	}

	return vendor, nil
}

func (s *VendorService) UpdateVendor(id uuid.UUID, req *models.VendorUpdateRequest) (*models.Vendor, error) {
	// Get existing vendor
	vendor, err := s.repo.GetByID(id)
	if err != nil {
		return nil, fmt.Errorf("vendor not found: %w", err)
	}

	// Update fields if provided
	if req.CompanyName != nil {
		vendor.CompanyName = *req.CompanyName
	}
	if req.BusinessType != nil {
		vendor.BusinessType = *req.BusinessType
	}
	if req.TaxID != nil {
		vendor.TaxID = req.TaxID
	}
	if req.RegistrationNumber != nil {
		vendor.RegistrationNumber = req.RegistrationNumber
	}
	if req.Address != nil {
		vendor.Address = models.JSONB(map[string]interface{}{
			"street":      req.Address.Street,
			"city":        req.Address.City,
			"state":       req.Address.State,
			"postal_code": req.Address.PostalCode,
			"country":     req.Address.Country,
		})
	}
	if req.BankDetails != nil {
		vendor.BankDetails = models.JSONB(map[string]interface{}{
			"account_name":   req.BankDetails.AccountName,
			"account_number": req.BankDetails.AccountNumber,
			"bank_name":      req.BankDetails.BankName,
			"bank_code":      req.BankDetails.BankCode,
			"iban":           req.BankDetails.IBAN,
			"swift_code":     req.BankDetails.SwiftCode,
		})
	}
	if req.Settings != nil {
		vendor.Settings = req.Settings
	}

	// Update in database
	err = s.repo.Update(vendor)
	if err != nil {
		return nil, fmt.Errorf("failed to update vendor: %w", err)
	}

	// Update cache
	s.cacheVendor(vendor)

	return vendor, nil
}

func (s *VendorService) ListVendors(page, limit int, status *models.VendorStatus, search *string) (*models.VendorListResponse, error) {
	vendors, total, err := s.repo.List(page, limit, status, search)
	if err != nil {
		return nil, fmt.Errorf("failed to list vendors: %w", err)
	}

	totalPages := int((total + int64(limit) - 1) / int64(limit))

	return &models.VendorListResponse{
		Vendors:    vendors,
		Total:      total,
		Page:       page,
		Limit:      limit,
		TotalPages: totalPages,
	}, nil
}

func (s *VendorService) ApproveVendor(id uuid.UUID) error {
	err := s.repo.UpdateStatus(id, models.VendorStatusActive)
	if err != nil {
		return fmt.Errorf("failed to approve vendor: %w", err)
	}

	// Clear cache
	s.clearVendorCache(id)

	return nil
}

func (s *VendorService) SuspendVendor(id uuid.UUID) error {
	err := s.repo.UpdateStatus(id, models.VendorStatusSuspended)
	if err != nil {
		return fmt.Errorf("failed to suspend vendor: %w", err)
	}

	// Clear cache
	s.clearVendorCache(id)

	return nil
}

func (s *VendorService) GetVendorAnalytics(vendorID uuid.UUID, startDate, endDate time.Time) ([]models.VendorAnalytics, error) {
	return s.repo.GetAnalytics(vendorID, startDate, endDate)
}

func (s *VendorService) GetVendorDashboard(vendorID uuid.UUID) (*models.VendorDashboard, error) {
	return s.repo.GetDashboardData(vendorID)
}

// Cache helper methods
func (s *VendorService) cacheVendor(vendor *models.Vendor) {
	key := fmt.Sprintf("vendor:%s", vendor.ID.String())
	data, _ := json.Marshal(vendor)
	s.redis.Set(nil, key, data, time.Hour)
}

func (s *VendorService) getCachedVendor(id uuid.UUID) *models.Vendor {
	key := fmt.Sprintf("vendor:%s", id.String())
	data, err := s.redis.Get(nil, key).Result()
	if err != nil {
		return nil
	}

	var vendor models.Vendor
	if err := json.Unmarshal([]byte(data), &vendor); err != nil {
		return nil
	}

	return &vendor
}

func (s *VendorService) clearVendorCache(id uuid.UUID) {
	key := fmt.Sprintf("vendor:%s", id.String())
	s.redis.Del(nil, key)
}
