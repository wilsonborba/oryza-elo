//! # Oryza-Elo Architecture Guardrail: Config Handlers

use crate::core::error::AppError;
use crate::presentation::api::state::AppState;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct UpdateConfigPayload {
    pub configs: BTreeMap<String, String>,
}

pub async fn get_all_config(State(state): State<AppState>) -> Result<Json<BTreeMap<String, String>>, AppError> {
    let configs = state.config_repo.list_all().await?;
    Ok(Json(configs))
}

pub async fn update_config(
    State(state): State<AppState>,
    Json(payload): Json<UpdateConfigPayload>,
) -> Result<Json<BTreeMap<String, String>>, AppError> {
    for (k, v) in payload.configs {
        state.config_repo.set(&k, &v).await?;
    }
    let updated = state.config_repo.list_all().await?;
    Ok(Json(updated))
}
