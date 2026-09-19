//! # Oryza-Elo Architecture Guardrail: ONNX Inference Engine
//!
//! High-performance edge inference engine using ONNX Runtime via `ort` crate.
//! Features singleton in-memory session, zero-copy tensors, and latency < 1 ms on CPU.

use crate::core::error::AppError;
use crate::domain::models::phenology::{PhenologyStage, TechnicalMetrics};
use crate::domain::models::weather::BiometFeatures;
use ndarray::Array2;
use ort::inputs;
use ort::session::builder::GraphOptimizationLevel;
use ort::session::Session;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

pub const DEFAULT_ONNX_MODEL_PATH: &str =
    "src/dal/data/processed/models/rice_stage_classifier.onnx";
pub const MODEL_METADATA_JSON: &str =
    include_str!("../data/processed/models/model_metadata.json");

/// Output prediction from the ONNX runtime model.
#[derive(Debug, Clone)]
pub struct InferenceResult {
    pub predicted_stage: PhenologyStage,
    pub confidence: f64,
    pub is_transitioning: bool,
    pub probabilities: BTreeMap<String, f64>,
    pub technical_metrics: TechnicalMetrics,
}

#[derive(Deserialize)]
struct ModelMetadata {
    categorical_encodings: CategoricalEncodings,
}

#[derive(Deserialize)]
struct CategoricalEncodings {
    rice_ecosystem: HashMap<String, f64>,
    rice_variety: HashMap<String, f64>,
    province: HashMap<String, f64>,
}

static METADATA: OnceLock<ModelMetadata> = OnceLock::new();

fn get_metadata() -> &'static ModelMetadata {
    METADATA.get_or_init(|| {
        serde_json::from_str(MODEL_METADATA_JSON).expect("Failed to parse model_metadata.json")
    })
}

/// Helper for encoding categorical variables using exact training dictionary.
pub struct CategoricalEncoder;

impl CategoricalEncoder {
    pub fn encode_ecosystem(name: &str) -> f64 {
        let meta = get_metadata();
        let trimmed = name.trim();
        if let Some(&code) = meta.categorical_encodings.rice_ecosystem.get(trimmed) {
            return code;
        }
        // Fallback for common aliases
        match trimmed.to_lowercase().as_str() {
            "irrigated" | "irrigado" | "na_chon" => 4.0, // นาชลประทาน
            "rainfed" | "sequeiro" | "na_nam_fon" => 5.0, // นาน้ำฝน
            _ => 4.0,                                    // Default irrigated
        }
    }

    pub fn encode_variety(name: &str) -> f64 {
        let meta = get_metadata();
        let trimmed = name.trim();
        if let Some(&code) = meta.categorical_encodings.rice_variety.get(trimmed) {
            return code;
        }
        // Match partials for Jasmine / KDML 105
        if trimmed.contains("105") || trimmed.to_lowercase().contains("jasmine") || trimmed.to_lowercase().contains("kdml") {
            return 73.0; // ขาวดอกมะลิ 105
        }
        // Match RD varieties
        if trimmed.to_lowercase().contains("rd43") || trimmed.contains("กข43") {
            return 39.0;
        }
        if trimmed.to_lowercase().contains("rd79") || trimmed.contains("กข79") {
            return 55.0;
        }

        0.0 // Default "ไม่ทราบพันธุ์" (Unknown)
    }

    pub fn encode_province(name: &str) -> f64 {
        let meta = get_metadata();
        let trimmed = name.trim();
        if let Some(&code) = meta.categorical_encodings.province.get(trimmed) {
            return code;
        }
        // Match Latin/English province names
        match trimmed.to_lowercase().as_str() {
            "suphan buri" | "suphanburi" => 40.0,
            "songkhla" => 37.0,
            "nakhon ratchasima" | "korat" => 13.0,
            "chiang mai" | "chiangmai" => 53.0,
            "khon kaen" | "khonkaen" => 4.0,
            "ubon ratchathani" => 50.0,
            _ => 40.0, // Default Suphan Buri (major rice central basin)
        }
    }
}

/// Thread-safe singleton ONNX Inference Engine wrapping the CatBoost classifier.
pub struct OnnxInferenceEngine {
    session: Mutex<Session>,
    model_version: String,
}

impl OnnxInferenceEngine {
    /// Loads the ONNX model from the provided path and creates an optimized session.
    pub fn new<P: AsRef<Path>>(model_path: P) -> Result<Self, AppError> {
        let session = Session::builder()
            .map_err(|e| AppError::Inference(e.to_string()))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| AppError::Inference(e.to_string()))?
            .with_intra_threads(1)
            .map_err(|e| AppError::Inference(e.to_string()))?
            .commit_from_file(&model_path)
            .map_err(|e| {
                AppError::Inference(format!(
                    "Failed to load ONNX model from {:?}: {}",
                    model_path.as_ref(),
                    e
                ))
            })?;

        Ok(Self {
            session: Mutex::new(session),
            model_version: "catboost_onnx_v1".into(),
        })
    }

    /// Initializes from default relative path `src/dal/data/processed/models/rice_stage_classifier.onnx`.
    pub fn init_default() -> Result<Self, AppError> {
        let path = PathBuf::from(DEFAULT_ONNX_MODEL_PATH);
        Self::new(path)
    }

    /// Executes inference on a 44-feature biometeorological input vector, returning probabilities and metrics.
    pub fn predict(&self, features: &BiometFeatures) -> Result<InferenceResult, AppError> {
        let t_start = Instant::now();

        // 1. Build [1, 44] Tensor strictly in features_order sequence
        let tensor_vec = features.to_tensor_vec();
        let array = Array2::from_shape_vec((1, 44), tensor_vec)
            .map_err(|e| AppError::Inference(e.to_string()))?;

        let input_tensor = ort::value::Tensor::from_array(array)
            .map_err(|e| AppError::Inference(e.to_string()))?;

        // 2. Run Session Inference with Lock
        let mut session = self
            .session
            .lock()
            .map_err(|e| AppError::Inference(format!("Lock poisoned: {}", e)))?;

        let outputs = session
            .run(inputs!["features" => input_tensor])
            .map_err(|e| AppError::Inference(format!("ONNX runtime execution error: {}", e)))?;

        let latency_ms = t_start.elapsed().as_secs_f64() * 1000.0;

        // 3. Extract Predicted Winner Label (Index 0..6)
        let label_tensor = outputs["label"]
            .try_extract_tensor::<i64>()
            .map_err(|e| AppError::Inference(format!("Failed to extract label: {}", e)))?;
        let predicted_idx = label_tensor.1[0] as usize;
        let predicted_stage = PhenologyStage::from_model_index(predicted_idx)
            .map_err(AppError::Domain)?;

        // 4. Extract Probability Map for all 7 Classes
        let seq = outputs["probabilities"]
            .try_extract_sequence::<ort::value::DynMapValueType>()
            .map_err(|e| AppError::Inference(format!("Failed to extract probabilities: {}", e)))?;

        let mut probabilities_map = BTreeMap::new();
        let mut prob_pairs: Vec<(PhenologyStage, f64)> = Vec::with_capacity(7);

        if !seq.is_empty() {
            let kvs = seq[0]
                .try_extract_key_values::<i64, f32>()
                .map_err(|e| AppError::Inference(format!("Failed to extract key-values: {}", e)))?;

            for (k, v) in kvs {
                if let Ok(stage) = PhenologyStage::from_model_index(k as usize) {
                    let prob = (v as f64).max(0.0);
                    probabilities_map.insert(stage.english_name().to_string(), prob);
                    prob_pairs.push((stage, prob));
                }
            }
        }
        drop(outputs);
        drop(session);

        // Sort probabilities descending
        prob_pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let confidence = prob_pairs.first().map(|p| p.1).unwrap_or(1.0);
        let second_confidence = prob_pairs.get(1).map(|p| p.1).unwrap_or(0.0);
        let margin = confidence - second_confidence;

        // Shannon Entropy: H = -sum(p * log2(p + 1e-12))
        let entropy: f64 = prob_pairs
            .iter()
            .map(|(_, p)| {
                if *p > 1e-12 {
                    -*p * (p + 1e-12).log2()
                } else {
                    0.0
                }
            })
            .sum();

        // Active Phenological Transition Detection:
        // Transition if margin between top 2 classes is small (< 0.18) OR normalized entropy > 0.70
        let norm_entropy = entropy / (7.0_f64).log2();
        let is_transitioning = margin < 0.18 || norm_entropy > 0.70;

        Ok(InferenceResult {
            predicted_stage,
            confidence: (confidence * 10000.0).round() / 10000.0,
            is_transitioning,
            probabilities: probabilities_map,
            technical_metrics: TechnicalMetrics {
                model_version: self.model_version.clone(),
                inference_latency_ms: (latency_ms * 10000.0).round() / 10000.0,
                entropy: (entropy * 10000.0).round() / 10000.0,
                margin: (margin * 10000.0).round() / 10000.0,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_features() -> BiometFeatures {
        BiometFeatures {
            gdd_cum_7d: 118.45, gdd_cum_14d: 239.29, gdd_cum_30d: 509.5, gdd_cum_60d: 1018.73,
            rain_cum_7d: 9.57, rain_cum_14d: 10.25, rain_cum_30d: 73.66, rain_cum_60d: 287.31,
            rain_max_7d: 3.3, rain_max_14d: 3.3, rain_max_30d: 17.0, rain_max_60d: 48.68,
            cdd_7d: 4.0, cdd_14d: 11.0, cdd_30d: 18.0, cdd_60d: 30.0,
            dtr_mean_7d: 1.29, dtr_mean_14d: 1.47, dtr_mean_30d: 1.43, dtr_mean_60d: 1.46,
            dtr_std_7d: 0.36, dtr_std_14d: 0.33, dtr_std_30d: 0.32, dtr_std_60d: 0.35,
            rad_cum_7d: 130.0, rad_cum_14d: 275.83, rad_cum_30d: 524.22, rad_cum_60d: 998.92,
            rh_mean_7d: 82.83, rh_mean_14d: 81.83, rh_mean_30d: 81.73, rh_mean_60d: 80.81,
            month: 2.0, day_of_year: 55.0, photoperiod_hours: 11.95,
            ptq_30d: 1.03, ptq_60d: 0.98, vpd_proxy_14d: 0.27, vpd_proxy_30d: 0.26,
            lat_clean: 7.82, lon_clean: 100.26,
            rice_ecosystem_code: 4.0, rice_variety_code: 55.0, province_code: 37.0,
        }
    }

    #[test]
    fn test_predict_real_sample() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let model_path = repo_root.join(DEFAULT_ONNX_MODEL_PATH);
        let engine = OnnxInferenceEngine::new(model_path).unwrap();

        let feat = dummy_features();
        let result = engine.predict(&feat).unwrap();

        println!("Predicted Stage: {:?}", result.predicted_stage);
        println!("Confidence: {:.4}", result.confidence);
        println!("Latency ms: {:.4}", result.technical_metrics.inference_latency_ms);
        println!("Probabilities: {:?}", result.probabilities);

        assert_eq!(result.probabilities.len(), 7);
        assert!(result.confidence > 0.0 && result.confidence <= 1.0);
        assert!(result.technical_metrics.inference_latency_ms < 5.0, "Latency must be < 5 ms");
    }

    #[test]
    fn test_categorical_encoders() {
        assert_eq!(CategoricalEncoder::encode_ecosystem("นาชลประทาน"), 4.0);
        assert_eq!(CategoricalEncoder::encode_ecosystem("irrigated"), 4.0);
        assert_eq!(CategoricalEncoder::encode_variety("ขาวดอกมะลิ 105"), 73.0);
        assert_eq!(CategoricalEncoder::encode_variety("KDML 105"), 73.0);
        assert_eq!(CategoricalEncoder::encode_province("สงขลา"), 37.0);
        assert_eq!(CategoricalEncoder::encode_province("songkhla"), 37.0);
    }
}
