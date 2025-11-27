use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
    pub stock: u32,
    pub image_url: String,
}

#[derive(Deserialize)]
pub struct NewProduct {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
    pub stock: u32,
    pub image_url: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub product_ids: Vec<String>,
    pub total: f64,
    pub status: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct NewOrder {
    pub id: String,
    pub user_id: String,
    pub product_ids: Vec<String>,
    pub total: f64,
    pub status: String,
    pub created_at: String,
}
