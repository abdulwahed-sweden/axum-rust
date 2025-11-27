<div align="center">

# Axum API

A blazing-fast REST API built with Rust and Axum.

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue?style=flat-square)](https://github.com/tokio-rs/axum)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)

[Features](#features) • [Tech Stack](#tech-stack) • [Getting Started](#getting-started) • [API Reference](#api-reference) • [Screenshots](#screenshots)

</div>

---

## Features

- **Type-Safe** — Leverages Rust's type system for compile-time guarantees
- **Async-First** — Built on Tokio for high-performance async I/O
- **In-Memory Store** — Demo data for users, products, and orders
- **Modern UI** — Beautiful landing page with Tailwind CSS and glassmorphism design
- **RESTful API** — Clean, intuitive endpoints following REST conventions
- **Zero Boilerplate** — Minimal setup with sensible defaults

## Tech Stack

| Technology | Purpose |
|------------|---------|
| [Rust](https://www.rust-lang.org/) | Systems programming language |
| [Axum](https://github.com/tokio-rs/axum) | Web framework |
| [Tokio](https://tokio.rs/) | Async runtime |
| [Serde](https://serde.rs/) | Serialization/deserialization |
| [Tailwind CSS](https://tailwindcss.com/) | Styling (via CDN) |

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) 1.75 or higher
- Cargo (comes with Rust)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/abdulwahed-sweden/axum-rust.git
   cd axum-rust
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Run the server**
   ```bash
   cargo run --release
   ```

4. **Open in browser**
   ```
   http://127.0.0.1:3000
   ```

## API Reference

### Users

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/users` | List all users |
| `GET` | `/users/{id}` | Get user by ID |
| `POST` | `/users` | Create a new user |
| `DELETE` | `/users/{id}` | Delete a user |

### Products

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/products` | List all products |
| `GET` | `/products/{id}` | Get product by ID |
| `POST` | `/products` | Create a new product |

### Orders

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/orders` | List all orders |
| `GET` | `/orders/{id}` | Get order by ID |
| `POST` | `/orders` | Create a new order |

### Other

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Landing page |
| `GET` | `/hello` | Health check |

### Example Requests

**Create a user:**
```bash
curl -X POST http://127.0.0.1:3000/users \
  -H "Content-Type: application/json" \
  -d '{"id": "6", "name": "John Doe", "email": "john@example.com", "role": "user"}'
```

**Get all products:**
```bash
curl http://127.0.0.1:3000/products
```

**Create an order:**
```bash
curl -X POST http://127.0.0.1:3000/orders \
  -H "Content-Type: application/json" \
  -d '{"id": "o6", "user_id": "1", "product_ids": ["p1", "p2"], "total": 2348.99, "status": "pending", "created_at": "2024-01-21T10:00:00Z"}'
```

## Project Structure

```
axum-rust/
├── src/
│   ├── main.rs           # Server setup and routing
│   ├── state.rs          # Application state and demo data
│   ├── models/
│   │   └── mod.rs        # User, Product, Order structs
│   ├── routes/
│   │   ├── mod.rs        # User routes and handlers
│   │   ├── products.rs   # Product routes
│   │   └── orders.rs     # Order routes
│   └── templates/
│       └── mod.rs        # HTML templates (header, footer, landing)
├── Cargo.toml
└── README.md
```

## Screenshots

<div align="center">

### Landing Page

<!-- Add your screenshot here -->
*Screenshot placeholder - Landing page with gradient background and endpoint cards*

### API Response

<!-- Add your screenshot here -->
*Screenshot placeholder - JSON response from /users endpoint*

</div>

## Demo Data

The API comes pre-loaded with:

- **5 Users** — Ada Lovelace, Grace Hopper, Alan Turing, Katherine Johnson, Linus Torvalds
- **8 Products** — Mix of electronics (MacBook, iPhone, headphones) and clothing (hoodie, chinos, sweater)
- **5 Orders** — Various statuses: pending, shipped, delivered

## License

This project is open source and available under the [MIT License](LICENSE).

---

<div align="center">

Built with Rust

</div>
