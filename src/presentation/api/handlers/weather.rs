//! # Oryza-Elo Architecture Guardrail: Weather Handlers

use crate::core::error::AppError;
use crate::domain::models::weather::DailyWeatherRecord;
use crate::domain::services::csv_ingestion::{CsvIngestionService, IngestionReport};
use crate::presentation::api::state::AppState;
use axum::extract::{Multipart, Query, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct IngestCsvPayload {
    pub parcel_id: String,
    pub device_id: String,
    pub csv_content: String,
}

#[derive(Deserialize)]
pub struct WeatherQuery {
    pub parcel_id: String,
    pub days: Option<usize>,
    pub up_to_date: Option<NaiveDate>,
}

#[derive(Serialize)]
pub struct UploadCsvResponse {
    pub message: String,
    pub report: IngestionReport,
    pub records_persisted: usize,
}

/// Ingests CSV sent as JSON payload.
pub async fn ingest_csv_json(
    State(state): State<AppState>,
    Json(payload): Json<IngestCsvPayload>,
) -> Result<(StatusCode, Json<UploadCsvResponse>), AppError> {
    process_csv_ingestion(&state, &payload.parcel_id, &payload.device_id, &payload.csv_content).await
}

/// Ingests CSV uploaded via HTTP Multipart Form (`multipart/form-data`).
pub async fn upload_csv_multipart(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<UploadCsvResponse>), AppError> {
    let mut parcel_id: Option<String> = None;
    let mut device_id: Option<String> = None;
    let mut csv_content: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to read multipart field: {}", e)))?
    {
        let name = field.name().unwrap_or_default().to_string();
        match name.as_str() {
            "parcel_id" => {
                parcel_id = Some(field.text().await.map_err(|e| AppError::BadRequest(e.to_string()))?);
            }
            "device_id" => {
                device_id = Some(field.text().await.map_err(|e| AppError::BadRequest(e.to_string()))?);
            }
            "file" | "csv" => {
                csv_content = Some(field.text().await.map_err(|e| AppError::BadRequest(e.to_string()))?);
            }
            _ => {}
        }
    }

    let p_id = parcel_id.ok_or_else(|| AppError::BadRequest("Missing 'parcel_id' in form data".into()))?;
    let d_id = device_id.ok_or_else(|| AppError::BadRequest("Missing 'device_id' in form data".into()))?;
    let content = csv_content.ok_or_else(|| AppError::BadRequest("Missing 'file' in form data".into()))?;

    process_csv_ingestion(&state, &p_id, &d_id, &content).await
}

async fn process_csv_ingestion(
    state: &AppState,
    parcel_id: &str,
    device_id: &str,
    csv_content: &str,
) -> Result<(StatusCode, Json<UploadCsvResponse>), AppError> {
    // 1. Verify parcel exists
    let _ = state
        .parcel_repo
        .get_by_id(parcel_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Parcel {} not found", parcel_id)))?;

    // 2. Fetch device mapping
    let mapping = state
        .device_repo
        .get_by_id(device_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Device mapping {} not found", device_id)))?;

    // 3. Parse and convert units deterministically
    let report = CsvIngestionService::parse_csv(csv_content, &mapping)?;

    // 4. Batch persist valid records
    let records_persisted = if !report.records.is_empty() {
        state
            .weather_repo
            .insert_batch(parcel_id, &report.records)
            .await?
    } else {
        0
    };

    Ok((
        StatusCode::OK,
        Json(UploadCsvResponse {
            message: format!(
                "Successfully processed {} rows ({} persisted, {} rejected)",
                report.total_rows, records_persisted, report.failed_rows
            ),
            report,
            records_persisted,
        }),
    ))
}

/// Retrieves retrospective weather records for a parcel.
pub async fn get_records(
    State(state): State<AppState>,
    Query(query): Query<WeatherQuery>,
) -> Result<Json<Vec<DailyWeatherRecord>>, AppError> {
    let up_to_date = query.up_to_date.unwrap_or_else(|| chrono::Utc::now().date_naive());
    let days = query.days.unwrap_or(60);

    let records = state
        .weather_repo
        .get_retrospective(&query.parcel_id, up_to_date, days)
        .await?;

    Ok(Json(records))
}
