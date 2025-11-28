pub fn landing_page() -> String {
    r##"
<!DOCTYPE html>
<html lang="ar" dir="rtl" data-theme="dark">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ferox Theme System - مرجع شامل</title>

    <!-- الخطوط -->
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

    <!-- CSS Variables للثيم -->
    <style>
        /* ========== الثيم الداكن (افتراضي) ========== */
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

        /* ========== الثيم الفاتح ========== */
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

        /* الأنماط الأساسية */
        html { font-size: 15px; }

        body {
            font-family: 'IBM Plex Sans Arabic', 'Inter', system-ui, sans-serif;
            background-color: var(--bg-base);
            color: var(--text-primary);
            line-height: 1.6;
            transition: background-color 0.2s ease, color 0.2s ease;
        }

        /* Scrollbar */
        ::-webkit-scrollbar { width: 8px; height: 8px; }
        ::-webkit-scrollbar-track { background: var(--bg-base); }
        ::-webkit-scrollbar-thumb { background: var(--border-strong); border-radius: 9999px; }
        ::-webkit-scrollbar-thumb:hover { background: var(--text-tertiary); }

        /* Selection */
        ::selection { background: var(--primary-soft); color: var(--text-primary); }

        /* الأرقام */
        .number-display {
            font-family: 'Inter', 'IBM Plex Sans Arabic', system-ui, sans-serif;
            font-weight: 700;
            font-variant-numeric: tabular-nums;
            letter-spacing: -0.02em;
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

        /* Theme Toggle */
        .theme-toggle {
            background: var(--bg-elevated);
            border: 1px solid var(--border-subtle);
            color: var(--text-secondary);
            transition: all 0.2s ease;
        }
        .theme-toggle:hover {
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
    </style>
</head>
<body class="min-h-screen">

    <!-- ===== الهيدر ===== -->
    <header class="theme-transition" style="background: var(--bg-surface); border-bottom: 1px solid var(--border-subtle);">
        <div class="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
            <!-- الشعار -->
            <div class="flex items-center gap-3">
                <div class="w-10 h-10 rounded-lg flex items-center justify-center text-white font-bold text-xl" style="background: var(--primary);">
                    F
                </div>
                <span class="font-bold text-lg" style="color: var(--text-primary);">Ferox Theme</span>
            </div>

            <!-- التنقل -->
            <nav class="flex items-center gap-6">
                <a href="#stats" class="nav-link text-sm font-medium">الإحصائيات</a>
                <a href="#table" class="nav-link text-sm font-medium">الجداول</a>
                <a href="#components" class="nav-link text-sm font-medium">المكونات</a>
                <a href="#api" class="nav-link text-sm font-medium">API</a>
            </nav>

            <!-- زر التبديل -->
            <button id="theme-toggle" class="theme-toggle w-10 h-10 rounded-lg flex items-center justify-center" onclick="toggleTheme()">
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
    </header>

    <main class="max-w-7xl mx-auto px-6 py-8">

        <!-- ===== Hero Section ===== -->
        <section class="text-center py-12 mb-12">
            <h1 class="text-4xl font-bold mb-4" style="color: var(--text-primary);">
                نظام Ferox Theme
            </h1>
            <p class="text-lg mb-6" style="color: var(--text-secondary);">
                مرجع شامل للثيم الداكن والفاتح - مصمم للعمل الطويل وراحة العين
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

        <!-- ===== بطاقات الإحصائيات ===== -->
        <section id="stats" class="mb-12">
            <h2 class="text-xl font-bold mb-6" style="color: var(--text-primary);">الإحصائيات</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">

                <!-- بطاقة 1 -->
                <div class="stat-card rounded-xl p-5 cursor-pointer">
                    <div class="flex justify-between items-start">
                        <div>
                            <p class="text-sm mb-2" style="color: var(--text-secondary);">المستخدمون</p>
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

                <!-- بطاقة 2 -->
                <div class="stat-card rounded-xl p-5 cursor-pointer">
                    <div class="flex justify-between items-start">
                        <div>
                            <p class="text-sm mb-2" style="color: var(--text-secondary);">الإيرادات</p>
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

                <!-- بطاقة 3 -->
                <div class="stat-card rounded-xl p-5 cursor-pointer">
                    <div class="flex justify-between items-start">
                        <div>
                            <p class="text-sm mb-2" style="color: var(--text-secondary);">الطلبات</p>
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

                <!-- بطاقة 4 -->
                <div class="stat-card rounded-xl p-5 cursor-pointer">
                    <div class="flex justify-between items-start">
                        <div>
                            <p class="text-sm mb-2" style="color: var(--text-secondary);">معدل التحويل</p>
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

        <!-- ===== الجدول ===== -->
        <section id="table" class="mb-12">
            <h2 class="text-xl font-bold mb-6" style="color: var(--text-primary);">جدول المعاملات</h2>
            <div class="rounded-xl overflow-hidden theme-transition"
                 style="background: var(--bg-surface); border: 1px solid var(--border-subtle); box-shadow: var(--shadow-card);">
                <table class="w-full">
                    <thead>
                        <tr style="background: var(--bg-elevated);">
                            <th class="text-right px-4 py-3 text-xs font-semibold uppercase tracking-wider" style="color: var(--text-tertiary);">الاسم</th>
                            <th class="text-right px-4 py-3 text-xs font-semibold uppercase tracking-wider" style="color: var(--text-tertiary);">الحالة</th>
                            <th class="text-right px-4 py-3 text-xs font-semibold uppercase tracking-wider" style="color: var(--text-tertiary);">المبلغ</th>
                            <th class="text-right px-4 py-3 text-xs font-semibold uppercase tracking-wider" style="color: var(--text-tertiary);">التغيير</th>
                            <th class="text-right px-4 py-3 text-xs font-semibold uppercase tracking-wider" style="color: var(--text-tertiary);">التاريخ</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr class="table-row" style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="px-4 py-3" style="color: var(--text-primary);">أحمد محمد</td>
                            <td class="px-4 py-3">
                                <span class="px-2 py-0.5 rounded-full text-xs font-medium"
                                      style="background: var(--success-soft); color: var(--success-text); border: 1px solid var(--success-border);">
                                    مكتمل
                                </span>
                            </td>
                            <td class="px-4 py-3 data-cell font-medium" style="color: var(--text-number-secondary);">$2,450.00</td>
                            <td class="px-4 py-3 data-cell font-medium" style="color: var(--data-positive);">+12.5%</td>
                            <td class="px-4 py-3 text-sm" style="color: var(--text-tertiary);">28 نوفمبر 2025</td>
                        </tr>
                        <tr class="table-row" style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="px-4 py-3" style="color: var(--text-primary);">سارة علي</td>
                            <td class="px-4 py-3">
                                <span class="px-2 py-0.5 rounded-full text-xs font-medium"
                                      style="background: var(--warning-soft); color: var(--warning-text); border: 1px solid var(--warning-border);">
                                    معلق
                                </span>
                            </td>
                            <td class="px-4 py-3 data-cell font-medium" style="color: var(--text-number-secondary);">$1,890.00</td>
                            <td class="px-4 py-3 data-cell font-medium" style="color: var(--data-negative);">-3.2%</td>
                            <td class="px-4 py-3 text-sm" style="color: var(--text-tertiary);">27 نوفمبر 2025</td>
                        </tr>
                        <tr class="table-row" style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="px-4 py-3" style="color: var(--text-primary);">خالد عبدالله</td>
                            <td class="px-4 py-3">
                                <span class="px-2 py-0.5 rounded-full text-xs font-medium"
                                      style="background: var(--success-soft); color: var(--success-text); border: 1px solid var(--success-border);">
                                    مكتمل
                                </span>
                            </td>
                            <td class="px-4 py-3 data-cell font-medium" style="color: var(--text-number-secondary);">$3,200.00</td>
                            <td class="px-4 py-3 data-cell font-medium" style="color: var(--data-positive);">+8.7%</td>
                            <td class="px-4 py-3 text-sm" style="color: var(--text-tertiary);">26 نوفمبر 2025</td>
                        </tr>
                        <tr class="table-row" style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="px-4 py-3" style="color: var(--text-primary);">نورة سعيد</td>
                            <td class="px-4 py-3">
                                <span class="px-2 py-0.5 rounded-full text-xs font-medium"
                                      style="background: var(--danger-soft); color: var(--danger-text); border: 1px solid var(--danger-border);">
                                    ملغي
                                </span>
                            </td>
                            <td class="px-4 py-3 data-cell font-medium" style="color: var(--text-number-secondary);">$950.00</td>
                            <td class="px-4 py-3 data-cell font-medium" style="color: var(--data-negative);">-15.3%</td>
                            <td class="px-4 py-3 text-sm" style="color: var(--text-tertiary);">25 نوفمبر 2025</td>
                        </tr>
                        <tr class="table-row">
                            <td class="px-4 py-3" style="color: var(--text-primary);">محمد إبراهيم</td>
                            <td class="px-4 py-3">
                                <span class="px-2 py-0.5 rounded-full text-xs font-medium"
                                      style="background: var(--info-soft); color: var(--info-text); border: 1px solid var(--info-border);">
                                    قيد المعالجة
                                </span>
                            </td>
                            <td class="px-4 py-3 data-cell font-medium" style="color: var(--text-number-secondary);">$4,100.00</td>
                            <td class="px-4 py-3 data-cell font-medium" style="color: var(--data-positive);">+22.1%</td>
                            <td class="px-4 py-3 text-sm" style="color: var(--text-tertiary);">24 نوفمبر 2025</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </section>

        <!-- ===== المكونات ===== -->
        <section id="components" class="mb-12">
            <h2 class="text-xl font-bold mb-6" style="color: var(--text-primary);">المكونات</h2>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">

                <!-- الأزرار -->
                <div class="rounded-xl p-6 theme-transition"
                     style="background: var(--bg-surface); border: 1px solid var(--border-subtle);">
                    <h3 class="font-semibold mb-4" style="color: var(--text-primary);">الأزرار</h3>
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

                <!-- الشارات -->
                <div class="rounded-xl p-6 theme-transition"
                     style="background: var(--bg-surface); border: 1px solid var(--border-subtle);">
                    <h3 class="font-semibold mb-4" style="color: var(--text-primary);">الشارات</h3>
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

                <!-- التنبيهات -->
                <div class="rounded-xl p-6 theme-transition"
                     style="background: var(--bg-surface); border: 1px solid var(--border-subtle);">
                    <h3 class="font-semibold mb-4" style="color: var(--text-primary);">التنبيهات</h3>
                    <div class="space-y-3">
                        <div class="p-3 rounded-lg text-sm"
                             style="background: var(--success-soft); color: var(--success-text); border: 1px solid var(--success-border);">
                            تمت العملية بنجاح!
                        </div>
                        <div class="p-3 rounded-lg text-sm"
                             style="background: var(--warning-soft); color: var(--warning-text); border: 1px solid var(--warning-border);">
                            تحذير: يرجى التحقق من البيانات.
                        </div>
                        <div class="p-3 rounded-lg text-sm"
                             style="background: var(--danger-soft); color: var(--danger-text); border: 1px solid var(--danger-border);">
                            حدث خطأ أثناء المعالجة.
                        </div>
                        <div class="p-3 rounded-lg text-sm"
                             style="background: var(--info-soft); color: var(--info-text); border: 1px solid var(--info-border);">
                            معلومة: هذه رسالة إعلامية.
                        </div>
                    </div>
                </div>

                <!-- حقول الإدخال -->
                <div class="rounded-xl p-6 theme-transition"
                     style="background: var(--bg-surface); border: 1px solid var(--border-subtle);">
                    <h3 class="font-semibold mb-4" style="color: var(--text-primary);">حقول الإدخال</h3>
                    <div class="space-y-4">
                        <div>
                            <label class="block text-sm font-medium mb-1" style="color: var(--text-secondary);">البريد الإلكتروني</label>
                            <input type="email" placeholder="example@email.com"
                                   class="input-field w-full px-3 py-2 rounded-lg text-sm">
                        </div>
                        <div>
                            <label class="block text-sm font-medium mb-1" style="color: var(--text-secondary);">المبلغ</label>
                            <input type="text" placeholder="0.00" value="1,234.56"
                                   class="input-field w-full px-3 py-2 rounded-lg text-sm data-cell">
                        </div>
                    </div>
                </div>

            </div>
        </section>

        <!-- ===== API Reference ===== -->
        <section id="api" class="mb-12">
            <h2 class="text-xl font-bold mb-6" style="color: var(--text-primary);">API Endpoints</h2>
            <div class="rounded-xl overflow-hidden theme-transition"
                 style="background: var(--bg-surface); border: 1px solid var(--border-subtle);">
                <table class="w-full">
                    <thead>
                        <tr style="background: var(--bg-elevated);">
                            <th class="text-right px-4 py-3 text-xs font-semibold uppercase" style="color: var(--text-tertiary);">Method</th>
                            <th class="text-right px-4 py-3 text-xs font-semibold uppercase" style="color: var(--text-tertiary);">Endpoint</th>
                            <th class="text-right px-4 py-3 text-xs font-semibold uppercase" style="color: var(--text-tertiary);">الوصف</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="px-4 py-3">
                                <span class="px-2 py-0.5 rounded text-xs font-mono font-medium"
                                      style="background: var(--success-soft); color: var(--success-text);">GET</span>
                            </td>
                            <td class="px-4 py-3 font-mono text-sm" style="color: var(--text-primary);">/users</td>
                            <td class="px-4 py-3 text-sm" style="color: var(--text-secondary);">قائمة المستخدمين</td>
                        </tr>
                        <tr style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="px-4 py-3">
                                <span class="px-2 py-0.5 rounded text-xs font-mono font-medium"
                                      style="background: var(--primary-soft); color: var(--primary-text);">POST</span>
                            </td>
                            <td class="px-4 py-3 font-mono text-sm" style="color: var(--text-primary);">/users</td>
                            <td class="px-4 py-3 text-sm" style="color: var(--text-secondary);">إنشاء مستخدم جديد</td>
                        </tr>
                        <tr style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="px-4 py-3">
                                <span class="px-2 py-0.5 rounded text-xs font-mono font-medium"
                                      style="background: var(--success-soft); color: var(--success-text);">GET</span>
                            </td>
                            <td class="px-4 py-3 font-mono text-sm" style="color: var(--text-primary);">/products</td>
                            <td class="px-4 py-3 text-sm" style="color: var(--text-secondary);">قائمة المنتجات</td>
                        </tr>
                        <tr style="border-bottom: 1px solid var(--border-subtle);">
                            <td class="px-4 py-3">
                                <span class="px-2 py-0.5 rounded text-xs font-mono font-medium"
                                      style="background: var(--success-soft); color: var(--success-text);">GET</span>
                            </td>
                            <td class="px-4 py-3 font-mono text-sm" style="color: var(--text-primary);">/orders</td>
                            <td class="px-4 py-3 text-sm" style="color: var(--text-secondary);">قائمة الطلبات</td>
                        </tr>
                        <tr>
                            <td class="px-4 py-3">
                                <span class="px-2 py-0.5 rounded text-xs font-mono font-medium"
                                      style="background: var(--danger-soft); color: var(--danger-text);">DELETE</span>
                            </td>
                            <td class="px-4 py-3 font-mono text-sm" style="color: var(--text-primary);">/users/{id}</td>
                            <td class="px-4 py-3 text-sm" style="color: var(--text-secondary);">حذف مستخدم</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </section>

        <!-- ===== ملخص الألوان ===== -->
        <section class="mb-12">
            <h2 class="text-xl font-bold mb-6" style="color: var(--text-primary);">ملخص الألوان</h2>
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

    <!-- ===== الفوتر ===== -->
    <footer class="theme-transition" style="background: var(--bg-surface); border-top: 1px solid var(--border-subtle);">
        <div class="max-w-7xl mx-auto px-6 py-6 text-center">
            <p class="text-sm" style="color: var(--text-tertiary);">
                Ferox Theme System - مرجع شامل للمشاريع المستقبلية -
                <span class="data-cell" style="color: var(--text-secondary);">v1.0.0</span>
            </p>
        </div>
    </footer>

    <!-- ===== JavaScript للتبديل ===== -->
    <script>
        // قراءة الثيم المحفوظ
        const savedTheme = localStorage.getItem('ferox-theme') || 'dark';
        document.documentElement.setAttribute('data-theme', savedTheme);
        updateThemeIcon(savedTheme);

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
    </script>

</body>
</html>
"##.to_string()
}
