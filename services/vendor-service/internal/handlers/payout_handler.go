package handlers

import (
	"strconv"

	"github.com/go-playground/validator/v10"
	"github.com/gofiber/fiber/v2"
	"github.com/google/uuid"
	"vendor-service/internal/models"
	"vendor-service/internal/services"
)

type PayoutHandler struct {
	payoutService *services.PayoutService
	vendorService *services.VendorService
	validator     *validator.Validate
}

func NewPayoutHandler(payoutService *services.PayoutService, vendorService *services.VendorService) *PayoutHandler {
	return &PayoutHandler{
		payoutService: payoutService,
		vendorService: vendorService,
		validator:     validator.New(),
	}
}

// CreatePayout creates a new payout request
func (h *PayoutHandler) CreatePayout(c *fiber.Ctx) error {
	var req models.CreatePayoutRequest
	if err := c.BodyParser(&req); err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid request body",
		})
	}

	// Validate request
	if err := h.validator.Struct(&req); err != nil {
		return c.Status(422).JSON(fiber.Map{
			"error": "Validation failed",
			"details": err.Error(),
		})
	}

	payout, err := h.payoutService.CreatePayout(&req)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(201).JSON(fiber.Map{
		"message": "Payout request created successfully",
		"payout":  payout,
	})
}

// GetPayout retrieves a payout by ID
func (h *PayoutHandler) GetPayout(c *fiber.Ctx) error {
	id := c.Params("id")
	payoutID, err := uuid.Parse(id)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid payout ID",
		})
	}

	payout, err := h.payoutService.GetPayout(payoutID)
	if err != nil {
		return c.Status(404).JSON(fiber.Map{
			"error": "Payout not found",
		})
	}

	return c.JSON(fiber.Map{
		"payout": payout,
	})
}

// ListPayouts lists payouts with pagination and filters
func (h *PayoutHandler) ListPayouts(c *fiber.Ctx) error {
	page, _ := strconv.Atoi(c.Query("page", "1"))
	limit, _ := strconv.Atoi(c.Query("limit", "20"))

	var vendorID *uuid.UUID
	if vendorIDStr := c.Query("vendor_id"); vendorIDStr != "" {
		id, err := uuid.Parse(vendorIDStr)
		if err != nil {
			return c.Status(400).JSON(fiber.Map{
				"error": "Invalid vendor ID",
			})
		}
		vendorID = &id
	}

	var status *models.PayoutStatus
	if statusStr := c.Query("status"); statusStr != "" {
		s := models.PayoutStatus(statusStr)
		status = &s
	}

	response, err := h.payoutService.ListPayouts(vendorID, page, limit, status)
	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"error": "Failed to list payouts",
		})
	}

	return c.JSON(response)
}

// ListMyPayouts lists current vendor's payouts
func (h *PayoutHandler) ListMyPayouts(c *fiber.Ctx) error {
	userID := c.Locals("user_id").(string)
	parsedUserID, err := uuid.Parse(userID)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid user ID",
		})
	}

	// Get vendor by user ID
	vendor, err := h.vendorService.GetVendorByUserID(parsedUserID)
	if err != nil {
		return c.Status(404).JSON(fiber.Map{
			"error": "Vendor profile not found",
		})
	}

	page, _ := strconv.Atoi(c.Query("page", "1"))
	limit, _ := strconv.Atoi(c.Query("limit", "20"))

	var status *models.PayoutStatus
	if statusStr := c.Query("status"); statusStr != "" {
		s := models.PayoutStatus(statusStr)
		status = &s
	}

	response, err := h.payoutService.ListPayouts(&vendor.ID, page, limit, status)
	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"error": "Failed to list payouts",
		})
	}

	return c.JSON(response)
}

// ProcessPayout processes a pending payout
func (h *PayoutHandler) ProcessPayout(c *fiber.Ctx) error {
	id := c.Params("id")
	payoutID, err := uuid.Parse(id)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid payout ID",
		})
	}

	err = h.payoutService.ProcessPayout(payoutID)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.JSON(fiber.Map{
		"message": "Payout processing initiated",
	})
}

// GetPendingAmount gets vendor's pending payout amount
func (h *PayoutHandler) GetPendingAmount(c *fiber.Ctx) error {
	userID := c.Locals("user_id").(string)
	parsedUserID, err := uuid.Parse(userID)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid user ID",
		})
	}

	// Get vendor by user ID
	vendor, err := h.vendorService.GetVendorByUserID(parsedUserID)
	if err != nil {
		return c.Status(404).JSON(fiber.Map{
			"error": "Vendor profile not found",
		})
	}

	amount, err := h.payoutService.GetPendingAmount(vendor.ID)
	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"error": "Failed to get pending amount",
		})
	}

	return c.JSON(fiber.Map{
		"pending_amount": amount,
		"currency":       "USD", // Default currency
	})
}
