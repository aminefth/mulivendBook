#!/bin/bash

# BookMarket Pro - Database Seeding Script
# This script runs all seed data files in the correct order

set -e

echo "🌱 Starting BookMarket Pro database seeding..."

# Database connection details
DB_HOST=${DB_HOST:-localhost}
DB_PORT=${DB_PORT:-5432}
DB_NAME=${DB_NAME:-bookmarket}
DB_USER=${DB_USER:-bookmarket}
DB_PASSWORD=${DB_PASSWORD:-bookmarket_dev}

# Check if PostgreSQL is running
echo "📡 Checking database connection..."
if ! pg_isready -h $DB_HOST -p $DB_PORT -U $DB_USER; then
    echo "❌ Database is not ready. Please start PostgreSQL first."
    exit 1
fi

echo "✅ Database connection successful"

# Run initialization script
echo "🏗️  Running database initialization..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -f init-db.sql

# Run seed data scripts in order
echo "👥 Seeding users, vendors, and categories..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -f seed-data.sql

echo "📚 Seeding products..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -f seed-products.sql

echo "🛒 Seeding orders, reviews, and analytics..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -f seed-orders-reviews.sql

echo "🎉 Database seeding completed successfully!"
echo ""
echo "📊 Seeded data summary:"
echo "   • 10 Users (1 admin, 6 customers, 3 vendors)"
echo "   • 4 Vendors (approved bookstores and publishers)"
echo "   • 12 Categories (main + subcategories)"
echo "   • 15 Products (diverse book collection)"
echo "   • 10 Product reviews (realistic ratings)"
echo "   • 5 Orders (various statuses)"
echo "   • 12 Vendor analytics records"
echo "   • 8 Vendor payout records"
echo ""
echo "🚀 Your BookMarket Pro development environment is ready!"
echo "   • Admin: admin@bookmarket.com"
echo "   • Customer: ahmed.hassan@gmail.com"
echo "   • Vendor: vendor@daralkitab.ma"
echo "   • Password for all: password123"
