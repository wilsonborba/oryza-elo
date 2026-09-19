//! # Oryza-Elo Architecture Guardrail: Parcels Handlers

use crate::core::error::AppError;
use crate::domain::models::farm::FarmParcel;
use crate::presentation::api::state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn list_parcels(State(state): State<AppState>) -> Result<Json<Vec<FarmParcel>>, AppError> {
    let parcels = state.parcel_repo.list_all().await?;
    Ok(Json(parcels))
}

pub async fn create_parcel(
    State(state): State<AppState>,
    Json(parcel): Json<FarmParcel>,
) -> Result<(StatusCode, Json<FarmParcel>), AppError> {
    state.parcel_repo.create(&parcel).await?;
    Ok((StatusCode::CREATED, Json(parcel)))
}

pub async fn get_parcel(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<FarmParcel>, AppError> {
    let parcel = state
        .parcel_repo
        .get_by_id(&id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Parcel {} not found", id)))?;

    Ok(Json(parcel))
}

pub async fn update_parcel(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(mut parcel): Json<FarmParcel>,
) -> Result<Json<FarmParcel>, AppError> {
    parcel.id = id;
    state.parcel_repo.update(&parcel).await?;
    Ok(Json(parcel))
}

pub async fn delete_parcel(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let deleted = state.parcel_repo.delete(&id).await?;
    if !deleted {
        return Err(AppError::NotFound(format!("Parcel {} not found", id)));
    }
    Ok(Json(json!({ "deleted": true, "id": id })))
}
