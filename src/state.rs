use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::{Order, Product, User};

pub type SharedUsers = Arc<Mutex<Vec<User>>>;
pub type SharedProducts = Arc<Mutex<Vec<Product>>>;
pub type SharedOrders = Arc<Mutex<Vec<Order>>>;

#[derive(Clone)]
pub struct AppState {
    pub users: SharedUsers,
    pub products: SharedProducts,
    pub orders: SharedOrders,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(seed_users())),
            products: Arc::new(Mutex::new(seed_products())),
            orders: Arc::new(Mutex::new(seed_orders())),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

fn seed_users() -> Vec<User> {
    vec![
        User { id: "1".into(), name: "Ada Lovelace".into(), email: "ada@example.com".into(), role: "admin".into() },
        User { id: "2".into(), name: "Grace Hopper".into(), email: "grace@example.com".into(), role: "engineer".into() },
        User { id: "3".into(), name: "Alan Turing".into(), email: "alan@example.com".into(), role: "researcher".into() },
        User { id: "4".into(), name: "Katherine Johnson".into(), email: "katherine@example.com".into(), role: "analyst".into() },
        User { id: "5".into(), name: "Linus Torvalds".into(), email: "linus@example.com".into(), role: "maintainer".into() },
    ]
}

fn seed_products() -> Vec<Product> {
    vec![
        Product {
            id: "p1".into(),
            name: "MacBook Pro 14\"".into(),
            description: "Apple M3 Pro chip, 18GB RAM, 512GB SSD".into(),
            price: 1999.00,
            category: "electronics".into(),
            stock: 25,
            image_url: "https://images.unsplash.com/photo-1517336714731-489689fd1ca8".into(),
        },
        Product {
            id: "p2".into(),
            name: "Sony WH-1000XM5".into(),
            description: "Wireless noise-canceling headphones with 30hr battery".into(),
            price: 349.99,
            category: "electronics".into(),
            stock: 50,
            image_url: "https://images.unsplash.com/photo-1505740420928-5e560c06d30e".into(),
        },
        Product {
            id: "p3".into(),
            name: "iPhone 15 Pro".into(),
            description: "A17 Pro chip, titanium design, 256GB".into(),
            price: 1199.00,
            category: "electronics".into(),
            stock: 100,
            image_url: "https://images.unsplash.com/photo-1592750475338-74b7b21085ab".into(),
        },
        Product {
            id: "p4".into(),
            name: "Samsung 4K Monitor".into(),
            description: "32\" UHD display with USB-C connectivity".into(),
            price: 599.99,
            category: "electronics".into(),
            stock: 30,
            image_url: "https://images.unsplash.com/photo-1527443224154-c4a3942d3acf".into(),
        },
        Product {
            id: "p5".into(),
            name: "Premium Cotton Hoodie".into(),
            description: "Organic cotton, relaxed fit, charcoal gray".into(),
            price: 89.00,
            category: "clothing".into(),
            stock: 200,
            image_url: "https://images.unsplash.com/photo-1556821840-3a63f95609a7".into(),
        },
        Product {
            id: "p6".into(),
            name: "Slim Fit Chinos".into(),
            description: "Stretch fabric, navy blue, all-season wear".into(),
            price: 79.00,
            category: "clothing".into(),
            stock: 150,
            image_url: "https://images.unsplash.com/photo-1473966968600-fa801b869a1a".into(),
        },
        Product {
            id: "p7".into(),
            name: "Merino Wool Sweater".into(),
            description: "Lightweight, breathable, forest green".into(),
            price: 129.00,
            category: "clothing".into(),
            stock: 75,
            image_url: "https://images.unsplash.com/photo-1434389677669-e08b4cac3105".into(),
        },
        Product {
            id: "p8".into(),
            name: "Leather Minimalist Wallet".into(),
            description: "Full-grain leather, RFID blocking, slim profile".into(),
            price: 59.00,
            category: "clothing".into(),
            stock: 300,
            image_url: "https://images.unsplash.com/photo-1627123424574-724758594e93".into(),
        },
    ]
}

fn seed_orders() -> Vec<Order> {
    vec![
        Order {
            id: "o1".into(),
            user_id: "1".into(),
            product_ids: vec!["p1".into(), "p2".into()],
            total: 2348.99,
            status: "delivered".into(),
            created_at: "2024-01-15T10:30:00Z".into(),
        },
        Order {
            id: "o2".into(),
            user_id: "2".into(),
            product_ids: vec!["p3".into()],
            total: 1199.00,
            status: "shipped".into(),
            created_at: "2024-01-18T14:22:00Z".into(),
        },
        Order {
            id: "o3".into(),
            user_id: "3".into(),
            product_ids: vec!["p5".into(), "p6".into(), "p7".into()],
            total: 297.00,
            status: "pending".into(),
            created_at: "2024-01-20T09:15:00Z".into(),
        },
        Order {
            id: "o4".into(),
            user_id: "4".into(),
            product_ids: vec!["p4".into(), "p8".into()],
            total: 658.99,
            status: "delivered".into(),
            created_at: "2024-01-10T16:45:00Z".into(),
        },
        Order {
            id: "o5".into(),
            user_id: "5".into(),
            product_ids: vec!["p1".into()],
            total: 1999.00,
            status: "shipped".into(),
            created_at: "2024-01-19T11:00:00Z".into(),
        },
    ]
}
