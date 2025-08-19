package handlers

import (
	"strconv"
	"time"

	"github.com/go-playground/validator/v10"
	"github.com/gofiber/fiber/v2"
	"github.com/google/uuid"
	"vendor-service/internal/models"
	"vendor-service/internal/services"
)

type VendorHandler struct {
	vendorService    *services.VendorService
	analyticsService *services.AnalyticsService
	validator        *validator.Validate
}

func NewVendorHandler(vendorService *services.VendorService, analyticsService *services.AnalyticsService) *VendorHandler {
	return &VendorHandler{
		vendorService:    vendorService,
		analyticsService: analyticsService,
		validator:        validator.New(),
	}
}

// RegisterVendor handles vendor registration
func (h *VendorHandler) RegisterVendor(c *fiber.Ctx) error {
	var req models.VendorRegistrationRequest
	if err := c.BodyParser(&req); err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid request body",
		})
	}

	// Get user ID from context
	userID := c.Locals("user_id").(string)
	parsedUserID, err := uuid.Parse(userID)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid user ID",
		})
	}
	req.UserID = parsedUserID

	// Validate request
	if err := h.validator.Struct(&req); err != nil {
		return c.Status(422).JSON(fiber.Map{
			"error": "Validation failed",
			"details": err.Error(),
		})
	}

	// Register vendor
	vendor, err := h.vendorService.RegisterVendor(&req)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(201).JSON(fiber.Map{
		"message": "Vendor registration submitted successfully",
		"vendor":  vendor,
	})
}

// GetVendor retrieves vendor by ID
func (h *VendorHandler) GetVendor(c *fiber.Ctx) error {
	id := c.Params("id")
	vendorID, err := uuid.Parse(id)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid vendor ID",
		})
	}

	vendor, err := h.vendorService.GetVendor(vendorID)
	if err != nil {
		return c.Status(404).JSON(fiber.Map{
			"error": "Vendor not found",
		})
	}

	// Record view for analytics
	h.analyticsService.RecordVendorView(vendorID)

	return c.JSON(fiber.Map{
		"vendor": vendor,
	})
}

// GetMyVendor retrieves current user's vendor profile
func (h *VendorHandler) GetMyVendor(c *fiber.Ctx) error {
	userID := c.Locals("user_id").(string)
	parsedUserID, err := uuid.Parse(userID)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid user ID",
		})
	}

	vendor, err := h.vendorService.GetVendorByUserID(parsedUserID)
	if err != nil {
		return c.Status(404).JSON(fiber.Map{
			"error": "Vendor profile not found",
		})
	}

	return c.JSON(fiber.Map{
		"vendor": vendor,
	})
}

// UpdateVendor updates vendor information
func (h *VendorHandler) UpdateVendor(c *fiber.Ctx) error {
	id := c.Params("id")
	vendorID, err := uuid.Parse(id)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid vendor ID",
		})
	}

	var req models.VendorUpdateRequest
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

	vendor, err := h.vendorService.UpdateVendor(vendorID, &req)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.JSON(fiber.Map{
		"message": "Vendor updated successfully",
		"vendor":  vendor,
	})
}

// ListVendors lists vendors with pagination and filters
func (h *VendorHandler) ListVendors(c *fiber.Ctx) error {
	page, _ := strconv.Atoi(c.Query("page", "1"))
	limit, _ := strconv.Atoi(c.Query("limit", "20"))
	search := c.Query("search")

	var status *models.VendorStatus
	if statusStr := c.Query("status"); statusStr != "" {
		s := models.VendorStatus(statusStr)
		status = &s
	}

	response, err := h.vendorService.ListVendors(page, limit, status, &search)
	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"error": "Failed to list vendors",
		})
	}

	return c.JSON(response)
}

// ApproveVendor approves a pending vendor
func (h *VendorHandler) ApproveVendor(c *fiber.Ctx) error {
	id := c.Params("id")
	vendorID, err := uuid.Parse(id)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid vendor ID",
		})
	}

	err = h.vendorService.ApproveVendor(vendorID)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.JSON(fiber.Map{
		"message": "Vendor approved successfully",
	})
}

// SuspendVendor suspends an active vendor
func (h *VendorHandler) SuspendVendor(c *fiber.Ctx) error {
	id := c.Params("id")
	vendorID, err := uuid.Parse(id)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid vendor ID",
		})
	}

	err = h.vendorService.SuspendVendor(vendorID)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.JSON(fiber.Map{
		"message": "Vendor suspended successfully",
	})
}

// GetVendorAnalytics retrieves vendor analytics data
func (h *VendorHandler) GetVendorAnalytics(c *fiber.Ctx) error {
	id := c.Params("id")
	vendorID, err := uuid.Parse(id)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid vendor ID",
		})
	}

	// Parse date range
	startDateStr := c.Query("start_date", time.Now().AddDate(0, -1, 0).Format("2006-01-02"))
	endDateStr := c.Query("end_date", time.Now().Format("2006-01-02"))

	startDate, err := time.Parse("2006-01-02", startDateStr)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid start date format",
		})
	}

	endDate, err := time.Parse("2006-01-02", endDateStr)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid end date format",
		})
	}

	analytics, err := h.vendorService.GetVendorAnalytics(vendorID, startDate, endDate)
	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"error": "Failed to get vendor analytics",
		})
	}

	return c.JSON(fiber.Map{
		"analytics": analytics,
	})
}

// GetVendorDashboard retrieves vendor dashboard data
func (h *VendorHandler) GetVendorDashboard(c *fiber.Ctx) error {
	id := c.Params("id")
	vendorID, err := uuid.Parse(id)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid vendor ID",
		})
	}

	dashboard, err := h.vendorService.GetVendorDashboard(vendorID)
	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"error": "Failed to get vendor dashboard",
		})
	}

	return c.JSON(fiber.Map{
		"dashboard": dashboard,
	})
}

// GetMyDashboard retrieves current vendor's dashboard
func (h *VendorHandler) GetMyDashboard(c *fiber.Ctx) error {
	userID := c.Locals("user_id").(string)
	parsedUserID, err := uuid.Parse(userID)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "Invalid user ID",
		})
	}

	// Get vendor by user ID first
	vendor, err := h.vendorService.GetVendorByUserID(parsedUserID)
	if err != nil {
		return c.Status(404).JSON(fiber.Map{
			"error": "Vendor profile not found",
		})
	}

	dashboard, err := h.vendorService.GetVendorDashboard(vendor.ID)
	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"error": "Failed to get dashboard data",
		})
	}

	return c.JSON(fiber.Map{
		"dashboard": dashboard,
	})
}
