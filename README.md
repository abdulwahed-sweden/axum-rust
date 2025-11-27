<div align="center">

# Axum API

A blazing-fast, type-safe REST API built with Rust and Axum.

[![CI](https://github.com/abdulwahed-sweden/axum-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/abdulwahed-sweden/axum-rust/actions/workflows/rust.yml)
[![Rust](https://img.shields.io/badge/Rust-1.80+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue?style=flat-square)](https://github.com/tokio-rs/axum)
[![License: MIT](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)

[Features](#features) • [Tech Stack](#tech-stack) • [Getting Started](#getting-started) • [API Reference](#api-reference) • [Screenshots](#screenshots)

</div>

---

## About

Axum API is a modern REST API demonstrating best practices for building web services with Rust. It features a complete CRUD implementation for users, products, and orders with an elegant landing page built using Tailwind CSS.

## Features

- **Blazing Fast** — Built with Rust for maximum performance and minimal latency
- **Type-Safe** — Leverages Rust's type system for compile-time guarantees
- **Async-First** — Powered by Tokio for high-performance async I/O
- **Modern UI** — Beautiful landing page with Tailwind CSS, glassmorphism, and animated gradients
- **RESTful Design** — Clean, intuitive endpoints following REST conventions
- **In-Memory Store** — Pre-loaded demo data for users, products, and orders
- **CI/CD Ready** — GitHub Actions workflow with formatting, linting, and MSRV checks

## Tech Stack

| Technology | Version | Purpose |
|------------|---------|---------|
| [Rust](https://www.rust-lang.org/) | 1.80+ | Systems programming language |
| [Axum](https://github.com/tokio-rs/axum) | 0.8 | Ergonomic web framework |
| [Tokio](https://tokio.rs/) | 1.48 | Async runtime |
| [Serde](https://serde.rs/) | 1.0 | JSON serialization/deserialization |
| [Tailwind CSS](https://tailwindcss.com/) | 3.x | Utility-first CSS (via CDN) |

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) 1.80 or higher
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

### Development

```bash
# Run with hot reload (install cargo-watch first)
cargo watch -x run

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test
```

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

### Other

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Landing page (HTML) |
| `GET` | `/hello` | Health check endpoint |

### Example Requests

<details>
<summary><b>Create a user</b></summary>

```bash
curl -X POST http://127.0.0.1:3000/users \
  -H "Content-Type: application/json" \
  -d '{
    "id": "6",
    "name": "John Doe",
    "email": "john@example.com",
    "role": "user"
  }'
```
</details>

<details>
<summary><b>Create a product</b></summary>

```bash
curl -X POST http://127.0.0.1:3000/products \
  -H "Content-Type: application/json" \
  -d '{
    "id": "p9",
    "name": "Wireless Keyboard",
    "description": "Mechanical RGB keyboard",
    "price": 149.99,
    "category": "electronics",
    "stock": 50,
    "image_url": "https://example.com/keyboard.jpg"
  }'
```
</details>

<details>
<summary><b>Create an order</b></summary>

```bash
curl -X POST http://127.0.0.1:3000/orders \
  -H "Content-Type: application/json" \
  -d '{
    "id": "o6",
    "user_id": "1",
    "product_ids": ["p1", "p2"],
    "total": 2348.99,
    "status": "pending",
    "created_at": "2024-01-21T10:00:00Z"
  }'
```
</details>

## Project Structure

```
axum-rust/
├── .github/
│   └── workflows/
│       └── rust.yml        # CI workflow
├── src/
│   ├── main.rs             # Server setup and routing
│   ├── state.rs            # Application state and demo data
│   ├── models/
│   │   └── mod.rs          # User, Product, Order structs
│   ├── routes/
│   │   ├── mod.rs          # User routes and handlers
│   │   ├── products.rs     # Product routes
│   │   └── orders.rs       # Order routes
│   └── templates/
│       └── mod.rs          # HTML templates
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Screenshots

<div align="center">

### Landing Page

<!-- Add your screenshot here -->
*Beautiful dark theme with animated grid background and gradient text*

### API Response

<!-- Add your screenshot here -->
*Clean JSON responses with proper status codes*

</div>

## Demo Data

The API comes pre-loaded with sample data:

| Resource | Count | Examples |
|----------|-------|----------|
| Users | 5 | Ada Lovelace, Grace Hopper, Alan Turing |
| Products | 8 | MacBook Pro, iPhone 15, Sony Headphones |
| Orders | 5 | Various statuses: pending, shipped, delivered |

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

**[Back to top](#axum-api)**

</div>
