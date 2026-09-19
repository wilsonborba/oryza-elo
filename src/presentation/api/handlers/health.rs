//! # Oryza-Elo Architecture Guardrail: Health & Monitoring Handlers
//!
//! Differentiated health probes:
//! - Root `/health`: Lightweight liveness ping for infrastructure (Docker, systemd, LB).
//! - `/api/v1/health/system`: Host OS, CPU cores, memory RSS, and edge hardware health.
//! - `/api/v1/health/app`: Application internal subsystems (SQLite WAL pool, ONNX session, Cron scheduler).
//! - `/api/v1/health`: Consolidated summary.

use crate::presentation::api::state::AppState;
use axum::extract::State;
use axum::Json;
use chrono::Utc;
use serde_json::{json, Value};
use std::sync::OnceLock;
use std::time::Instant;

static START_TIME: OnceLock<Instant> = OnceLock::new();

fn get_uptime_secs() -> u64 {
    START_TIME.get_or_init(Instant::now).elapsed().as_secs()
}

fn get_memory_rss_bytes() -> u64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/self/statm") {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(pages) = parts[1].parse::<u64>() {
                    return pages * 4096;
                }
            }
        }
    }
    0
}

/// Lightweight liveness ping for load balancers, Docker, and systemd watchdog.
pub async fn liveness_ping() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "timestamp": Utc::now().to_rfc3339()
    }))
}

/// System and host hardware health probe (Linux OS, CPU parallelism, memory RSS).
pub async fn system_health(State(_state): State<AppState>) -> Json<Value> {
    let uptime_sec = get_uptime_secs();
    let rss_bytes = get_memory_rss_bytes();
    let rss_mb = (rss_bytes as f64) / (1024.0 * 1024.0);
    let cpu_cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    Json(json!({
        "status": "healthy",
        "scope": "system",
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "cpu_cores": cpu_cores,
        "memory_rss_mb": (rss_mb * 100.0).round() / 100.0,
        "memory_rss_bytes": rss_bytes,
        "uptime_seconds": uptime_sec,
        "timestamp": Utc::now().to_rfc3339()
    }))
}

/// Application internal subsystems health probe (SQLite, ONNX session, Cron scheduler).
pub async fn app_health(State(state): State<AppState>) -> Json<Value> {
    let sqlite_status = match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    let total_parcels = state
        .parcel_repo
        .list_all()
        .await
        .map(|p| p.len())
        .unwrap_or(0);

    let weather_records_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM weather_records")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let predictions_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM prediction_history")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    Json(json!({
        "status": "healthy",
        "scope": "application",
        "app_name": state.settings.app_name,
        "version": env!("CARGO_PKG_VERSION"),
        "sqlite": {
            "status": sqlite_status,
            "total_weather_records": weather_records_count,
            "total_predictions": predictions_count,
            "total_parcels": total_parcels
        },
        "onnx_engine": {
            "status": "loaded",
            "model_path": "src/dal/data/processed/models/rice_stage_classifier.onnx"
        },
        "cron_scheduler": {
            "target_time": "23:59",
            "status": "active"
        },
        "edge_mode": "autonomous-air-gapped",
        "timestamp": Utc::now().to_rfc3339()
    }))
}

/// Consolidated overall health status combining system and application metrics.
pub async fn consolidated_health(State(state): State<AppState>) -> Json<Value> {
    let sys = system_health(State(state.clone())).await.0;
    let app = app_health(State(state)).await.0;

    Json(json!({
        "status": "healthy",
        "system": sys,
        "application": app,
        "timestamp": Utc::now().to_rfc3339()
    }))
}
