package handlers

import (
	"github.com/gofiber/fiber/v2"
	"vendor-service/internal/services"
)

type AnalyticsHandler struct {
	analyticsService *services.AnalyticsService
}

func NewAnalyticsHandler(analyticsService *services.AnalyticsService) *AnalyticsHandler {
	return &AnalyticsHandler{
		analyticsService: analyticsService,
	}
}

// GetOverviewAnalytics retrieves platform-wide analytics overview
func (h *AnalyticsHandler) GetOverviewAnalytics(c *fiber.Ctx) error {
	overview, err := h.analyticsService.GetOverviewAnalytics()
	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"error": "Failed to get overview analytics",
		})
	}

	return c.JSON(fiber.Map{
		"overview": overview,
	})
}
