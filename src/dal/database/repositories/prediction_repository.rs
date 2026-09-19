//! # Oryza-Elo Architecture Guardrail: Prediction Repository
//!
//! Persistence operations for phenological stage inference history in SQLite.

use crate::core::error::AppError;
use crate::domain::models::advisory::FarmerAdvisory;
use crate::domain::models::phenology::{PhenologyPrediction, PhenologyStage, TechnicalMetrics};
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};
use std::collections::BTreeMap;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone)]
pub struct PredictionRepository {
    pool: SqlitePool,
}

impl PredictionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, pred: &PhenologyPrediction) -> Result<String, AppError> {
        let id = Uuid::new_v4().to_string();
        let probs_json = serde_json::to_string(&pred.probabilities)?;

        // Consolidate advisory and all_translations into a single payload
        let advisory_envelope = serde_json::json!({
            "advisory": pred.advisory,
            "all_translations": pred.all_translations,
        });
        let advisory_json = serde_json::to_string(&advisory_envelope)?;
        let metrics_json = serde_json::to_string(&pred.technical_metrics)?;

        sqlx::query(
            r#"
            INSERT INTO prediction_history (
                id, parcel_id, evaluated_at, macro_phase, granular_stage,
                confidence, is_transitioning, probabilities_json, advisory_json, metrics_json
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&pred.parcel_id)
        .bind(pred.evaluated_at.to_rfc3339())
        .bind(pred.macro_phase.code())
        .bind(pred.granular_stage.english_name())
        .bind(pred.confidence)
        .bind(if pred.is_transitioning { 1 } else { 0 })
        .bind(&probs_json)
        .bind(&advisory_json)
        .bind(&metrics_json)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to save prediction: {}", e)))?;

        Ok(id)
    }

    pub async fn get_latest_for_parcel(
        &self,
        parcel_id: &str,
    ) -> Result<Option<PhenologyPrediction>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, parcel_id, evaluated_at, macro_phase, granular_stage,
                   confidence, is_transitioning, probabilities_json, advisory_json, metrics_json
            FROM prediction_history
            WHERE parcel_id = ?
            ORDER BY evaluated_at DESC
            LIMIT 1
            "#,
        )
        .bind(parcel_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to fetch latest prediction: {}", e)))?;

        row.map(Self::row_to_prediction).transpose()
    }

    pub async fn list_for_parcel(
        &self,
        parcel_id: &str,
        limit: usize,
    ) -> Result<Vec<PhenologyPrediction>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, parcel_id, evaluated_at, macro_phase, granular_stage,
                   confidence, is_transitioning, probabilities_json, advisory_json, metrics_json
            FROM prediction_history
            WHERE parcel_id = ?
            ORDER BY evaluated_at DESC
            LIMIT ?
            "#,
        )
        .bind(parcel_id)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to list predictions: {}", e)))?;

        rows.into_iter().map(Self::row_to_prediction).collect()
    }

    fn row_to_prediction(r: sqlx::sqlite::SqliteRow) -> Result<PhenologyPrediction, AppError> {
        let eval_str: String = r.get("evaluated_at");
        let evaluated_at = DateTime::parse_from_rfc3339(&eval_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        let stage_str: String = r.get("granular_stage");
        let granular_stage = PhenologyStage::from_str(&stage_str).map_err(AppError::Domain)?;
        let macro_phase = granular_stage.macro_phase();

        let confidence: f64 = r.get("confidence");
        let is_trans_int: i64 = r.get("is_transitioning");
        let is_transitioning = is_trans_int == 1;

        let probs_json: String = r.get("probabilities_json");
        let probabilities: BTreeMap<String, f64> = serde_json::from_str(&probs_json)?;

        let adv_json: String = r.get("advisory_json");
        let adv_val: serde_json::Value = serde_json::from_str(&adv_json)?;
        let advisory: FarmerAdvisory = serde_json::from_value(
            adv_val.get("advisory").cloned().unwrap_or(serde_json::Value::Null),
        )?;
        let all_translations: BTreeMap<String, FarmerAdvisory> = serde_json::from_value(
            adv_val.get("all_translations").cloned().unwrap_or(serde_json::Value::Null),
        )?;

        let metrics_json: String = r.get("metrics_json");
        let technical_metrics: TechnicalMetrics = serde_json::from_str(&metrics_json)?;

        Ok(PhenologyPrediction {
            evaluated_at,
            parcel_id: r.get("parcel_id"),
            macro_phase,
            granular_stage,
            confidence,
            is_transitioning,
            probabilities,
            advisory,
            all_translations,
            technical_metrics,
        })
    }
}
