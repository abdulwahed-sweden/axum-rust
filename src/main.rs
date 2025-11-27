mod models;
mod routes;
mod state;
mod templates;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState::new();

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/hello", get(routes::hello))
        .route("/users", get(routes::list_users).post(routes::create_user))
        .route(
            "/users/{id}",
            get(routes::get_user).delete(routes::delete_user),
        )
        .route(
            "/products",
            get(routes::products::list_products).post(routes::products::create_product),
        )
        .route("/products/{id}", get(routes::products::get_product))
        .route(
            "/orders",
            get(routes::orders::list_orders).post(routes::orders::create_order),
        )
        .route("/orders/{id}", get(routes::orders::get_order))
        .with_state(state);

    let addr: SocketAddr = "127.0.0.1:3000".parse()?;
    let listener = TcpListener::bind(addr).await?;
    let bound_addr = listener.local_addr()?;
    println!("listening on http://{bound_addr}");

    axum::serve(listener, app).await?;
    Ok(())
}
