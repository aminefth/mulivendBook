.PHONY: help build test clean start-infra stop-infra start-all stop-all

# Default target
help:
	@echo "BookMarket Pro - Multi-Vendor Marketplace"
	@echo ""
	@echo "Available commands:"
	@echo "  start-infra     Start infrastructure services (databases, queues, etc.)"
	@echo "  stop-infra      Stop infrastructure services"
	@echo "  start-all       Start all services including microservices"
	@echo "  stop-all        Stop all services"
	@echo "  build           Build all microservices"
	@echo "  test            Run tests for all services"
	@echo "  clean           Clean build artifacts"
	@echo "  migrate         Run database migrations"
	@echo "  seed            Seed database with sample data"

# Infrastructure management
start-infra:
	@echo "Starting infrastructure services..."
	docker-compose up -d postgres redis elasticsearch clickhouse kafka zookeeper rabbitmq minio prometheus grafana
	@echo "Waiting for services to be ready..."
	@sleep 30
	@echo "Infrastructure services started successfully!"

stop-infra:
	@echo "Stopping infrastructure services..."
	docker-compose down
	@echo "Infrastructure services stopped!"

# Service management
start-all: start-infra
	@echo "Starting all microservices..."
	@make -C services/auth-service start &
	@make -C services/vendor-service start &
	@make -C services/catalog-service start &
	@make -C services/order-service start &
	@make -C services/payment-service start &
	@make -C services/search-service start &
	@make -C services/notification-service start &
	@make -C services/analytics-service start &
	@echo "All services started!"

stop-all:
	@echo "Stopping all services..."
	@pkill -f "auth-service\|vendor-service\|catalog-service\|order-service\|payment-service\|search-service\|notification-service\|analytics-service" || true
	@make stop-infra

# Build targets
build:
	@echo "Building all microservices..."
	@make -C services/auth-service build
	@make -C services/vendor-service build
	@make -C services/catalog-service build
	@make -C services/order-service build
	@make -C services/payment-service build
	@make -C services/search-service build
	@make -C services/notification-service build
	@make -C services/analytics-service build
	@echo "All services built successfully!"

# Test targets
test:
	@echo "Running tests for all services..."
	@make -C services/auth-service test
	@make -C services/vendor-service test
	@make -C services/catalog-service test
	@make -C services/order-service test
	@make -C services/payment-service test
	@make -C services/search-service test
	@make -C services/notification-service test
	@make -C services/analytics-service test
	@echo "All tests completed!"

# Database operations
migrate:
	@echo "Running database migrations..."
	@cd scripts && ./migrate.sh
	@echo "Migrations completed!"

seed:
	@echo "Seeding database with sample data..."
	@cd scripts && ./seed.sh
	@echo "Database seeded!"

# Cleanup
clean:
	@echo "Cleaning build artifacts..."
	@find . -name "target" -type d -exec rm -rf {} + 2>/dev/null || true
	@find . -name "node_modules" -type d -exec rm -rf {} + 2>/dev/null || true
	@find . -name "_build" -type d -exec rm -rf {} + 2>/dev/null || true
	@echo "Cleanup completed!"

# Development helpers
dev-setup:
	@echo "Setting up development environment..."
	@./scripts/dev-setup.sh
	@echo "Development environment ready!"

logs:
	@echo "Showing logs for all services..."
	docker-compose logs -f

status:
	@echo "Service status:"
	@docker-compose ps
