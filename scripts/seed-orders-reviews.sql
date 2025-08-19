-- Insert Product Reviews
INSERT INTO product_reviews (product_id, user_id, rating, review_text, created_at, updated_at) VALUES
(1, 2, 5, 'Excellent Tafsir with clear explanations. The Arabic text is beautifully printed and the commentary is very helpful for understanding.', NOW() - INTERVAL '2 months', NOW()),
(1, 3, 5, 'Outstanding quality and comprehensive coverage. Highly recommended for serious Islamic studies.', NOW() - INTERVAL '1 month', NOW()),
(2, 4, 5, 'Authentic Hadith collection with excellent organization. Essential for every Muslim library.', NOW() - INTERVAL '3 weeks', NOW()),
(3, 5, 4, 'Good introduction to Islamic jurisprudence. Well-structured and easy to follow for beginners.', NOW() - INTERVAL '2 weeks', NOW()),
(4, 6, 5, 'Beautiful Arabic translation that captures the essence of the original. Highly recommended.', NOW() - INTERVAL '1 week', NOW()),
(5, 2, 5, 'Classic poetry with excellent commentary. The historical context provided is very valuable.', NOW() - INTERVAL '5 days', NOW()),
(6, 3, 4, 'Great edition of Camus classic. The critical essays add significant value.', NOW() - INTERVAL '3 days', NOW()),
(7, 4, 5, 'Comprehensive history book with detailed analysis. Perfect for understanding modern Morocco.', NOW() - INTERVAL '4 weeks', NOW()),
(8, 5, 5, 'Amazing cookbook with authentic recipes. The photography is beautiful and instructions are clear.', NOW() - INTERVAL '2 weeks', NOW()),
(12, 6, 4, 'Inspiring stories of Moroccan entrepreneurs. Very motivating and practical advice.', NOW() - INTERVAL '1 week', NOW());

-- Insert Orders
INSERT INTO orders (user_id, order_number, status, total_amount, shipping_address, billing_address, payment_method, payment_status, notes, created_at, updated_at, shipped_at, delivered_at) VALUES
(2, 'ORD-2024-001001', 'delivered', 545.00, 'Rue Al Massira, Apt 12, Casablanca 20000, Morocco', 'Rue Al Massira, Apt 12, Casablanca 20000, Morocco', 'credit_card', 'paid', 'Customer requested express delivery', NOW() - INTERVAL '2 months', NOW() - INTERVAL '2 months' + INTERVAL '1 day', NOW() - INTERVAL '2 months' + INTERVAL '2 days', NOW() - INTERVAL '2 months' + INTERVAL '5 days'),

(3, 'ORD-2024-001002', 'delivered', 200.00, 'Avenue Mohammed V, Rabat 10000, Morocco', 'Avenue Mohammed V, Rabat 10000, Morocco', 'bank_transfer', 'paid', '', NOW() - INTERVAL '6 weeks', NOW() - INTERVAL '6 weeks' + INTERVAL '1 day', NOW() - INTERVAL '6 weeks' + INTERVAL '3 days', NOW() - INTERVAL '6 weeks' + INTERVAL '7 days'),

(4, 'ORD-2024-001003', 'shipped', 280.00, 'Boulevard Zerktouni, Casablanca 20100, Morocco', 'Boulevard Zerktouni, Casablanca 20100, Morocco', 'credit_card', 'paid', 'Gift wrapping requested', NOW() - INTERVAL '1 week', NOW() - INTERVAL '1 week' + INTERVAL '1 day', NOW() - INTERVAL '2 days', NULL),

(5, 'ORD-2024-001004', 'paid', 220.00, 'Medina, Marrakech 40000, Morocco', 'Medina, Marrakech 40000, Morocco', 'paypal', 'paid', '', NOW() - INTERVAL '3 days', NOW() - INTERVAL '3 days' + INTERVAL '2 hours', NULL, NULL),

(6, 'ORD-2024-001005', 'pending', 95.00, 'Boulevard Pasteur, Tangier 90000, Morocco', 'Boulevard Pasteur, Tangier 90000, Morocco', 'cash_on_delivery', 'pending', 'Customer prefers afternoon delivery', NOW() - INTERVAL '1 day', NOW() - INTERVAL '1 day', NULL, NULL);

-- Insert Order Items
INSERT INTO order_items (order_id, product_id, quantity, unit_price, total_price, created_at, updated_at) VALUES
-- Order 1 items (Ahmed Hassan)
(1, 1, 1, 450.00, 450.00, NOW() - INTERVAL '2 months', NOW() - INTERVAL '2 months'),
(1, 4, 1, 95.00, 95.00, NOW() - INTERVAL '2 months', NOW() - INTERVAL '2 months'),

-- Order 2 items (Fatima Alaoui)
(2, 3, 1, 125.00, 125.00, NOW() - INTERVAL '6 weeks', NOW() - INTERVAL '6 weeks'),
(2, 6, 1, 75.00, 75.00, NOW() - INTERVAL '6 weeks', NOW() - INTERVAL '6 weeks'),

-- Order 3 items (Omar Benali)
(3, 9, 1, 280.00, 280.00, NOW() - INTERVAL '1 week', NOW() - INTERVAL '1 week'),

-- Order 4 items (Aisha Mansouri)
(4, 10, 1, 220.00, 220.00, NOW() - INTERVAL '3 days', NOW() - INTERVAL '3 days'),

-- Order 5 items (Youssef Tazi)
(5, 12, 1, 95.00, 95.00, NOW() - INTERVAL '1 day', NOW() - INTERVAL '1 day');

-- Insert Vendor Analytics
INSERT INTO vendor_analytics (vendor_id, date, total_sales, total_orders, total_products, total_views, conversion_rate, created_at, updated_at) VALUES
-- Dar Al Kitab analytics
(1, CURRENT_DATE - INTERVAL '30 days', 2250.00, 15, 3, 450, 3.33, NOW() - INTERVAL '30 days', NOW() - INTERVAL '30 days'),
(1, CURRENT_DATE - INTERVAL '15 days', 1800.00, 12, 3, 380, 3.16, NOW() - INTERVAL '15 days', NOW() - INTERVAL '15 days'),
(1, CURRENT_DATE - INTERVAL '7 days', 950.00, 8, 3, 220, 3.64, NOW() - INTERVAL '7 days', NOW() - INTERVAL '7 days'),

-- Librairie Al Maghrib analytics
(2, CURRENT_DATE - INTERVAL '30 days', 1850.00, 22, 4, 520, 4.23, NOW() - INTERVAL '30 days', NOW() - INTERVAL '30 days'),
(2, CURRENT_DATE - INTERVAL '15 days', 1200.00, 16, 4, 350, 4.57, NOW() - INTERVAL '15 days', NOW() - INTERVAL '15 days'),
(2, CURRENT_DATE - INTERVAL '7 days', 680.00, 9, 4, 180, 5.00, NOW() - INTERVAL '7 days', NOW() - INTERVAL '7 days'),

-- Casablanca Books Hub analytics
(3, CURRENT_DATE - INTERVAL '30 days', 3200.00, 28, 5, 850, 3.29, NOW() - INTERVAL '30 days', NOW() - INTERVAL '30 days'),
(3, CURRENT_DATE - INTERVAL '15 days', 2100.00, 18, 5, 520, 3.46, NOW() - INTERVAL '15 days', NOW() - INTERVAL '15 days'),
(3, CURRENT_DATE - INTERVAL '7 days', 1250.00, 12, 5, 320, 3.75, NOW() - INTERVAL '7 days', NOW() - INTERVAL '7 days'),

-- Rabat Academic Reads analytics
(4, CURRENT_DATE - INTERVAL '30 days', 1950.00, 18, 3, 420, 4.29, NOW() - INTERVAL '30 days', NOW() - INTERVAL '30 days'),
(4, CURRENT_DATE - INTERVAL '15 days', 1350.00, 12, 3, 280, 4.29, NOW() - INTERVAL '15 days', NOW() - INTERVAL '15 days'),
(4, CURRENT_DATE - INTERVAL '7 days', 625.00, 5, 3, 150, 3.33, NOW() - INTERVAL '7 days', NOW() - INTERVAL '7 days');

-- Insert Vendor Payouts
INSERT INTO vendor_payouts (vendor_id, amount, period_start, period_end, status, transaction_id, created_at, updated_at, processed_at) VALUES
(1, 1912.50, '2024-01-01', '2024-01-31', 'completed', 'TXN-2024-001', NOW() - INTERVAL '3 weeks', NOW() - INTERVAL '3 weeks', NOW() - INTERVAL '2 weeks'),
(2, 1720.50, '2024-01-01', '2024-01-31', 'completed', 'TXN-2024-002', NOW() - INTERVAL '3 weeks', NOW() - INTERVAL '3 weeks', NOW() - INTERVAL '2 weeks'),
(3, 2912.00, '2024-01-01', '2024-01-31', 'completed', 'TXN-2024-003', NOW() - INTERVAL '3 weeks', NOW() - INTERVAL '3 weeks', NOW() - INTERVAL '2 weeks'),
(4, 1823.25, '2024-01-01', '2024-01-31', 'completed', 'TXN-2024-004', NOW() - INTERVAL '3 weeks', NOW() - INTERVAL '3 weeks', NOW() - INTERVAL '2 weeks'),

-- Current month pending payouts
(1, 856.25, '2024-02-01', '2024-02-29', 'pending', NULL, NOW() - INTERVAL '1 day', NOW() - INTERVAL '1 day', NULL),
(2, 632.40, '2024-02-01', '2024-02-29', 'pending', NULL, NOW() - INTERVAL '1 day', NOW() - INTERVAL '1 day', NULL),
(3, 1162.50, '2024-02-01', '2024-02-29', 'pending', NULL, NOW() - INTERVAL '1 day', NOW() - INTERVAL '1 day', NULL),
(4, 584.38, '2024-02-01', '2024-02-29', 'pending', NULL, NOW() - INTERVAL '1 day', NOW() - INTERVAL '1 day', NULL);
