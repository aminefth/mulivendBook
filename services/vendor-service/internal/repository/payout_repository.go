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

type PayoutRepository struct {
	db *sql.DB
}

func NewPayoutRepository(db *sql.DB) *PayoutRepository {
	return &PayoutRepository{db: db}
}

func (r *PayoutRepository) Create(payout *models.Payout) error {
	payoutDetailsJSON, _ := json.Marshal(payout.PayoutDetails)

	query := `
		INSERT INTO marketplace.vendor_payouts (
			id, vendor_id, amount, currency, status, payout_method, payout_details
		) VALUES ($1, $2, $3, $4, $5, $6, $7)
		RETURNING created_at
	`

	err := r.db.QueryRow(
		query,
		payout.ID, payout.VendorID, payout.Amount, payout.Currency,
		payout.Status, payout.PayoutMethod, payoutDetailsJSON,
	).Scan(&payout.CreatedAt)

	return err
}

func (r *PayoutRepository) GetByID(id uuid.UUID) (*models.Payout, error) {
	payout := &models.Payout{}
	var payoutDetailsJSON []byte

	query := `
		SELECT id, vendor_id, amount, currency, status, payout_method,
			   payout_details, processed_at, created_at
		FROM marketplace.vendor_payouts
		WHERE id = $1
	`

	err := r.db.QueryRow(query, id).Scan(
		&payout.ID, &payout.VendorID, &payout.Amount, &payout.Currency,
		&payout.Status, &payout.PayoutMethod, &payoutDetailsJSON,
		&payout.ProcessedAt, &payout.CreatedAt,
	)

	if err != nil {
		return nil, err
	}

	json.Unmarshal(payoutDetailsJSON, &payout.PayoutDetails)
	return payout, nil
}

func (r *PayoutRepository) List(vendorID *uuid.UUID, page, limit int, status *models.PayoutStatus) ([]models.Payout, int64, error) {
	offset := (page - 1) * limit
	
	// Build WHERE clause
	var conditions []string
	var args []interface{}
	argIndex := 1

	if vendorID != nil {
		conditions = append(conditions, fmt.Sprintf("vendor_id = $%d", argIndex))
		args = append(args, *vendorID)
		argIndex++
	}

	if status != nil {
		conditions = append(conditions, fmt.Sprintf("status = $%d", argIndex))
		args = append(args, *status)
		argIndex++
	}

	whereClause := ""
	if len(conditions) > 0 {
		whereClause = "WHERE " + strings.Join(conditions, " AND ")
	}

	// Get total count
	countQuery := fmt.Sprintf("SELECT COUNT(*) FROM marketplace.vendor_payouts %s", whereClause)
	var total int64
	err := r.db.QueryRow(countQuery, args...).Scan(&total)
	if err != nil {
		return nil, 0, err
	}

	// Get payouts
	query := fmt.Sprintf(`
		SELECT id, vendor_id, amount, currency, status, payout_method,
			   payout_details, processed_at, created_at
		FROM marketplace.vendor_payouts
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

	var payouts []models.Payout
	for rows.Next() {
		payout := models.Payout{}
		var payoutDetailsJSON []byte

		err := rows.Scan(
			&payout.ID, &payout.VendorID, &payout.Amount, &payout.Currency,
			&payout.Status, &payout.PayoutMethod, &payoutDetailsJSON,
			&payout.ProcessedAt, &payout.CreatedAt,
		)
		if err != nil {
			return nil, 0, err
		}

		json.Unmarshal(payoutDetailsJSON, &payout.PayoutDetails)
		payouts = append(payouts, payout)
	}

	return payouts, total, nil
}

func (r *PayoutRepository) UpdateStatus(id uuid.UUID, status models.PayoutStatus) error {
	var processedAt *time.Time
	if status == models.PayoutStatusCompleted || status == models.PayoutStatusFailed {
		now := time.Now()
		processedAt = &now
	}

	query := `
		UPDATE marketplace.vendor_payouts
		SET status = $2, processed_at = $3
		WHERE id = $1
	`

	_, err := r.db.Exec(query, id, status, processedAt)
	return err
}

func (r *PayoutRepository) GetPendingAmount(vendorID uuid.UUID) (float64, error) {
	var amount float64
	query := `
		SELECT COALESCE(SUM(amount), 0)
		FROM marketplace.vendor_payouts
		WHERE vendor_id = $1 AND status = 'pending'
	`

	err := r.db.QueryRow(query, vendorID).Scan(&amount)
	return amount, err
}
