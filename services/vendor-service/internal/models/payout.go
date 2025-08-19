package models

import (
	"time"

	"github.com/google/uuid"
)

type PayoutStatus string

const (
	PayoutStatusPending    PayoutStatus = "pending"
	PayoutStatusProcessing PayoutStatus = "processing"
	PayoutStatusCompleted  PayoutStatus = "completed"
	PayoutStatusFailed     PayoutStatus = "failed"
)

type PayoutMethod string

const (
	PayoutMethodBankTransfer PayoutMethod = "bank_transfer"
	PayoutMethodPayPal       PayoutMethod = "paypal"
	PayoutMethodStripe       PayoutMethod = "stripe"
)

type Payout struct {
	ID            uuid.UUID    `json:"id" db:"id"`
	VendorID      uuid.UUID    `json:"vendor_id" db:"vendor_id"`
	Amount        float64      `json:"amount" db:"amount"`
	Currency      string       `json:"currency" db:"currency"`
	Status        PayoutStatus `json:"status" db:"status"`
	PayoutMethod  PayoutMethod `json:"payout_method" db:"payout_method"`
	PayoutDetails JSONB        `json:"payout_details" db:"payout_details"`
	ProcessedAt   *time.Time   `json:"processed_at" db:"processed_at"`
	CreatedAt     time.Time    `json:"created_at" db:"created_at"`
}

type CreatePayoutRequest struct {
	VendorID      uuid.UUID    `json:"vendor_id" validate:"required"`
	Amount        float64      `json:"amount" validate:"required,gt=0"`
	Currency      string       `json:"currency" validate:"required,len=3"`
	PayoutMethod  PayoutMethod `json:"payout_method" validate:"required,oneof=bank_transfer paypal stripe"`
	PayoutDetails JSONB        `json:"payout_details" validate:"required"`
}

type PayoutListResponse struct {
	Payouts    []Payout `json:"payouts"`
	Total      int64    `json:"total"`
	Page       int      `json:"page"`
	Limit      int      `json:"limit"`
	TotalPages int      `json:"total_pages"`
}
