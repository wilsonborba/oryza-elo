//! # Oryza-Elo Architecture Guardrail: Health Handler

use crate::presentation::api::state::AppState;
use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
    let sqlite_status = match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    Json(json!({
        "status": "healthy",
        "app_name": state.settings.app_name,
        "version": env!("CARGO_PKG_VERSION"),
        "sqlite": sqlite_status,
        "onnx_engine": "loaded",
        "edge_mode": "autonomous-air-gapped"
    }))
}
