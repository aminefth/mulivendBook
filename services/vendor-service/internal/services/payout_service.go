package services

import (
	"fmt"

	"github.com/google/uuid"
	"vendor-service/internal/models"
	"vendor-service/internal/repository"
)

type PayoutService struct {
	payoutRepo *repository.PayoutRepository
	vendorRepo *repository.VendorRepository
}

func NewPayoutService(payoutRepo *repository.PayoutRepository, vendorRepo *repository.VendorRepository) *PayoutService {
	return &PayoutService{
		payoutRepo: payoutRepo,
		vendorRepo: vendorRepo,
	}
}

func (s *PayoutService) CreatePayout(req *models.CreatePayoutRequest) (*models.Payout, error) {
	// Verify vendor exists and is active
	vendor, err := s.vendorRepo.GetByID(req.VendorID)
	if err != nil {
		return nil, fmt.Errorf("vendor not found: %w", err)
	}

	if vendor.Status != models.VendorStatusActive {
		return nil, fmt.Errorf("vendor is not active")
	}

	// Check if vendor has sufficient balance
	// This would typically check against order commissions
	// For now, we'll assume the amount is valid

	payout := &models.Payout{
		ID:            uuid.New(),
		VendorID:      req.VendorID,
		Amount:        req.Amount,
		Currency:      req.Currency,
		Status:        models.PayoutStatusPending,
		PayoutMethod:  req.PayoutMethod,
		PayoutDetails: req.PayoutDetails,
	}

	err = s.payoutRepo.Create(payout)
	if err != nil {
		return nil, fmt.Errorf("failed to create payout: %w", err)
	}

	return payout, nil
}

func (s *PayoutService) GetPayout(id uuid.UUID) (*models.Payout, error) {
	payout, err := s.payoutRepo.GetByID(id)
	if err != nil {
		return nil, fmt.Errorf("payout not found: %w", err)
	}

	return payout, nil
}

func (s *PayoutService) ListPayouts(vendorID *uuid.UUID, page, limit int, status *models.PayoutStatus) (*models.PayoutListResponse, error) {
	payouts, total, err := s.payoutRepo.List(vendorID, page, limit, status)
	if err != nil {
		return nil, fmt.Errorf("failed to list payouts: %w", err)
	}

	totalPages := int((total + int64(limit) - 1) / int64(limit))

	return &models.PayoutListResponse{
		Payouts:    payouts,
		Total:      total,
		Page:       page,
		Limit:      limit,
		TotalPages: totalPages,
	}, nil
}

func (s *PayoutService) ProcessPayout(id uuid.UUID) error {
	// Get payout
	payout, err := s.payoutRepo.GetByID(id)
	if err != nil {
		return fmt.Errorf("payout not found: %w", err)
	}

	if payout.Status != models.PayoutStatusPending {
		return fmt.Errorf("payout is not in pending status")
	}

	// Update status to processing
	err = s.payoutRepo.UpdateStatus(id, models.PayoutStatusProcessing)
	if err != nil {
		return fmt.Errorf("failed to update payout status: %w", err)
	}

	// Here you would integrate with payment processors
	// For now, we'll simulate processing and mark as completed
	go s.simulatePayoutProcessing(id)

	return nil
}

func (s *PayoutService) simulatePayoutProcessing(id uuid.UUID) {
	// Simulate processing time
	// time.Sleep(time.Second * 5)

	// In a real implementation, this would:
	// 1. Call payment processor API
	// 2. Handle success/failure responses
	// 3. Update status accordingly
	// 4. Send notifications

	// For simulation, mark as completed
	s.payoutRepo.UpdateStatus(id, models.PayoutStatusCompleted)
}

func (s *PayoutService) GetPendingAmount(vendorID uuid.UUID) (float64, error) {
	return s.payoutRepo.GetPendingAmount(vendorID)
}
