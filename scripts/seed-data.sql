-- BookMarket Pro - Comprehensive Seed Data
-- This script populates the database with realistic book marketplace data

-- Clear existing data
TRUNCATE TABLE order_items, orders, product_reviews, products, categories, vendor_analytics, vendor_payouts, vendors, users CASCADE;

-- Reset sequences
ALTER SEQUENCE users_id_seq RESTART WITH 1;
ALTER SEQUENCE vendors_id_seq RESTART WITH 1;
ALTER SEQUENCE categories_id_seq RESTART WITH 1;
ALTER SEQUENCE products_id_seq RESTART WITH 1;
ALTER SEQUENCE orders_id_seq RESTART WITH 1;

-- Insert Users (Customers and Vendors)
INSERT INTO users (email, password_hash, full_name, role, is_verified, created_at, updated_at) VALUES
-- Admin Users
('admin@bookmarket.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj6hsxq5S/kS', 'BookMarket Admin', 'admin', true, NOW(), NOW()),

-- Customer Users
('ahmed.hassan@gmail.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj6hsxq5S/kS', 'Ahmed Hassan', 'customer', true, NOW() - INTERVAL '6 months', NOW()),
('fatima.alaoui@gmail.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj6hsxq5S/kS', 'Fatima Alaoui', 'customer', true, NOW() - INTERVAL '4 months', NOW()),
('omar.benali@yahoo.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj6hsxq5S/kS', 'Omar Benali', 'customer', true, NOW() - INTERVAL '3 months', NOW()),
('aisha.mansouri@hotmail.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj6hsxq5S/kS', 'Aisha Mansouri', 'customer', true, NOW() - INTERVAL '2 months', NOW()),
('youssef.tazi@gmail.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj6hsxq5S/kS', 'Youssef Tazi', 'customer', true, NOW() - INTERVAL '1 month', NOW()),

-- Vendor Users
('vendor@daralkitab.ma', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj6hsxq5S/kS', 'Dar Al Kitab Manager', 'vendor', true, NOW() - INTERVAL '8 months', NOW()),
('contact@librairiealmaghrib.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj6hsxq5S/kS', 'Al Maghrib Books Manager', 'vendor', true, NOW() - INTERVAL '7 months', NOW()),
('admin@casablancabooks.ma', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj6hsxq5S/kS', 'Casablanca Books Admin', 'vendor', true, NOW() - INTERVAL '6 months', NOW()),
('info@rabatreads.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj6hsxq5S/kS', 'Rabat Reads Manager', 'vendor', true, NOW() - INTERVAL '5 months', NOW());

-- Insert Vendors
INSERT INTO vendors (user_id, business_name, business_type, description, address, phone, website, tax_id, bank_account, status, commission_rate, created_at, updated_at) VALUES
(7, 'Dar Al Kitab', 'bookstore', 'Leading Arabic and Islamic books publisher in Morocco', 'Avenue Mohammed V, Casablanca 20000, Morocco', '+212522123456', 'https://daralkitab.ma', 'TAX001MA', 'BANK001234567', 'approved', 8.5, NOW() - INTERVAL '8 months', NOW()),
(8, 'Librairie Al Maghrib', 'publisher', 'Premier French and Arabic literature publisher', 'Rue de la Liberté, Rabat 10000, Morocco', '+212537654321', 'https://librairiealmaghrib.com', 'TAX002MA', 'BANK002345678', 'approved', 7.0, NOW() - INTERVAL '7 months', NOW()),
(9, 'Casablanca Books Hub', 'distributor', 'Modern bookstore chain with international bestsellers', 'Boulevard Zerktouni, Casablanca 20100, Morocco', '+212522987654', 'https://casablancabooks.ma', 'TAX003MA', 'BANK003456789', 'approved', 9.0, NOW() - INTERVAL '6 months', NOW()),
(10, 'Rabat Academic Reads', 'academic', 'University bookstore specializing in academic texts', 'Campus Universitaire, Rabat 10100, Morocco', '+212537111222', 'https://rabatreads.com', 'TAX004MA', 'BANK004567890', 'approved', 6.5, NOW() - INTERVAL '5 months', NOW());

-- Insert Categories
INSERT INTO categories (name, description, parent_id, created_at, updated_at) VALUES
('Islamic Studies', 'Religious texts, Quran studies, Islamic jurisprudence', NULL, NOW(), NOW()),
('Arabic Literature', 'Classical and contemporary Arabic poetry and novels', NULL, NOW(), NOW()),
('French Literature', 'French novels, poetry, philosophy, and literary works', NULL, NOW(), NOW()),
('Moroccan Culture', 'Books about Moroccan history, traditions, and culture', NULL, NOW(), NOW()),
('Academic Textbooks', 'University-level textbooks across various disciplines', NULL, NOW(), NOW()),
('Business & Economics', 'Business management, economics, and entrepreneurship', NULL, NOW(), NOW()),
('Science & Technology', 'Computer science, engineering, and natural sciences', NULL, NOW(), NOW()),
('Children Books', 'Books for children and teenagers', NULL, NOW(), NOW()),

-- Subcategories
('Quran & Tafsir', 'Quranic studies and interpretations', 1, NOW(), NOW()),
('Hadith Collections', 'Prophetic traditions and sayings', 1, NOW(), NOW()),
('Engineering', 'Engineering textbooks and technical manuals', 5, NOW(), NOW()),
('Economics', 'Economic theory and applied economics', 5, NOW(), NOW());
