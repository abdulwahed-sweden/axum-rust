pub mod orders;
pub mod products;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    Json,
};

use crate::{
    models::{NewUser, User},
    state::AppState,
    templates,
};

pub async fn index() -> Html<String> {
    Html(templates::landing_page())
}

pub async fn hello() -> &'static str {
    "Hello, world!"
}

pub async fn list_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.users.lock().await;
    Json(users.clone())
}

pub async fn get_user(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.lock().await;
    users
        .iter()
        .find(|user| user.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let mut users = state.users.lock().await;

    if users.iter().any(|user| user.id == payload.id) {
        return Err(StatusCode::CONFLICT);
    }

    let user = User {
        id: payload.id,
        name: payload.name,
        email: payload.email,
        role: payload.role,
    };
    users.push(user.clone());
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn delete_user(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let mut users = state.users.lock().await;
    let before = users.len();
    users.retain(|user| user.id != id);
    if users.len() == before {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
