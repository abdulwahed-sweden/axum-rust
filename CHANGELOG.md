# Changelog

All notable changes to the Ferox Theme System will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2024-11-28

### Added
- **Internationalization (i18n)**: Full Arabic/English language support
  - `data-i18n` attributes for translatable elements
  - Dynamic language switching with `toggleLanguage()` function
  - RTL/LTR layout support with automatic direction switching
  - Translation system with `translations` object

- **Typography improvements**
  - Increased base font size from 15px to 16px for better readability
  - Maintained number display sizes (they were already optimal)
  - Added `font-variant-numeric: tabular-nums` for consistent number alignment

- **Language toggle button**
  - AR/EN toggle next to theme toggle
  - Persistent language preference via localStorage
  - Automatic layout direction change (RTL ↔ LTR)

- **Comprehensive documentation**
  - `docs/README.md` - Documentation index
  - `docs/THEME_SYSTEM.md` - Theme architecture guide
  - `docs/CSS_VARIABLES.md` - Design tokens reference
  - `docs/COMPONENTS.md` - Component library
  - `docs/TYPOGRAPHY.md` - Typography system
  - `docs/COLORS.md` - Color palette
  - `docs/DARK_LIGHT_MODE.md` - Theme switching guide
  - `docs/I18N.md` - Internationalization guide
  - `docs/QUICK_REFERENCE.md` - Copy-paste snippets

### Changed
- Updated main README.md with Ferox Theme System branding
- Improved project structure documentation
- Enhanced component examples with i18n support

## [1.0.0] - 2024-11-28

### Added
- **Ferox Theme System**: Complete theme implementation
  - Dark/Light mode with smooth 300ms transitions
  - CSS Variables for all design tokens
  - localStorage persistence for theme preference

- **CSS Variables**
  - Background colors: `--bg-base`, `--bg-surface`, `--bg-elevated`
  - Text colors: `--text-primary`, `--text-secondary`, `--text-muted`
  - Border colors: `--border-default`, `--border-subtle`
  - Semantic colors: `--success`, `--warning`, `--error`, `--info`
  - Soft variants: `--success-soft`, `--warning-soft`, `--error-soft`, `--info-soft`

- **Component Library**
  - Statistics cards with trend indicators
  - Data tables with status badges
  - Buttons: primary, secondary, success, warning, danger, ghost
  - Badges: success, warning, error, info
  - Alerts: info, success, warning, error
  - Form inputs with labels and helper text

- **Typography System**
  - IBM Plex Sans Arabic for Arabic text
  - Inter for English text
  - JetBrains Mono for code and numbers
  - Responsive font sizing

- **Theme Toggle**
  - Sun/Moon icon toggle button
  - Smooth icon transition
  - Keyboard accessible

### Technical
- Built with Rust + Axum framework
- Tailwind CSS via CDN
- Zero runtime dependencies for theme system
- WCAG AA compliant color contrast

---

## Version History

| Version | Date | Description |
|---------|------|-------------|
| 1.1.0 | 2024-11-28 | Added i18n support, documentation |
| 1.0.0 | 2024-11-28 | Initial Ferox Theme System |

---

## Upgrade Guide

### From 1.0.0 to 1.1.0

1. **Add translation object** to your page:
```javascript
const translations = {
    ar: { /* Arabic translations */ },
    en: { /* English translations */ }
};
```

2. **Add `data-i18n` attributes** to translatable elements:
```html
<h1 data-i18n="title">العنوان</h1>
```

3. **Add language toggle function**:
```javascript
function toggleLanguage() {
    const lang = document.documentElement.lang === 'ar' ? 'en' : 'ar';
    document.documentElement.lang = lang;
    document.documentElement.dir = lang === 'ar' ? 'rtl' : 'ltr';
    // Update translations...
}
```

4. **Update font size** (optional but recommended):
```css
html { font-size: 16px; }  /* Was 15px */
```

---

<div align="center">

[Back to README](README.md)

</div>
