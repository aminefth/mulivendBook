-- Insert Products (Books) - Part 2 of seed data
INSERT INTO products (vendor_id, name, description, price, stock_quantity, category_id, sku, isbn, author, publisher, publication_date, language, pages, weight, dimensions, tags, images, status, created_at, updated_at) VALUES

-- Islamic Studies Books
(1, 'تفسير القرآن العظيم - ابن كثير', 'Complete Tafsir of the Holy Quran by Ibn Kathir, one of the most respected classical commentaries. This comprehensive 8-volume set provides detailed explanations of Quranic verses with historical context and scholarly insights.', 450.00, 25, 9, 'ISL001', '9789953634012', 'ابن كثير', 'دار الكتاب العربي', '2020-01-15', 'Arabic', 3200, 4.5, '25x18x15 cm', 'tafsir,quran,ibn-kathir,islamic-studies', '["tafsir-ibn-kathir-1.jpg", "tafsir-ibn-kathir-2.jpg"]', 'active', NOW() - INTERVAL '6 months', NOW()),

(1, 'صحيح البخاري - كاملاً', 'The most authentic collection of Prophetic traditions (Hadith) compiled by Imam Al-Bukhari. This complete edition includes all 7563 hadiths with Arabic text and modern commentary.', 380.00, 18, 10, 'ISL002', '9789953634029', 'الإمام البخاري', 'دار الكتاب العربي', '2019-11-20', 'Arabic', 2800, 3.8, '24x17x12 cm', 'hadith,bukhari,sunnah,islamic-studies', '["sahih-bukhari-1.jpg", "sahih-bukhari-2.jpg"]', 'active', NOW() - INTERVAL '5 months', NOW()),

(4, 'Introduction to Islamic Jurisprudence', 'A comprehensive English guide to Islamic law (Fiqh) covering worship, transactions, family law, and contemporary issues. Perfect for students and researchers.', 125.00, 35, 1, 'ISL003', '9781234567890', 'Dr. Ahmad Al-Mansouri', 'Rabat Academic Press', '2021-03-10', 'English', 480, 0.8, '23x15x3 cm', 'fiqh,islamic-law,jurisprudence,academic', '["islamic-jurisprudence-1.jpg"]', 'active', NOW() - INTERVAL '4 months', NOW()),

-- Arabic Literature
(2, 'مئة عام من العزلة - ترجمة عربية', 'Arabic translation of Gabriel García Márquez masterpiece "One Hundred Years of Solitude". Expertly translated to capture the magical realism in Arabic literary tradition.', 95.00, 42, 2, 'LIT001', '9789953634036', 'غابرييل غارسيا ماركيز', 'مكتبة المغرب', '2020-06-15', 'Arabic', 420, 0.6, '21x14x3 cm', 'novel,translation,magical-realism,literature', '["hundred-years-solitude-ar.jpg"]', 'active', NOW() - INTERVAL '3 months', NOW()),

(2, 'ديوان المتنبي - شرح وتحقيق', 'Complete collection of Al-Mutanabbi poetry with modern commentary and analysis. Includes biographical information and historical context of the great Arab poet.', 85.00, 28, 2, 'LIT002', '9789953634043', 'أبو الطيب المتنبي', 'مكتبة المغرب', '2021-01-20', 'Arabic', 350, 0.5, '20x14x2.5 cm', 'poetry,classical,mutanabbi,arabic-literature', '["diwan-mutanabbi.jpg"]', 'active', NOW() - INTERVAL '2 months', NOW()),

-- French Literature
(2, 'L\'Étranger - Albert Camus', 'Classic French novel by Albert Camus exploring themes of existentialism and absurdism. This edition includes critical essays and study notes.', 75.00, 50, 3, 'LIT003', '9782070360024', 'Albert Camus', 'Librairie Al Maghrib', '2020-09-10', 'French', 180, 0.3, '18x11x1.5 cm', 'existentialism,camus,french-literature,philosophy', '["letranger-camus.jpg"]', 'active', NOW() - INTERVAL '4 months', NOW()),

-- Moroccan Culture
(3, 'تاريخ المغرب الحديث والمعاصر', 'Comprehensive history of modern and contemporary Morocco from the Alaouite dynasty to present day. Includes political, social, and economic developments.', 150.00, 22, 4, 'CUL001', '9789953634050', 'د. عبد الله العروي', 'كتب التراث المراكشي', '2020-12-05', 'Arabic', 650, 1.2, '24x16x4 cm', 'history,morocco,politics,culture', '["history-morocco-modern.jpg"]', 'active', NOW() - INTERVAL '5 months', NOW()),

(3, 'Moroccan Cuisine: Traditional Recipes', 'Authentic Moroccan recipes with step-by-step instructions, cultural context, and beautiful photography. Features tagines, couscous, pastries, and more.', 120.00, 45, 4, 'CUL002', '9789953634067', 'Lalla Fatima Hal', 'Marrakech Heritage Books', '2021-04-12', 'English', 280, 0.8, '26x20x2 cm', 'cuisine,recipes,moroccan-food,cooking', '["moroccan-cuisine-1.jpg", "moroccan-cuisine-2.jpg"]', 'active', NOW() - INTERVAL '2 months', NOW()),

-- Academic Textbooks
(4, 'Engineering Mathematics - 4th Edition', 'Comprehensive mathematics textbook for engineering students covering calculus, differential equations, linear algebra, and complex analysis.', 280.00, 15, 11, 'ENG001', '9781234567907', 'Dr. Hassan Benali', 'Rabat Academic Press', '2021-08-15', 'English', 850, 1.8, '25x18x5 cm', 'mathematics,engineering,calculus,textbook', '["engineering-math-4th.jpg"]', 'active', NOW() - INTERVAL '4 months', NOW()),

(4, 'Principles of Economics - Moroccan Context', 'Economics textbook adapted for Moroccan and North African economic conditions. Covers microeconomics, macroeconomics, and development economics.', 220.00, 30, 12, 'ECO001', '9789953634074', 'Prof. Aicha Belarbi', 'Atlas Academic Press', '2020-10-20', 'English', 720, 1.5, '24x17x4 cm', 'economics,morocco,development,textbook', '["principles-economics-morocco.jpg"]', 'active', NOW() - INTERVAL '6 months', NOW()),

-- Business & Economics
(3, 'Entrepreneurship in Morocco: Success Stories', 'Inspiring stories of successful Moroccan entrepreneurs with practical advice for starting and growing businesses in Morocco and MENA region.', 95.00, 55, 6, 'BUS001', '9789953634081', 'Youssef Alami', 'Casablanca Books Hub', '2021-05-30', 'English', 320, 0.6, '22x15x2 cm', 'entrepreneurship,business,morocco,success-stories', '["entrepreneurship-morocco.jpg"]', 'active', NOW() - INTERVAL '1 month', NOW()),

(3, 'Digital Marketing for MENA Markets', 'Complete guide to digital marketing strategies specifically designed for Middle East and North Africa markets, including cultural considerations.', 165.00, 32, 6, 'BUS002', '9789953634098', 'Sara El Fassi', 'Casablanca Books Hub', '2021-07-10', 'English', 450, 0.8, '23x15x3 cm', 'digital-marketing,mena,social-media,business', '["digital-marketing-mena.jpg"]', 'active', NOW() - INTERVAL '3 weeks', NOW()),

-- Science & Technology
(3, 'Artificial Intelligence: Theory and Practice', 'Comprehensive introduction to AI covering machine learning, neural networks, natural language processing, and practical applications.', 195.00, 28, 7, 'SCI001', '9789953634105', 'Dr. Omar Benjelloun', 'Casablanca Books Hub', '2021-09-15', 'English', 580, 1.1, '24x16x3.5 cm', 'artificial-intelligence,machine-learning,technology', '["ai-theory-practice.jpg"]', 'active', NOW() - INTERVAL '2 weeks', NOW()),

-- Children Books
(2, 'حكايات جدتي المغربية', 'Collection of traditional Moroccan folktales for children, beautifully illustrated with moral lessons and cultural values.', 45.00, 65, 8, 'CHI001', '9789953634112', 'أمينة الخطيب', 'نصوص طنجة الدولية', '2021-03-25', 'Arabic', 120, 0.4, '21x21x1 cm', 'children,folktales,moroccan-culture,illustrated', '["moroccan-folktales-1.jpg", "moroccan-folktales-2.jpg"]', 'active', NOW() - INTERVAL '6 weeks', NOW());
