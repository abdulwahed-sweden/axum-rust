use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    models::{NewOrder, Order},
    state::AppState,
};

pub async fn list_orders(State(state): State<AppState>) -> Json<Vec<Order>> {
    let orders = state.orders.lock().await;
    Json(orders.clone())
}

pub async fn get_order(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Order>, StatusCode> {
    let orders = state.orders.lock().await;
    orders
        .iter()
        .find(|order| order.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_order(
    State(state): State<AppState>,
    Json(payload): Json<NewOrder>,
) -> Result<(StatusCode, Json<Order>), StatusCode> {
    let mut orders = state.orders.lock().await;

    if orders.iter().any(|order| order.id == payload.id) {
        return Err(StatusCode::CONFLICT);
    }

    let order = Order {
        id: payload.id,
        user_id: payload.user_id,
        product_ids: payload.product_ids,
        total: payload.total,
        status: payload.status,
        created_at: payload.created_at,
    };
    orders.push(order.clone());
    Ok((StatusCode::CREATED, Json(order)))
}
