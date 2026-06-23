use crate::monitor::AppState;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use tower_http::services::ServeDir;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/status", get(get_all_status))
        .route("/api/status/{name}", get(get_site_status))
        .nest_service("/assets", ServeDir::new("static/assets"))
        .fallback_service(ServeDir::new("static"))
        .with_state(state)
}

async fn get_all_status(State(state): State<AppState>) -> impl IntoResponse {
    let map = state.read().await;
    Json(serde_json::json!(*map))
}

async fn get_site_status(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let map = state.read().await;

    match map.get(&name) {
        Some(status) => Ok(Json(serde_json::json!(status))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
