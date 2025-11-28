<div align="center">

# Ferox Theme System

A production-ready, modern theme system built with Rust and Axum featuring dark/light modes, RTL/LTR internationalization, and a comprehensive component library.

[![CI](https://github.com/abdulwahed-sweden/axum-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/abdulwahed-sweden/axum-rust/actions/workflows/rust.yml)
[![Rust](https://img.shields.io/badge/Rust-1.81+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue?style=flat-square)](https://github.com/tokio-rs/axum)
[![License: MIT](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)

[Features](#features) • [Quick Start](#quick-start) • [Documentation](#documentation) • [API Reference](#api-reference)

</div>

---

## About

Ferox Theme System is a complete, production-ready theme implementation demonstrating best practices for building modern web applications with Rust. It features a comprehensive design system with CSS variables, dark/light theme switching, Arabic/English internationalization with RTL/LTR support, and a full component library.

## Features

### Theme System
- **Dark/Light Mode** — Seamless theme switching with smooth 300ms transitions
- **CSS Variables** — 40+ design tokens for complete customization
- **Persistent Preferences** — User choices saved to localStorage

### Internationalization
- **RTL/LTR Support** — Full Arabic and English language support
- **Dynamic Translation** — JavaScript-based i18n with `data-i18n` attributes
- **Direction-Aware Layout** — Automatic layout adjustment for text direction

### Components
- **Statistics Cards** — Animated counters with trend indicators
- **Data Tables** — Sortable tables with status badges
- **Buttons** — Primary, secondary, success, and danger variants
- **Badges** — Status indicators in multiple colors
- **Alerts** — Info, success, warning, and error states
- **Form Inputs** — Text fields with labels and helper text

### Technical
- **Blazing Fast** — Built with Rust for maximum performance
- **Type-Safe** — Leverages Rust's type system for compile-time guarantees
- **Async-First** — Powered by Tokio for high-performance async I/O
- **Modern Stack** — Tailwind CSS + Custom CSS Variables

## Tech Stack

| Technology | Version | Purpose |
|------------|---------|---------|
| [Rust](https://www.rust-lang.org/) | 1.81+ | Systems programming language |
| [Axum](https://github.com/tokio-rs/axum) | 0.8 | Ergonomic web framework |
| [Tokio](https://tokio.rs/) | 1.48 | Async runtime |
| [Tailwind CSS](https://tailwindcss.com/) | 3.x | Utility-first CSS (via CDN) |

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.81 or higher
- Cargo (comes with Rust)

### Installation

```bash
# Clone the repository
git clone https://github.com/abdulwahed-sweden/axum-rust.git
cd axum-rust

# Build the project
cargo build --release

# Run the server
cargo run --release

# Open in browser
open http://127.0.0.1:3000
```

## Documentation

Comprehensive documentation is available in the [`docs/`](docs/) folder:

| Document | Description |
|----------|-------------|
| [Getting Started](docs/README.md) | Documentation index and overview |
| [Theme System](docs/THEME_SYSTEM.md) | Complete theme architecture guide |
| [CSS Variables](docs/CSS_VARIABLES.md) | All design tokens reference |
| [Components](docs/COMPONENTS.md) | Component library with examples |
| [Typography](docs/TYPOGRAPHY.md) | Font system and text styles |
| [Colors](docs/COLORS.md) | Color palette and usage |
| [Dark/Light Mode](docs/DARK_LIGHT_MODE.md) | Theme switching implementation |
| [Internationalization](docs/I18N.md) | RTL/LTR and translation system |
| [Quick Reference](docs/QUICK_REFERENCE.md) | Copy-paste code snippets |

## API Reference

### Users

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| `GET` | `/users` | List all users | `200` JSON array |
| `GET` | `/users/{id}` | Get user by ID | `200` JSON / `404` |
| `POST` | `/users` | Create a new user | `201` JSON / `409` |
| `DELETE` | `/users/{id}` | Delete a user | `204` / `404` |

### Products

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| `GET` | `/products` | List all products | `200` JSON array |
| `GET` | `/products/{id}` | Get product by ID | `200` JSON / `404` |
| `POST` | `/products` | Create a new product | `201` JSON / `409` |

### Orders

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| `GET` | `/orders` | List all orders | `200` JSON array |
| `GET` | `/orders/{id}` | Get order by ID | `200` JSON / `404` |
| `POST` | `/orders` | Create a new order | `201` JSON / `409` |

## Project Structure

```
axum-rust/
├── .github/
│   └── workflows/
│       └── rust.yml           # CI workflow
├── docs/
│   ├── README.md              # Documentation index
│   ├── THEME_SYSTEM.md        # Theme architecture
│   ├── CSS_VARIABLES.md       # Design tokens
│   ├── COMPONENTS.md          # Component library
│   ├── TYPOGRAPHY.md          # Font system
│   ├── COLORS.md              # Color palette
│   ├── DARK_LIGHT_MODE.md     # Theme switching
│   ├── I18N.md                # Internationalization
│   └── QUICK_REFERENCE.md     # Code snippets
├── src/
│   ├── main.rs                # Server setup and routing
│   ├── state.rs               # Application state
│   ├── models/
│   │   └── mod.rs             # Data models
│   ├── routes/
│   │   ├── mod.rs             # User routes
│   │   ├── products.rs        # Product routes
│   │   └── orders.rs          # Order routes
│   └── templates/
│       └── mod.rs             # Ferox Theme System
├── Cargo.toml
├── CHANGELOG.md
└── README.md
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

Built with Rust

**[Back to top](#ferox-theme-system)**

</div>
