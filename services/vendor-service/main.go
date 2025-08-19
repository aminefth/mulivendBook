package main

import (
	"log"
	"os"
	"time"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/logger"
	"github.com/gofiber/fiber/v2/middleware/recover"
	"github.com/joho/godotenv"
	_ "github.com/lib/pq"
	"github.com/redis/go-redis/v9"
	"github.com/sirupsen/logrus"

	"vendor-service/internal/config"
	"vendor-service/internal/database"
	"vendor-service/internal/handlers"
	"vendor-service/internal/middleware"
	"vendor-service/internal/repository"
	"vendor-service/internal/services"
)

func main() {
	// Load environment variables
	if err := godotenv.Load(); err != nil {
		logrus.Warn("No .env file found")
	}

	// Load configuration
	cfg := config.Load()

	// Setup logging
	logrus.SetLevel(logrus.DebugLevel)
	logrus.SetFormatter(&logrus.JSONFormatter{})

	logrus.Info("Starting BookMarket Vendor Service")

	// Initialize database
	db, err := database.Connect(cfg.DatabaseURL)
	if err != nil {
		logrus.Fatal("Failed to connect to database: ", err)
	}
	defer db.Close()

	// Initialize Redis
	rdb := redis.NewClient(&redis.Options{
		Addr:     cfg.RedisURL,
		Password: "",
		DB:       0,
	})

	// Initialize repositories
	vendorRepo := repository.NewVendorRepository(db)
	payoutRepo := repository.NewPayoutRepository(db)

	// Initialize services
	vendorService := services.NewVendorService(vendorRepo, rdb)
	payoutService := services.NewPayoutService(payoutRepo, vendorRepo)
	analyticsService := services.NewAnalyticsService(db, rdb)

	// Initialize handlers
	vendorHandler := handlers.NewVendorHandler(vendorService, analyticsService)
	payoutHandler := handlers.NewPayoutHandler(payoutService, vendorService)
	analyticsHandler := handlers.NewAnalyticsHandler(analyticsService)

	// Initialize Fiber app
	app := fiber.New(fiber.Config{
		ErrorHandler: middleware.ErrorHandler,
		ReadTimeout:  time.Second * 30,
		WriteTimeout: time.Second * 30,
	})

	// Middleware
	app.Use(recover.New())
	app.Use(logger.New())
	app.Use(cors.New(cors.Config{
		AllowOrigins: "*",
		AllowMethods: "GET,POST,PUT,DELETE,OPTIONS",
		AllowHeaders: "Origin,Content-Type,Accept,Authorization",
	}))

	// Health checks
	app.Get("/health", func(c *fiber.Ctx) error {
		return c.JSON(fiber.Map{
			"status":    "healthy",
			"service":   "vendor-service",
			"version":   "1.0.0",
			"timestamp": time.Now().UTC().Format(time.RFC3339),
		})
	})

	app.Get("/ready", func(c *fiber.Ctx) error {
		// Check database connectivity
		if err := db.Ping(); err != nil {
			return c.Status(503).JSON(fiber.Map{
				"status": "not_ready",
				"error":  "database connection failed",
			})
		}

		// Check Redis connectivity
		if err := rdb.Ping(c.Context()).Err(); err != nil {
			return c.Status(503).JSON(fiber.Map{
				"status": "not_ready",
				"error":  "redis connection failed",
			})
		}

		return c.JSON(fiber.Map{
			"status": "ready",
			"checks": fiber.Map{
				"database": "healthy",
				"redis":    "healthy",
			},
			"timestamp": time.Now().UTC().Format(time.RFC3339),
		})
	})

	// Metrics endpoint
	app.Get("/metrics", func(c *fiber.Ctx) error {
		return c.JSON(fiber.Map{
			"service": "vendor-service",
			"metrics": fiber.Map{
				"uptime":     time.Since(time.Now()).String(),
				"timestamp": time.Now().UTC().Format(time.RFC3339),
			},
		})
	})

	// API routes
	api := app.Group("/api/v1")

	// Vendor routes
	vendors := api.Group("/vendors")
	vendors.Post("/register", middleware.AuthRequired(), vendorHandler.RegisterVendor)
	vendors.Get("/me", middleware.AuthRequired(), vendorHandler.GetMyVendor)
	vendors.Get("/me/dashboard", middleware.AuthRequired(), vendorHandler.GetMyDashboard)
	vendors.Get("/:id", middleware.AuthRequired(), vendorHandler.GetVendor)
	vendors.Put("/:id", middleware.AuthRequired(), vendorHandler.UpdateVendor)
	vendors.Get("/:id/analytics", middleware.AuthRequired(), vendorHandler.GetVendorAnalytics)
	vendors.Get("/:id/dashboard", middleware.AuthRequired(), vendorHandler.GetVendorDashboard)

	// Payout routes
	payouts := api.Group("/payouts", middleware.AuthRequired())
	payouts.Get("/", payoutHandler.ListPayouts)
	payouts.Post("/", payoutHandler.CreatePayout)
	payouts.Get("/:id", payoutHandler.GetPayout)
	payouts.Put("/:id/process", payoutHandler.ProcessPayout)

	// Admin routes
	admin := api.Group("/admin", middleware.AuthRequired(), middleware.AdminRequired())
	admin.Get("/vendors", vendorHandler.ListVendors)
	admin.Put("/vendors/:id/approve", vendorHandler.ApproveVendor)
	admin.Put("/vendors/:id/suspend", vendorHandler.SuspendVendor)
	admin.Get("/analytics/overview", analyticsHandler.GetOverviewAnalytics)

	// Start server
	port := os.Getenv("PORT")
	if port == "" {
		port = "8002"
	}

	logrus.Infof("Vendor service listening on port %s", port)
	log.Fatal(app.Listen(":" + port))
}
