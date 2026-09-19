//! # Oryza-Elo Architecture Guardrail: Phenology Handlers

use crate::core::error::AppError;
use crate::dal::inference::onnx_engine::CategoricalEncoder;
use crate::domain::models::locale::Locale;
use crate::domain::models::phenology::PhenologyPrediction;
use crate::domain::services::agronomic_advisor::AgronomicAdvisor;
use crate::domain::services::biomet_calculator::BiometCalculator;
use crate::presentation::api::state::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::{NaiveDate, Utc};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct PredictRequest {
    pub parcel_id: String,
    pub eval_date: Option<NaiveDate>,
    pub locale: Option<String>,
}

#[derive(Deserialize)]
pub struct LatestQuery {
    pub parcel_id: String,
    pub locale: Option<String>,
}

#[derive(Deserialize)]
pub struct HistoryQuery {
    pub parcel_id: String,
    pub limit: Option<usize>,
}

/// Executes on-demand phenological stage prediction for a parcel.
pub async fn predict_stage(
    State(state): State<AppState>,
    Json(req): Json<PredictRequest>,
) -> Result<(StatusCode, Json<PhenologyPrediction>), AppError> {
    // 1. Fetch parcel
    let parcel = state
        .parcel_repo
        .get_by_id(&req.parcel_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Parcel {} not found", req.parcel_id)))?;

    // 2. Resolve evaluation date (fallback to today)
    let eval_date = req.eval_date.unwrap_or_else(|| Utc::now().date_naive());

    // 3. Resolve target locale (fallback to Portuguese ESALQ standard)
    let target_locale = req
        .locale
        .as_deref()
        .and_then(|l| Locale::from_str(l).ok())
        .unwrap_or(Locale::PtBr);

    // 4. Fetch retrospective weather series up to eval_date (up to 60 days)
    let history = state
        .weather_repo
        .get_retrospective(&parcel.id, eval_date, 60)
        .await?;

    if history.len() < 7 {
        return Err(AppError::Domain(
            crate::core::error::DomainError::InsufficientWeatherData {
                required: 7,
                available: history.len(),
            },
        ));
    }

    // 5. Encode categoricals using domain dictionaries
    let eco_code = CategoricalEncoder::encode_ecosystem(&parcel.rice_ecosystem);
    let var_code = CategoricalEncoder::encode_variety(&parcel.rice_variety);
    let prov_code = CategoricalEncoder::encode_province("Suphan Buri");

    // 6. Compute 44 biometeorological features
    let features = BiometCalculator::compute(
        &history,
        eval_date,
        parcel.latitude,
        parcel.longitude,
        eco_code,
        var_code,
        prov_code,
        None,
    )?;

    // 7. Execute ONNX inference (< 1 ms)
    let inference = state.onnx_engine.predict(&features)?;

    // 8. Generate trilingual advisories and full offline translations
    let stage = inference.predicted_stage;
    let advisory = AgronomicAdvisor::generate_advisory(stage, target_locale);
    let all_translations = AgronomicAdvisor::generate_all_translations(stage);

    let prediction = PhenologyPrediction {
        evaluated_at: Utc::now(),
        parcel_id: parcel.id.clone(),
        macro_phase: stage.macro_phase(),
        granular_stage: stage,
        confidence: inference.confidence,
        is_transitioning: inference.is_transitioning,
        probabilities: inference.probabilities,
        advisory,
        all_translations,
        technical_metrics: inference.technical_metrics,
    };

    // 9. Persist prediction in SQLite history
    state.prediction_repo.save(&prediction).await?;

    Ok((StatusCode::OK, Json(prediction)))
}

/// Retrieves latest prediction consolidated in database for a parcel.
pub async fn get_latest_prediction(
    State(state): State<AppState>,
    Query(query): Query<LatestQuery>,
) -> Result<Json<PhenologyPrediction>, AppError> {
    let mut pred = state
        .prediction_repo
        .get_latest_for_parcel(&query.parcel_id)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "No prediction history found for parcel {}",
                query.parcel_id
            ))
        })?;

    // If a specific locale was requested, adapt active advisory
    if let Some(ref l_str) = query.locale {
        if let Ok(loc) = Locale::from_str(l_str) {
            pred.advisory = AgronomicAdvisor::generate_advisory(pred.granular_stage, loc);
        }
    }

    Ok(Json(pred))
}

/// Retrieves prediction history for a parcel.
pub async fn get_prediction_history(
    State(state): State<AppState>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<Vec<PhenologyPrediction>>, AppError> {
    let limit = query.limit.unwrap_or(30);
    let history = state
        .prediction_repo
        .list_for_parcel(&query.parcel_id, limit)
        .await?;

    Ok(Json(history))
}
