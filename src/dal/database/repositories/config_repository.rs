//! # Oryza-Elo Architecture Guardrail: Config Repository
//!
//! Persistence operations for local edge node key-value configurations.

use crate::core::error::AppError;
use chrono::Utc;
use sqlx::{Row, SqlitePool};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct ConfigRepository {
    pool: SqlitePool,
}

impl ConfigRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        let row = sqlx::query("SELECT value FROM edge_config WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(format!("Failed to read config key {}: {}", key, e)))?;

        Ok(row.map(|r| r.get("value")))
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"
            INSERT INTO edge_config (key, value, updated_at)
            VALUES (?, ?, ?)
            ON CONFLICT(key) DO UPDATE SET
                value = excluded.value,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(key)
        .bind(value)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to set config key {}: {}", key, e)))?;

        Ok(())
    }

    pub async fn list_all(&self) -> Result<BTreeMap<String, String>, AppError> {
        let rows = sqlx::query("SELECT key, value FROM edge_config ORDER BY key ASC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(format!("Failed to list configs: {}", e)))?;

        let mut map = BTreeMap::new();
        for r in rows {
            map.insert(r.get("key"), r.get("value"));
        }

        Ok(map)
    }
}
