//! # Oryza-Elo Architecture Guardrail: Edge Benchmark Handler
//!
//! Live CPU inference latency benchmark endpoint for edge hardware verification (Raspberry Pi / Jetson / PC).

use crate::core::error::AppError;
use crate::presentation::api::state::AppState;
use axum::extract::{Query, State};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Deserialize)]
pub struct BenchmarkQuery {
    pub iterations: Option<usize>,
}

#[derive(Serialize)]
pub struct LatencyBenchmarkResponse {
    pub hardware_target: String,
    pub iterations: usize,
    pub mean_latency_us: f64,
    pub mean_latency_ms: f64,
    pub min_latency_us: f64,
    pub max_latency_us: f64,
    pub p50_latency_us: f64,
    pub p95_latency_us: f64,
    pub p99_latency_us: f64,
    pub target_ceiling_ms: f64,
    pub speedup_vs_edge_ceiling: f64,
    pub status: String,
    pub timestamp: DateTime<Utc>,
}

use crate::domain::models::weather::BiometFeatures;

fn sample_features() -> BiometFeatures {
    BiometFeatures {
        gdd_cum_7d: 118.45,
        gdd_cum_14d: 239.29,
        gdd_cum_30d: 509.5,
        gdd_cum_60d: 1018.73,
        rain_cum_7d: 9.57,
        rain_cum_14d: 10.25,
        rain_cum_30d: 73.66,
        rain_cum_60d: 287.31,
        rain_max_7d: 3.3,
        rain_max_14d: 3.3,
        rain_max_30d: 17.0,
        rain_max_60d: 48.68,
        cdd_7d: 4.0,
        cdd_14d: 11.0,
        cdd_30d: 18.0,
        cdd_60d: 30.0,
        dtr_mean_7d: 1.29,
        dtr_mean_14d: 1.47,
        dtr_mean_30d: 1.43,
        dtr_mean_60d: 1.46,
        dtr_std_7d: 0.36,
        dtr_std_14d: 0.33,
        dtr_std_30d: 0.32,
        dtr_std_60d: 0.35,
        rad_cum_7d: 130.0,
        rad_cum_14d: 275.83,
        rad_cum_30d: 524.22,
        rad_cum_60d: 998.92,
        rh_mean_7d: 82.83,
        rh_mean_14d: 81.83,
        rh_mean_30d: 81.73,
        rh_mean_60d: 80.81,
        month: 2.0,
        day_of_year: 55.0,
        photoperiod_hours: 11.95,
        ptq_30d: 1.03,
        ptq_60d: 0.98,
        vpd_proxy_14d: 0.27,
        vpd_proxy_30d: 0.26,
        lat_clean: 7.82,
        lon_clean: 100.26,
        rice_ecosystem_code: 4.0,
        rice_variety_code: 55.0,
        province_code: 37.0,
    }
}

/// Runs a live CPU inference latency micro-benchmark on the edge engine.
pub async fn run_latency_benchmark(
    State(state): State<AppState>,
    Query(query): Query<BenchmarkQuery>,
) -> Result<Json<LatencyBenchmarkResponse>, AppError> {
    let iterations = query.iterations.unwrap_or(100).clamp(10, 1000);
    let sample = sample_features();

    // Warm-up runs (5 iterations)
    for _ in 0..5 {
        let _ = state.onnx_engine.predict(&sample)?;
    }

    let mut latencies_us: Vec<f64> = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let t0 = Instant::now();
        let _ = state.onnx_engine.predict(&sample)?;
        let elapsed = t0.elapsed().as_nanos() as f64 / 1_000.0;
        latencies_us.push(elapsed);
    }


    latencies_us.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let sum: f64 = latencies_us.iter().sum();
    let mean_us = sum / iterations as f64;
    let mean_ms = mean_us / 1000.0;
    let min_us = latencies_us[0];
    let max_us = latencies_us[iterations - 1];
    let p50_us = latencies_us[(iterations as f64 * 0.50) as usize];
    let p95_us = latencies_us[((iterations as f64 * 0.95) as usize).min(iterations - 1)];
    let p99_us = latencies_us[((iterations as f64 * 0.99) as usize).min(iterations - 1)];

    let target_ceiling_ms = 5.0;
    let speedup = (target_ceiling_ms * 1000.0) / mean_us;
    let status = if mean_ms < target_ceiling_ms {
        "passed".to_string()
    } else {
        "degraded".to_string()
    };

    Ok(Json(LatencyBenchmarkResponse {
        hardware_target: std::env::consts::ARCH.to_string(),
        iterations,
        mean_latency_us: (mean_us * 100.0).round() / 100.0,
        mean_latency_ms: (mean_ms * 10000.0).round() / 10000.0,
        min_latency_us: (min_us * 100.0).round() / 100.0,
        max_latency_us: (max_us * 100.0).round() / 100.0,
        p50_latency_us: (p50_us * 100.0).round() / 100.0,
        p95_latency_us: (p95_us * 100.0).round() / 100.0,
        p99_latency_us: (p99_us * 100.0).round() / 100.0,
        target_ceiling_ms,
        speedup_vs_edge_ceiling: (speedup * 10.0).round() / 10.0,
        status,
        timestamp: Utc::now(),
    }))
}
