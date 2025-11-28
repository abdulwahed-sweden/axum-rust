pub fn landing_page() -> String {
    r##"
<!DOCTYPE html>
<html lang="ar" dir="rtl" data-theme="dark">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ferox Theme System</title>

    <!-- Fonts -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Sans+Arabic:wght@400;500;600;700&family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">

    <!-- Tailwind CSS CDN -->
    <script src="https://cdn.tailwindcss.com"></script>

    <!-- Tailwind Config -->
    <script>
        tailwind.config = {
            darkMode: ['class', '[data-theme="dark"]'],
            theme: {
                extend: {
                    fontFamily: {
                        sans: ['IBM Plex Sans Arabic', 'Inter', 'system-ui', 'sans-serif'],
                        mono: ['JetBrains Mono', 'monospace'],
                        number: ['Inter', 'IBM Plex Sans Arabic', 'system-ui'],
                    },
                }
            }
        }
    </script>

    <!-- Translation System -->
    <script>
        const translations = {
            ar: {
                // Header
                stats: "الإحصائيات",
                tables: "الجداول",
                components: "المكونات",
                api: "API",

                // Hero
                heroTitle: "نظام Ferox Theme",
                heroSubtitle: "مرجع شامل للثيم الداكن والفاتح • مصمم للعمل الطويل وراحة العين",

                // Statistics
                statsTitle: "الإحصائيات",
                users: "المستخدمون",
                revenue: "الإيرادات",
                orders: "الطلبات",
                conversionRate: "معدل التحويل",

                // Table
                tableTitle: "جدول المعاملات",
                name: "الاسم",
                status: "الحالة",
                amount: "المبلغ",
                change: "التغيير",
                date: "التاريخ",
                completed: "مكتمل",
                pending: "معلق",
                cancelled: "ملغي",
                processing: "قيد المعالجة",

                // Names
                name1: "أحمد محمد",
                name2: "سارة علي",
                name3: "خالد عبدالله",
                name4: "نورة سعيد",
                name5: "محمد إبراهيم",

                // Dates
                date1: "28 نوفمبر 2025",
                date2: "27 نوفمبر 2025",
                date3: "26 نوفمبر 2025",
                date4: "25 نوفمبر 2025",
                date5: "24 نوفمبر 2025",

                // Components
                componentsTitle: "المكونات",
                buttons: "الأزرار",
                badges: "الشارات",
                alerts: "التنبيهات",
                inputs: "حقول الإدخال",
                email: "البريد الإلكتروني",
                amountLabel: "المبلغ",

                // Alerts
                alertSuccess: "تمت العملية بنجاح!",
                alertWarning: "تحذير: يرجى التحقق من البيانات.",
                alertDanger: "حدث خطأ أثناء المعالجة.",
                alertInfo: "معلومة: هذه رسالة إعلامية.",

                // API
                apiTitle: "API Endpoints",
                method: "Method",
                endpoint: "Endpoint",
                description: "الوصف",
                listUsers: "قائمة المستخدمين",
                createUser: "إنشاء مستخدم جديد",
                listProducts: "قائمة المنتجات",
                listOrders: "قائمة الطلبات",
                deleteUser: "حذف مستخدم",

                // Colors
                colorsTitle: "ملخص الألوان",

                // Footer
                footerText: "Ferox Theme System • مرجع شامل للمشاريع المستقبلية",
            },
            en: {
                // Header
                stats: "Statistics",
                tables: "Tables",
                components: "Components",
                api: "API",

                // Hero
                heroTitle: "Ferox Theme System",
                heroSubtitle: "A comprehensive reference for dark and light themes • Designed for long work sessions and eye comfort",

                // Statistics
                statsTitle: "Statistics",
                users: "Users",
                revenue: "Revenue",
                orders: "Orders",
                conversionRate: "Conversion Rate",

                // Table
                tableTitle: "Transactions Table",
                name: "Name",
                status: "Status",
                amount: "Amount",
                change: "Change",
                date: "Date",
                completed: "Completed",
                pending: "Pending",
                cancelled: "Cancelled",
                processing: "Processing",

                // Names
                name1: "Ahmed Mohammed",
                name2: "Sarah Ali",
                name3: "Khalid Abdullah",
                name4: "Noura Saeed",
                name5: "Mohammed Ibrahim",

                // Dates
                date1: "Nov 28, 2025",
                date2: "Nov 27, 2025",
                date3: "Nov 26, 2025",
                date4: "Nov 25, 2025",
                date5: "Nov 24, 2025",

                // Components
                componentsTitle: "Components",
                buttons: "Buttons",
                badges: "Badges",
                alerts: "Alerts",
                inputs: "Input Fields",
                email: "Email Address",
                amountLabel: "Amount",

                // Alerts
                alertSuccess: "Operation completed successfully!",
                alertWarning: "Warning: Please verify your data.",
                alertDanger: "An error occurred during processing.",
                alertInfo: "Info: This is an informational message.",

                // API
                apiTitle: "API Endpoints",
                method: "Method",
                endpoint: "Endpoint",
                description: "Description",
                listUsers: "List all users",
                createUser: "Create a new user",
                listProducts: "List all products",
                listOrders: "List all orders",
                deleteUser: "Delete a user",

                // Colors
                colorsTitle: "Color Palette",

                // Footer
                footerText: "Ferox Theme System • A comprehensive reference for future projects",
            }
        };
    </script>

    <!-- CSS Variables -->
    <style>
        /* ========== Dark Theme (Default) ========== */
        [data-theme="dark"] {
            --bg-base: #12161F;
            --bg-surface: #1A1F2B;
            --bg-elevated: #242A38;
            --bg-sidebar: #161A24;
            --bg-hover: #2A3142;

            --text-primary: #F0F2F5;
            --text-secondary: #A0A8B8;
            --text-tertiary: #6B7280;
            --text-number: #FFFFFF;
            --text-number-secondary: #E5E7EB;
            --text-data: #D1D5DB;

            --border-subtle: #2A3142;
            --border-default: #374151;
            --border-strong: #4B5563;

            --primary: #3B82F6;
            --primary-hover: #2563EB;
            --primary-soft: rgba(59, 130, 246, 0.15);
            --primary-text: #60A5FA;

            --success: #10B981;
            --success-soft: rgba(16, 185, 129, 0.15);
            --success-text: #34D399;
            --success-border: rgba(16, 185, 129, 0.3);

            --warning: #F59E0B;
            --warning-soft: rgba(245, 158, 11, 0.15);
            --warning-text: #FBBF24;
            --warning-border: rgba(245, 158, 11, 0.3);

            --danger: #EF4444;
            --danger-soft: rgba(239, 68, 68, 0.15);
            --danger-text: #F87171;
            --danger-border: rgba(239, 68, 68, 0.3);

            --info: #06B6D4;
            --info-soft: rgba(6, 182, 212, 0.15);
            --info-text: #22D3EE;
            --info-border: rgba(6, 182, 212, 0.3);

            --purple: #8B5CF6;
            --purple-soft: rgba(139, 92, 246, 0.15);
            --purple-text: #A78BFA;
            --purple-border: rgba(139, 92, 246, 0.3);

            --data-positive: #34D399;
            --data-negative: #F87171;
            --data-highlight: #FBBF24;

            --shadow-card: 0 2px 8px rgba(0, 0, 0, 0.25);
            --shadow-md: 0 4px 12px rgba(0, 0, 0, 0.4);
        }

        /* ========== Light Theme ========== */
        [data-theme="light"] {
            --bg-base: #F4F5F7;
            --bg-surface: #FFFFFF;
            --bg-elevated: #F9FAFB;
            --bg-sidebar: #EBEDF0;
            --bg-hover: #F0F1F3;

            --text-primary: #111827;
            --text-secondary: #4B5563;
            --text-tertiary: #6B7280;
            --text-number: #030712;
            --text-number-secondary: #1F2937;
            --text-data: #374151;

            --border-subtle: #E5E7EB;
            --border-default: #D1D5DB;
            --border-strong: #9CA3AF;

            --primary: #2563EB;
            --primary-hover: #1D4ED8;
            --primary-soft: rgba(37, 99, 235, 0.1);
            --primary-text: #1D4ED8;

            --success: #059669;
            --success-soft: rgba(5, 150, 105, 0.1);
            --success-text: #047857;
            --success-border: rgba(5, 150, 105, 0.25);

            --warning: #D97706;
            --warning-soft: rgba(217, 119, 6, 0.1);
            --warning-text: #B45309;
            --warning-border: rgba(217, 119, 6, 0.25);

            --danger: #DC2626;
            --danger-soft: rgba(220, 38, 38, 0.1);
            --danger-text: #B91C1C;
            --danger-border: rgba(220, 38, 38, 0.25);

            --info: #0891B2;
            --info-soft: rgba(8, 145, 178, 0.1);
            --info-text: #0E7490;
            --info-border: rgba(8, 145, 178, 0.25);

            --purple: #7C3AED;
            --purple-soft: rgba(124, 58, 237, 0.1);
            --purple-text: #6D28D9;
            --purple-border: rgba(124, 58, 237, 0.25);

            --data-positive: #059669;
            --data-negative: #DC2626;
            --data-highlight: #D97706;

            --shadow-card: 0 1px 4px rgba(0, 0, 0, 0.06);
            --shadow-md: 0 4px 12px rgba(0, 0, 0, 0.08);
        }

        /* Base Styles - Increased font size */
        html { font-size: 16px; }

        body {
            font-family: 'IBM Plex Sans Arabic', 'Inter', system-ui, sans-serif;
            background-color: var(--bg-base);
            color: var(--text-primary);
            line-height: 1.6;
            font-size: 1rem;
            transition: background-color 0.2s ease, color 0.2s ease;
        }

        /* Increased text sizes for better readability */
        h1 { font-size: 2.5rem; }
        h2 { font-size: 1.375rem; }
        h3 { font-size: 1.125rem; }

        p, span, td, th, label, a {
            font-size: 0.9375rem;
        }

        .text-sm {
            font-size: 0.875rem !important;
        }

        .text-xs {
            font-size: 0.8125rem !important;
        }

        /* Scrollbar */
        ::-webkit-scrollbar { width: 8px; height: 8px; }
        ::-webkit-scrollbar-track { background: var(--bg-base); }
        ::-webkit-scrollbar-thumb { background: var(--border-strong); border-radius: 9999px; }
        ::-webkit-scrollbar-thumb:hover { background: var(--text-tertiary); }

        /* Selection */
        ::selection { background: var(--primary-soft); color: var(--text-primary); }

        /* Numbers - Keep original size */
        .number-display {
            font-family: 'Inter', 'IBM Plex Sans Arabic', system-ui, sans-serif;
            font-weight: 700;
            font-variant-numeric: tabular-nums;
            letter-spacing: -0.02em;
            direction: ltr;
        }

        .data-cell {
            font-family: 'JetBrains Mono', monospace;
            font-variant-numeric: tabular-nums;
            direction: ltr;
            text-align: left;
        }

        /* Transitions */
        .theme-transition {
            transition: background-color 0.2s ease, border-color 0.2s ease, color 0.2s ease, box-shadow 0.2s ease;
        }

        /* Nav Links */
        .nav-link {
            color: var(--text-secondary);
            transition: color 0.2s ease;
        }
        .nav-link:hover {
            color: var(--text-primary);
        }

        /* Cards */
        .stat-card {
            background: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            box-shadow: var(--shadow-card);
            transition: all 0.2s ease;
        }
        .stat-card:hover {
            border-color: var(--border-default);
            box-shadow: var(--shadow-md);
            transform: translateY(-2px);
        }

        /* Table Rows */
        .table-row {
            transition: background-color 0.2s ease;
        }
        .table-row:hover {
            background: var(--bg-hover);
        }

        /* Buttons */
        .btn-primary {
            background: var(--primary);
            transition: background-color 0.2s ease;
        }
        .btn-primary:hover {
            background: var(--primary-hover);
        }
        .btn-success {
            background: var(--success);
            transition: background-color 0.2s ease;
        }
        .btn-success:hover {
            background: #047857;
        }
        .btn-danger {
            background: var(--danger);
            transition: background-color 0.2s ease;
        }
        .btn-danger:hover {
            background: #B91C1C;
        }

        /* Toggle Buttons */
        .toggle-btn {
            background: var(--bg-elevated);
            border: 1px solid var(--border-subtle);
            color: var(--text-secondary);
            transition: all 0.2s ease;
        }
        .toggle-btn:hover {
            background: var(--bg-hover);
            border-color: var(--border-default);
        }

        /* Inputs */
        .input-field {
            background: var(--bg-base);
            border: 1px solid var(--border-default);
            color: var(--text-primary);
            transition: all 0.2s ease;
        }
        .input-field:focus {
            border-color: var(--primary);
            box-shadow: 0 0 0 3px var(--primary-soft);
            outline: none;
        }

        /* LTR Support */
        [dir="ltr"] {
            text-align: left;
        }

        [dir="ltr"] th,
        [dir="ltr"] td {
            text-align: left;
        }

        [dir="ltr"] .text-right {
            text-align: left;
        }

        /* Table alignment classes */
        .table-cell {
            text-align: inherit;
        }

        [dir="rtl"] .table-cell {
            text-align: right;
        }

        [dir="ltr"] .table-cell {
            text-align: left;
        }
    </style>
</head>
<body class="min-h-screen">

    <!-- ===== Header ===== -->
    <header class="theme-transition" style="background: var(--bg-surface); border-bottom: 1px solid var(--border-subtle);">
        <div class="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
            <!-- Logo -->
            <div class="flex items-center gap-3">
                <div class="w-10 h-10 rounded-lg flex items-center justify-center text-white font-bold text-xl" style="background: var(--primary);">
                    F
                </div>
                <span class="font-bold text-lg" style="color: var(--text-primary);">Ferox Theme</span>
            </div>

            <!-- Navigation -->
            <nav class="flex items-center gap-6">
                <a href="#stats" class="nav-link text-sm font-medium" data-i18n="stats">الإحصائيات</a>
                <a href="#table" class="nav-link text-sm font-medium" data-i18n="tables">الجداول</a>
                <a href="#components" class="nav-link text-sm font-medium" data-i18n="components">المكونات</a>
                <a href="#api" class="nav-link text-sm font-medium" data-i18n="api">API</a>
            </nav>

            <!-- Toggle Buttons -->
            <div class="flex items-center gap-2">
                <!-- Language Toggle -->
                <button id="lang-toggle" class="toggle-btn px-3 py-2 rounded-lg text-sm font-semibold" onclick="toggleLanguage()">
                    <span id="lang-text">EN</span>
                </button>

                <!-- Theme Toggle -->
                <button id="theme-toggle" class="toggle-btn w-10 h-10 rounded-lg flex items-center justify-center" onclick="toggleTheme()">
                    <svg id="sun-icon" class="w-5 h-5 hidden" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <circle cx="12" cy="12" r="5"></circle>
                        <line x1="12" y1="1" x2="12" y2="3"></line>
                        <line x1="12" y1="21" x2="12" y2="23"></line>
                        <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                        <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                        <line x1="1" y1="12" x2="3" y2="12"></line>
                        <line x1="21" y1="12" x2="23" y2="12"></line>
                        <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                        <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
                    </svg>
                    <svg id="moon-icon" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                    </svg>
                </button>
            </div>
        </div>
    </header>

    <main class="max-w-7xl mx-auto px-6 py-8">

        <!-- ===== Hero Section ===== -->
        <section class="text-center py-12 mb-12">
            <h1 class="font-bold mb-4" style="color: var(--text-primary);" data-i18n="heroTitle">
                نظام Ferox Theme
            </h1>
            <p class="text-lg mb-6" style="color: var(--text-secondary);" data-i18n="heroSubtitle">
                مرجع شامل للثيم الداكن والفاتح • مصمم للعمل الطويل وراحة العين
            </p>
            <div class="flex items-center justify-center gap-4">
                <span class="px-3 py-1 rounded-full text-xs font-medium"
                      style="background: var(--primary-soft); color: var(--primary-text); border: 1px solid var(--primary);">
                    Rust + Axum
                </span>
                <span class="px-3 py-1 rounded-full text-xs font-medium"
                      style="background: var(--success-soft); color: var(--success-text); border: 1px solid var(--success-border);">
                    Tailwind CSS
                </span>
                <span class="px-3 py-1 rounded-full text-xs font-medium"
                      style="background: var(--purple-soft); color: var(--purple-text); border: 1px solid var(--purple-border);">
                    Dark/Light Mode
                </span>
            </div>
        </section>

        <!-- ===== Statistics Cards ===== -->
        <section id="stats" class="mb-12">
            <h2 class="font-bold mb-6" style="color: var(--text-primary);" data-i18n="statsTitle">الإحصائيات</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">

                <!-- Card 1 -->
                <div class="stat-card rounded-xl p-5 cursor-pointer">
                    <div class="flex justify-between items-start">
                        <div>
                            <p class="text-sm mb-2" style="color: var(--text-secondary);" data-i18n="users">المستخدمون</p>
                            <h3 class="number-display text-3xl mb-2" style="color: var(--text-number);">15,847</h3>
                            <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium"
                                  style="background: var(--success-soft); color: var(--success-text); border: 1px solid var(--success-border);">
                                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <polyline points="23 6 13.5 15.5 8.5 10.5 1 18"></polyline>
                                </svg>
                                +12.5%
                            </span>
                        </div>
                        <div class="w-12 h-12 rounded-lg flex items-center justify-center"
                             style="background: var(--primary-soft); color: var(--primary-text); border: 1px solid var(--primary);">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
                                <circle cx="9" cy="7" r="4"></circle>
                            </svg>
                        </div>
                    </div>
                </div>

                <!-- Card 2 -->
                <div class="stat-card rounded-xl p-5 cursor-pointer">
                    <div class="flex justify-between items-start">
                        <div>
                            <p class="text-sm mb-2" style="color: var(--text-secondary);" data-i18n="revenue">الإيرادات</p>
                            <h3 class="number-display text-3xl mb-2" style="color: var(--text-number);">$284,590</h3>
                            <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium"
                                  style="background: var(--success-soft); color: var(--success-text); border: 1px solid var(--success-border);">
                                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <polyline points="23 6 13.5 15.5 8.5 10.5 1 18"></polyline>
                                </svg>
                                +8.2%
                            </span>
                        </div>
                        <div class="w-12 h-12 rounded-lg flex items-center justify-center"
                             style="background: var(--success-soft); color: var(--success-text); border: 1px solid var(--success-border);">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <line x1="12" y1="1" x2="12" y2="23"></line>
                                <path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"></path>
                            </svg>
                        </div>
                    </div>
                </div>

                <!-- Card 3 -->
                <div class="stat-card rounded-xl p-5 cursor-pointer">
                    <div class="flex justify-between items-start">
                        <div>
                            <p class="text-sm mb-2" style="color: var(--text-secondary);" data-i18n="orders">الطلبات</p>
                            <h3 class="number-display text-3xl mb-2" style="color: var(--text-number);">1,284</h3>
                            <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium"
                                  style="background: var(--danger-soft); color: var(--danger-text); border: 1px solid var(--danger-border);">
                                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <polyline points="23 18 13.5 8.5 8.5 13.5 1 6"></polyline>
                                </svg>
                                -3.1%
                            </span>
                        </div>
                        <div class="w-12 h-12 rounded-lg flex items-center justify-center"
                             style="background: var(--danger-soft); color: var(--danger-text); border: 1px solid var(--danger-border);">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <circle cx="9" cy="21" r="1"></circle>
                                <circle cx="20" cy="21" r="1"></circle>
                                <path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"></path>
                            </svg>
                        </div>
                    </div>
                </div>

                <!-- Card 4 -->
                <div class="stat-card rounded-xl p-5 cursor-pointer">
                    <div class="flex justify-between items-start">
                        <div>
                            <p class="text-sm mb-2" style="color: var(--text-secondary);" data-i18n="conversionRate">معدل التحويل</p>
                            <h3 class="number-display text-3xl mb-2" style="color: var(--text-number);">24.8%</h3>
                            <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium"
                                  style="background: var(--success-soft); color: var(--success-text); border: 1px solid var(--success-border);">
                                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <polyline points="23 6 13.5 15.5 8.5 10.5 1 18"></polyline>
                                </svg>
                                +5.4%
                            </span>
                        </div>
                        <div class="w-12 h-12 rounded-lg flex items-center justify-center"
                             style="background: var(--purple-soft); color: var(--purple-text); border: 1px solid var(--purple-border);">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline>
                            </svg>
                        </div>
                    </div>
                </div>

            </div>
        </section>

        <!-- ===== Table ===== -->
        <section id="table" class="mb-12">
            <h2 class="font-bold mb-6" style="color: var(--text-primary);" data-i18n="tableTitle">جدول المعاملات</h2>
            <div class="rounded-xl overflow-hidden theme-transition"
                 style="background: var(--bg-surface); border: 1px solid var(--border-subtle); box-shadow: var(--shadow-card);">
                <table class="w-full">
                    <thead>
                        <tr style="background: var(--bg-elevated);">
                            <th class="table-cell px-4 py-3 text-xs font-semibold uppercase tracking-wider" style="color: var(--text-tertiary);" data-i18n="name">الاسم</th>
                            <th class="table-cell px-4 py-3 text-xs font-semibold uppercase tracking-wider" style="color: var(--text-tertiary);" data-i18n="status">الحالة</th>
                            <th class="table-cell px-4 py-3 text-xs font-semibold uppercase tracking-wider" style="color: var(--text-tertiary);" data-i18n="amount">المبلغ</th>
                            <th class="table-cell px-4 py-3 text-xs font-semibold uppercase tracking-wider" style="color: var(--text-tertiary);" data-i18n="change">التغيير</th>
                            <th class="table-cell px-4 py-3 text-xs font-semibold uppercase tracking-wider" style="color: var(--text-tertiary);" data-i18n="date">التاريخ</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr class="table-row" style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="table-cell px-4 py-3" style="color: var(--text-primary);" data-i18n="name1">أحمد محمد</td>
                            <td class="table-cell px-4 py-3">
                                <span class="px-2 py-0.5 rounded-full text-xs font-medium"
                                      style="background: var(--success-soft); color: var(--success-text); border: 1px solid var(--success-border);" data-i18n="completed">
                                    مكتمل
                                </span>
                            </td>
                            <td class="table-cell px-4 py-3 data-cell font-medium" style="color: var(--text-number-secondary);">$2,450.00</td>
                            <td class="table-cell px-4 py-3 data-cell font-medium" style="color: var(--data-positive);">+12.5%</td>
                            <td class="table-cell px-4 py-3 text-sm" style="color: var(--text-tertiary);" data-i18n="date1">28 نوفمبر 2025</td>
                        </tr>
                        <tr class="table-row" style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="table-cell px-4 py-3" style="color: var(--text-primary);" data-i18n="name2">سارة علي</td>
                            <td class="table-cell px-4 py-3">
                                <span class="px-2 py-0.5 rounded-full text-xs font-medium"
                                      style="background: var(--warning-soft); color: var(--warning-text); border: 1px solid var(--warning-border);" data-i18n="pending">
                                    معلق
                                </span>
                            </td>
                            <td class="table-cell px-4 py-3 data-cell font-medium" style="color: var(--text-number-secondary);">$1,890.00</td>
                            <td class="table-cell px-4 py-3 data-cell font-medium" style="color: var(--data-negative);">-3.2%</td>
                            <td class="table-cell px-4 py-3 text-sm" style="color: var(--text-tertiary);" data-i18n="date2">27 نوفمبر 2025</td>
                        </tr>
                        <tr class="table-row" style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="table-cell px-4 py-3" style="color: var(--text-primary);" data-i18n="name3">خالد عبدالله</td>
                            <td class="table-cell px-4 py-3">
                                <span class="px-2 py-0.5 rounded-full text-xs font-medium"
                                      style="background: var(--success-soft); color: var(--success-text); border: 1px solid var(--success-border);" data-i18n="completed">
                                    مكتمل
                                </span>
                            </td>
                            <td class="table-cell px-4 py-3 data-cell font-medium" style="color: var(--text-number-secondary);">$3,200.00</td>
                            <td class="table-cell px-4 py-3 data-cell font-medium" style="color: var(--data-positive);">+8.7%</td>
                            <td class="table-cell px-4 py-3 text-sm" style="color: var(--text-tertiary);" data-i18n="date3">26 نوفمبر 2025</td>
                        </tr>
                        <tr class="table-row" style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="table-cell px-4 py-3" style="color: var(--text-primary);" data-i18n="name4">نورة سعيد</td>
                            <td class="table-cell px-4 py-3">
                                <span class="px-2 py-0.5 rounded-full text-xs font-medium"
                                      style="background: var(--danger-soft); color: var(--danger-text); border: 1px solid var(--danger-border);" data-i18n="cancelled">
                                    ملغي
                                </span>
                            </td>
                            <td class="table-cell px-4 py-3 data-cell font-medium" style="color: var(--text-number-secondary);">$950.00</td>
                            <td class="table-cell px-4 py-3 data-cell font-medium" style="color: var(--data-negative);">-15.3%</td>
                            <td class="table-cell px-4 py-3 text-sm" style="color: var(--text-tertiary);" data-i18n="date4">25 نوفمبر 2025</td>
                        </tr>
                        <tr class="table-row">
                            <td class="table-cell px-4 py-3" style="color: var(--text-primary);" data-i18n="name5">محمد إبراهيم</td>
                            <td class="table-cell px-4 py-3">
                                <span class="px-2 py-0.5 rounded-full text-xs font-medium"
                                      style="background: var(--info-soft); color: var(--info-text); border: 1px solid var(--info-border);" data-i18n="processing">
                                    قيد المعالجة
                                </span>
                            </td>
                            <td class="table-cell px-4 py-3 data-cell font-medium" style="color: var(--text-number-secondary);">$4,100.00</td>
                            <td class="table-cell px-4 py-3 data-cell font-medium" style="color: var(--data-positive);">+22.1%</td>
                            <td class="table-cell px-4 py-3 text-sm" style="color: var(--text-tertiary);" data-i18n="date5">24 نوفمبر 2025</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </section>

        <!-- ===== Components ===== -->
        <section id="components" class="mb-12">
            <h2 class="font-bold mb-6" style="color: var(--text-primary);" data-i18n="componentsTitle">المكونات</h2>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">

                <!-- Buttons -->
                <div class="rounded-xl p-6 theme-transition"
                     style="background: var(--bg-surface); border: 1px solid var(--border-subtle);">
                    <h3 class="font-semibold mb-4" style="color: var(--text-primary);" data-i18n="buttons">الأزرار</h3>
                    <div class="flex flex-wrap gap-3">
                        <button class="btn-primary px-4 py-2 rounded-lg text-sm font-medium text-white">
                            Primary
                        </button>
                        <button class="btn-success px-4 py-2 rounded-lg text-sm font-medium text-white">
                            Success
                        </button>
                        <button class="btn-danger px-4 py-2 rounded-lg text-sm font-medium text-white">
                            Danger
                        </button>
                        <button class="px-4 py-2 rounded-lg text-sm font-medium text-white theme-transition"
                                style="background: var(--warning);">
                            Warning
                        </button>
                        <button class="px-4 py-2 rounded-lg text-sm font-medium theme-transition"
                                style="background: var(--bg-elevated); border: 1px solid var(--border-default); color: var(--text-primary);">
                            Secondary
                        </button>
                    </div>
                </div>

                <!-- Badges -->
                <div class="rounded-xl p-6 theme-transition"
                     style="background: var(--bg-surface); border: 1px solid var(--border-subtle);">
                    <h3 class="font-semibold mb-4" style="color: var(--text-primary);" data-i18n="badges">الشارات</h3>
                    <div class="flex flex-wrap gap-3">
                        <span class="px-3 py-1 rounded-full text-xs font-medium"
                              style="background: var(--primary-soft); color: var(--primary-text); border: 1px solid var(--primary);">
                            Primary
                        </span>
                        <span class="px-3 py-1 rounded-full text-xs font-medium"
                              style="background: var(--success-soft); color: var(--success-text); border: 1px solid var(--success-border);">
                            Success
                        </span>
                        <span class="px-3 py-1 rounded-full text-xs font-medium"
                              style="background: var(--warning-soft); color: var(--warning-text); border: 1px solid var(--warning-border);">
                            Warning
                        </span>
                        <span class="px-3 py-1 rounded-full text-xs font-medium"
                              style="background: var(--danger-soft); color: var(--danger-text); border: 1px solid var(--danger-border);">
                            Danger
                        </span>
                        <span class="px-3 py-1 rounded-full text-xs font-medium"
                              style="background: var(--info-soft); color: var(--info-text); border: 1px solid var(--info-border);">
                            Info
                        </span>
                        <span class="px-3 py-1 rounded-full text-xs font-medium"
                              style="background: var(--purple-soft); color: var(--purple-text); border: 1px solid var(--purple-border);">
                            Purple
                        </span>
                    </div>
                </div>

                <!-- Alerts -->
                <div class="rounded-xl p-6 theme-transition"
                     style="background: var(--bg-surface); border: 1px solid var(--border-subtle);">
                    <h3 class="font-semibold mb-4" style="color: var(--text-primary);" data-i18n="alerts">التنبيهات</h3>
                    <div class="space-y-3">
                        <div class="p-3 rounded-lg text-sm"
                             style="background: var(--success-soft); color: var(--success-text); border: 1px solid var(--success-border);" data-i18n="alertSuccess">
                            تمت العملية بنجاح!
                        </div>
                        <div class="p-3 rounded-lg text-sm"
                             style="background: var(--warning-soft); color: var(--warning-text); border: 1px solid var(--warning-border);" data-i18n="alertWarning">
                            تحذير: يرجى التحقق من البيانات.
                        </div>
                        <div class="p-3 rounded-lg text-sm"
                             style="background: var(--danger-soft); color: var(--danger-text); border: 1px solid var(--danger-border);" data-i18n="alertDanger">
                            حدث خطأ أثناء المعالجة.
                        </div>
                        <div class="p-3 rounded-lg text-sm"
                             style="background: var(--info-soft); color: var(--info-text); border: 1px solid var(--info-border);" data-i18n="alertInfo">
                            معلومة: هذه رسالة إعلامية.
                        </div>
                    </div>
                </div>

                <!-- Input Fields -->
                <div class="rounded-xl p-6 theme-transition"
                     style="background: var(--bg-surface); border: 1px solid var(--border-subtle);">
                    <h3 class="font-semibold mb-4" style="color: var(--text-primary);" data-i18n="inputs">حقول الإدخال</h3>
                    <div class="space-y-4">
                        <div>
                            <label class="block text-sm font-medium mb-1" style="color: var(--text-secondary);" data-i18n="email">البريد الإلكتروني</label>
                            <input type="email" placeholder="example@email.com"
                                   class="input-field w-full px-3 py-2 rounded-lg text-sm">
                        </div>
                        <div>
                            <label class="block text-sm font-medium mb-1" style="color: var(--text-secondary);" data-i18n="amountLabel">المبلغ</label>
                            <input type="text" placeholder="0.00" value="1,234.56"
                                   class="input-field w-full px-3 py-2 rounded-lg text-sm data-cell">
                        </div>
                    </div>
                </div>

            </div>
        </section>

        <!-- ===== API Reference ===== -->
        <section id="api" class="mb-12">
            <h2 class="font-bold mb-6" style="color: var(--text-primary);" data-i18n="apiTitle">API Endpoints</h2>
            <div class="rounded-xl overflow-hidden theme-transition"
                 style="background: var(--bg-surface); border: 1px solid var(--border-subtle);">
                <table class="w-full">
                    <thead>
                        <tr style="background: var(--bg-elevated);">
                            <th class="table-cell px-4 py-3 text-xs font-semibold uppercase" style="color: var(--text-tertiary);" data-i18n="method">Method</th>
                            <th class="table-cell px-4 py-3 text-xs font-semibold uppercase" style="color: var(--text-tertiary);" data-i18n="endpoint">Endpoint</th>
                            <th class="table-cell px-4 py-3 text-xs font-semibold uppercase" style="color: var(--text-tertiary);" data-i18n="description">الوصف</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="table-cell px-4 py-3">
                                <span class="px-2 py-0.5 rounded text-xs font-mono font-medium"
                                      style="background: var(--success-soft); color: var(--success-text);">GET</span>
                            </td>
                            <td class="table-cell px-4 py-3 font-mono text-sm" style="color: var(--text-primary);">/users</td>
                            <td class="table-cell px-4 py-3 text-sm" style="color: var(--text-secondary);" data-i18n="listUsers">قائمة المستخدمين</td>
                        </tr>
                        <tr style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="table-cell px-4 py-3">
                                <span class="px-2 py-0.5 rounded text-xs font-mono font-medium"
                                      style="background: var(--primary-soft); color: var(--primary-text);">POST</span>
                            </td>
                            <td class="table-cell px-4 py-3 font-mono text-sm" style="color: var(--text-primary);">/users</td>
                            <td class="table-cell px-4 py-3 text-sm" style="color: var(--text-secondary);" data-i18n="createUser">إنشاء مستخدم جديد</td>
                        </tr>
                        <tr style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="table-cell px-4 py-3">
                                <span class="px-2 py-0.5 rounded text-xs font-mono font-medium"
                                      style="background: var(--success-soft); color: var(--success-text);">GET</span>
                            </td>
                            <td class="table-cell px-4 py-3 font-mono text-sm" style="color: var(--text-primary);">/products</td>
                            <td class="table-cell px-4 py-3 text-sm" style="color: var(--text-secondary);" data-i18n="listProducts">قائمة المنتجات</td>
                        </tr>
                        <tr style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="table-cell px-4 py-3">
                                <span class="px-2 py-0.5 rounded text-xs font-mono font-medium"
                                      style="background: var(--success-soft); color: var(--success-text);">GET</span>
                            </td>
                            <td class="table-cell px-4 py-3 font-mono text-sm" style="color: var(--text-primary);">/orders</td>
                            <td class="table-cell px-4 py-3 text-sm" style="color: var(--text-secondary);" data-i18n="listOrders">قائمة الطلبات</td>
                        </tr>
                        <tr>
                            <td class="table-cell px-4 py-3">
                                <span class="px-2 py-0.5 rounded text-xs font-mono font-medium"
                                      style="background: var(--danger-soft); color: var(--danger-text);">DELETE</span>
                            </td>
                            <td class="table-cell px-4 py-3 font-mono text-sm" style="color: var(--text-primary);">/users/{id}</td>
                            <td class="table-cell px-4 py-3 text-sm" style="color: var(--text-secondary);" data-i18n="deleteUser">حذف مستخدم</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </section>

        <!-- ===== Color Palette ===== -->
        <section class="mb-12">
            <h2 class="font-bold mb-6" style="color: var(--text-primary);" data-i18n="colorsTitle">ملخص الألوان</h2>
            <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
                <div class="text-center">
                    <div class="w-full h-16 rounded-lg mb-2" style="background: var(--primary);"></div>
                    <span class="text-xs" style="color: var(--text-secondary);">Primary</span>
                </div>
                <div class="text-center">
                    <div class="w-full h-16 rounded-lg mb-2" style="background: var(--success);"></div>
                    <span class="text-xs" style="color: var(--text-secondary);">Success</span>
                </div>
                <div class="text-center">
                    <div class="w-full h-16 rounded-lg mb-2" style="background: var(--warning);"></div>
                    <span class="text-xs" style="color: var(--text-secondary);">Warning</span>
                </div>
                <div class="text-center">
                    <div class="w-full h-16 rounded-lg mb-2" style="background: var(--danger);"></div>
                    <span class="text-xs" style="color: var(--text-secondary);">Danger</span>
                </div>
                <div class="text-center">
                    <div class="w-full h-16 rounded-lg mb-2" style="background: var(--info);"></div>
                    <span class="text-xs" style="color: var(--text-secondary);">Info</span>
                </div>
                <div class="text-center">
                    <div class="w-full h-16 rounded-lg mb-2" style="background: var(--purple);"></div>
                    <span class="text-xs" style="color: var(--text-secondary);">Purple</span>
                </div>
            </div>
        </section>

    </main>

    <!-- ===== Footer ===== -->
    <footer class="theme-transition" style="background: var(--bg-surface); border-top: 1px solid var(--border-subtle);">
        <div class="max-w-7xl mx-auto px-6 py-6 text-center">
            <p class="text-sm" style="color: var(--text-tertiary);">
                <span data-i18n="footerText">Ferox Theme System • مرجع شامل للمشاريع المستقبلية</span> •
                <span class="data-cell" style="color: var(--text-secondary);">v1.0.0</span>
            </p>
        </div>
    </footer>

    <!-- ===== JavaScript ===== -->
    <script>
        // Initialize saved preferences
        const savedTheme = localStorage.getItem('ferox-theme') || 'dark';
        let currentLang = localStorage.getItem('ferox-lang') || 'ar';

        // Apply theme
        document.documentElement.setAttribute('data-theme', savedTheme);
        updateThemeIcon(savedTheme);

        // Apply language
        document.documentElement.setAttribute('lang', currentLang);
        document.documentElement.setAttribute('dir', currentLang === 'ar' ? 'rtl' : 'ltr');
        updateLanguage(currentLang);
        updateLangButton(currentLang);

        function toggleTheme() {
            const html = document.documentElement;
            const currentTheme = html.getAttribute('data-theme');
            const newTheme = currentTheme === 'dark' ? 'light' : 'dark';

            html.setAttribute('data-theme', newTheme);
            localStorage.setItem('ferox-theme', newTheme);
            updateThemeIcon(newTheme);
        }

        function updateThemeIcon(theme) {
            const sunIcon = document.getElementById('sun-icon');
            const moonIcon = document.getElementById('moon-icon');

            if (theme === 'dark') {
                sunIcon.classList.add('hidden');
                moonIcon.classList.remove('hidden');
            } else {
                sunIcon.classList.remove('hidden');
                moonIcon.classList.add('hidden');
            }
        }

        function toggleLanguage() {
            currentLang = currentLang === 'ar' ? 'en' : 'ar';

            // Update direction
            document.documentElement.setAttribute('lang', currentLang);
            document.documentElement.setAttribute('dir', currentLang === 'ar' ? 'rtl' : 'ltr');

            // Save language
            localStorage.setItem('ferox-lang', currentLang);

            // Update texts
            updateLanguage(currentLang);
            updateLangButton(currentLang);
        }

        function updateLanguage(lang) {
            const elements = document.querySelectorAll('[data-i18n]');
            elements.forEach(el => {
                const key = el.getAttribute('data-i18n');
                if (translations[lang] && translations[lang][key]) {
                    el.textContent = translations[lang][key];
                }
            });
        }

        function updateLangButton(lang) {
            const langText = document.getElementById('lang-text');
            langText.textContent = lang === 'ar' ? 'EN' : 'AR';
        }
    </script>

</body>
</html>
"##.to_string()
}
