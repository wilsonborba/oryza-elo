//! # Oryza-Elo Architecture Guardrail: Nightly Cron Scheduler
//!
//! Autonomous background scheduler evaluating phenological progression for all parcels daily.

use crate::core::error::AppError;
use crate::dal::database::repositories::{ParcelRepository, PredictionRepository, WeatherRepository};
use crate::dal::inference::onnx_engine::{CategoricalEncoder, OnnxInferenceEngine};
use crate::domain::models::locale::Locale;
use crate::domain::models::phenology::PhenologyPrediction;
use crate::domain::services::agronomic_advisor::AgronomicAdvisor;
use crate::domain::services::biomet_calculator::BiometCalculator;
use chrono::{Local, NaiveDate, Timelike, Utc};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

pub struct CronScheduler;

impl CronScheduler {
    /// Spawns the background scheduler thread inside the Tokio runtime.
    pub fn spawn(
        parcel_repo: ParcelRepository,
        weather_repo: WeatherRepository,
        prediction_repo: PredictionRepository,
        onnx_engine: Arc<OnnxInferenceEngine>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            info!("Iniciando Nightly Cron Scheduler de borda (Alvo diário: 23:59)...");
            let mut last_executed_date: Option<NaiveDate> = None;

            loop {
                // Check every 30 seconds
                tokio::time::sleep(Duration::from_secs(30)).await;

                let now_local = Local::now();
                let current_date = now_local.date_naive();
                let hour = now_local.hour();
                let minute = now_local.minute();

                // Trigger at 23:59 if not already executed today
                let should_run = (hour == 23 && minute >= 59)
                    && (last_executed_date != Some(current_date));

                if should_run {
                    info!(
                        date = %current_date,
                        "Disparando rotina noturna de avaliação fenológica autônoma (23:59)..."
                    );

                    match Self::execute_evaluation_for_all_parcels(
                        &parcel_repo,
                        &weather_repo,
                        &prediction_repo,
                        &onnx_engine,
                        current_date,
                    )
                    .await
                    {
                        Ok(count) => {
                            info!(
                                parcels_evaluated = count,
                                date = %current_date,
                                "Avaliação fenológica noturna concluída com sucesso para todos os talhões!"
                            );
                            last_executed_date = Some(current_date);
                        }
                        Err(e) => {
                            error!(
                                error = %e,
                                date = %current_date,
                                "Erro durante execução da rotina noturna fenológica"
                            );
                        }
                    }
                }
            }
        })
    }

    /// Evaluates all registered farm parcels for a specific observation date.
    pub async fn execute_evaluation_for_all_parcels(
        parcel_repo: &ParcelRepository,
        weather_repo: &WeatherRepository,
        prediction_repo: &PredictionRepository,
        onnx_engine: &OnnxInferenceEngine,
        eval_date: NaiveDate,
    ) -> Result<usize, AppError> {
        let parcels = parcel_repo.list_all().await?;
        let mut processed = 0;

        for p in parcels {
            let history = weather_repo.get_retrospective(&p.id, eval_date, 60).await?;
            if history.len() < 7 {
                warn!(
                    parcel_id = %p.id,
                    available_days = history.len(),
                    "Talhão com histórico meteorológico insuficiente (< 7 dias). Pulando avaliação."
                );
                continue;
            }

            let eco_code = CategoricalEncoder::encode_ecosystem(&p.rice_ecosystem);
            let var_code = CategoricalEncoder::encode_variety(&p.rice_variety);
            let prov_code = CategoricalEncoder::encode_province("Suphan Buri");

            let features = match BiometCalculator::compute(
                &history,
                eval_date,
                p.latitude,
                p.longitude,
                eco_code,
                var_code,
                prov_code,
                None,
            ) {
                Ok(f) => f,
                Err(e) => {
                    warn!(parcel_id = %p.id, error = %e, "Falha no cálculo de features biometeorológicas");
                    continue;
                }
            };

            let inference = match onnx_engine.predict(&features) {
                Ok(inf) => inf,
                Err(e) => {
                    warn!(parcel_id = %p.id, error = %e, "Falha na inferência do modelo ONNX");
                    continue;
                }
            };

            let stage = inference.predicted_stage;
            let advisory = AgronomicAdvisor::generate_advisory(stage, Locale::PtBr);
            let all_translations = AgronomicAdvisor::generate_all_translations(stage);

            let prediction = PhenologyPrediction {
                evaluated_at: Utc::now(),
                parcel_id: p.id.clone(),
                macro_phase: stage.macro_phase(),
                granular_stage: stage,
                confidence: inference.confidence,
                is_transitioning: inference.is_transitioning,
                probabilities: inference.probabilities,
                advisory,
                all_translations,
                technical_metrics: inference.technical_metrics,
            };

            prediction_repo.save(&prediction).await?;
            processed += 1;
        }

        Ok(processed)
    }
}
