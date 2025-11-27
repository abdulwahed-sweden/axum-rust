use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    models::{NewProduct, Product},
    state::AppState,
};

pub async fn list_products(State(state): State<AppState>) -> Json<Vec<Product>> {
    let products = state.products.lock().await;
    Json(products.clone())
}

pub async fn get_product(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Product>, StatusCode> {
    let products = state.products.lock().await;
    products
        .iter()
        .find(|product| product.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<NewProduct>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    let mut products = state.products.lock().await;

    if products.iter().any(|product| product.id == payload.id) {
        return Err(StatusCode::CONFLICT);
    }

    let product = Product {
        id: payload.id,
        name: payload.name,
        description: payload.description,
        price: payload.price,
        category: payload.category,
        stock: payload.stock,
        image_url: payload.image_url,
    };
    products.push(product.clone());
    Ok((StatusCode::CREATED, Json(product)))
}
