//! # Oryza-Elo Architecture Guardrail: Device Mapping Handlers

use crate::core::error::AppError;
use crate::domain::models::device_mapping::DeviceMapping;
use crate::presentation::api::state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn list_presets(State(state): State<AppState>) -> Result<Json<Vec<DeviceMapping>>, AppError> {
    let presets = state.device_repo.list_presets().await?;
    Ok(Json(presets))
}

pub async fn list_mappings(State(state): State<AppState>) -> Result<Json<Vec<DeviceMapping>>, AppError> {
    let mappings = state.device_repo.list_all().await?;
    Ok(Json(mappings))
}

pub async fn create_mapping(
    State(state): State<AppState>,
    Json(mapping): Json<DeviceMapping>,
) -> Result<(StatusCode, Json<DeviceMapping>), AppError> {
    state.device_repo.save(&mapping).await?;
    Ok((StatusCode::CREATED, Json(mapping)))
}

pub async fn get_mapping(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<DeviceMapping>, AppError> {
    let mapping = state
        .device_repo
        .get_by_id(&id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Device mapping {} not found", id)))?;

    Ok(Json(mapping))
}

pub async fn delete_mapping(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let deleted = state.device_repo.delete(&id).await?;
    if !deleted {
        return Err(AppError::NotFound(format!("Device mapping {} not found", id)));
    }
    Ok(Json(json!({ "deleted": true, "id": id })))
}
