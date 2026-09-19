//! # Oryza-Elo Architecture Guardrail: Device Mapping Repository
//!
//! Persistence operations for hardware sensor mappings and factory presets.

use crate::core::error::AppError;
use crate::domain::models::device_mapping::DeviceMapping;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};

#[derive(Clone)]
pub struct DeviceMappingRepository {
    pool: SqlitePool,
}

impl DeviceMappingRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, m: &DeviceMapping) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO device_mappings (
                id, device_name, manufacturer, is_preset,
                date_col, date_format,
                t_max_col, t_max_unit, t_max_scale,
                t_min_col, t_min_unit, t_min_scale,
                rain_col, rain_unit, rain_scale,
                rad_col, rad_unit, rad_scale,
                rh_col, rh_unit, rh_scale,
                created_at
            ) VALUES (
                ?, ?, ?, ?,
                ?, ?,
                ?, ?, ?,
                ?, ?, ?,
                ?, ?, ?,
                ?, ?, ?,
                ?, ?, ?,
                ?
            )
            ON CONFLICT(id) DO UPDATE SET
                device_name = excluded.device_name,
                manufacturer = excluded.manufacturer,
                is_preset = excluded.is_preset,
                date_col = excluded.date_col,
                date_format = excluded.date_format,
                t_max_col = excluded.t_max_col,
                t_max_unit = excluded.t_max_unit,
                t_max_scale = excluded.t_max_scale,
                t_min_col = excluded.t_min_col,
                t_min_unit = excluded.t_min_unit,
                t_min_scale = excluded.t_min_scale,
                rain_col = excluded.rain_col,
                rain_unit = excluded.rain_unit,
                rain_scale = excluded.rain_scale,
                rad_col = excluded.rad_col,
                rad_unit = excluded.rad_unit,
                rad_scale = excluded.rad_scale,
                rh_col = excluded.rh_col,
                rh_unit = excluded.rh_unit,
                rh_scale = excluded.rh_scale
            "#,
        )
        .bind(&m.id)
        .bind(&m.device_name)
        .bind(&m.manufacturer)
        .bind(if m.is_preset { 1 } else { 0 })
        .bind(&m.date_col)
        .bind(&m.date_format)
        .bind(&m.t_max_col)
        .bind(&m.t_max_unit)
        .bind(m.t_max_scale)
        .bind(&m.t_min_col)
        .bind(&m.t_min_unit)
        .bind(m.t_min_scale)
        .bind(&m.rain_col)
        .bind(&m.rain_unit)
        .bind(m.rain_scale)
        .bind(&m.rad_col)
        .bind(&m.rad_unit)
        .bind(m.rad_scale)
        .bind(&m.rh_col)
        .bind(&m.rh_unit)
        .bind(m.rh_scale)
        .bind(m.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to save device mapping {}: {}", m.id, e)))?;

        Ok(())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<DeviceMapping>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, device_name, manufacturer, is_preset,
                   date_col, date_format,
                   t_max_col, t_max_unit, t_max_scale,
                   t_min_col, t_min_unit, t_min_scale,
                   rain_col, rain_unit, rain_scale,
                   rad_col, rad_unit, rad_scale,
                   rh_col, rh_unit, rh_scale,
                   created_at
            FROM device_mappings WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to fetch mapping {}: {}", id, e)))?;

        row.map(Self::row_to_mapping).transpose()
    }

    pub async fn list_all(&self) -> Result<Vec<DeviceMapping>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, device_name, manufacturer, is_preset,
                   date_col, date_format,
                   t_max_col, t_max_unit, t_max_scale,
                   t_min_col, t_min_unit, t_min_scale,
                   rain_col, rain_unit, rain_scale,
                   rad_col, rad_unit, rad_scale,
                   rh_col, rh_unit, rh_scale,
                   created_at
            FROM device_mappings ORDER BY is_preset DESC, device_name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to list mappings: {}", e)))?;

        rows.into_iter().map(Self::row_to_mapping).collect()
    }

    pub async fn list_presets(&self) -> Result<Vec<DeviceMapping>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, device_name, manufacturer, is_preset,
                   date_col, date_format,
                   t_max_col, t_max_unit, t_max_scale,
                   t_min_col, t_min_unit, t_min_scale,
                   rain_col, rain_unit, rain_scale,
                   rad_col, rad_unit, rad_scale,
                   rh_col, rh_unit, rh_scale,
                   created_at
            FROM device_mappings WHERE is_preset = 1 ORDER BY device_name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to list presets: {}", e)))?;

        rows.into_iter().map(Self::row_to_mapping).collect()
    }

    pub async fn delete(&self, id: &str) -> Result<bool, AppError> {
        // Prevent deletion of factory presets
        let mapping = self.get_by_id(id).await?;
        if let Some(m) = mapping {
            if m.is_preset {
                return Err(AppError::BadRequest(format!(
                    "Cannot delete factory preset device mapping: {}",
                    id
                )));
            }
        } else {
            return Ok(false);
        }

        let rows = sqlx::query("DELETE FROM device_mappings WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(format!("Failed to delete mapping {}: {}", id, e)))?
            .rows_affected();

        Ok(rows > 0)
    }

    fn row_to_mapping(r: sqlx::sqlite::SqliteRow) -> Result<DeviceMapping, AppError> {
        let is_preset_int: i64 = r.get("is_preset");
        let created_str: String = r.get("created_at");
        let created_at = DateTime::parse_from_rfc3339(&created_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(DeviceMapping {
            id: r.get("id"),
            device_name: r.get("device_name"),
            manufacturer: r.get("manufacturer"),
            is_preset: is_preset_int == 1,
            date_col: r.get("date_col"),
            date_format: r.get("date_format"),
            t_max_col: r.get("t_max_col"),
            t_max_unit: r.get("t_max_unit"),
            t_max_scale: r.get("t_max_scale"),
            t_min_col: r.get("t_min_col"),
            t_min_unit: r.get("t_min_unit"),
            t_min_scale: r.get("t_min_scale"),
            rain_col: r.get("rain_col"),
            rain_unit: r.get("rain_unit"),
            rain_scale: r.get("rain_scale"),
            rad_col: r.get("rad_col"),
            rad_unit: r.get("rad_unit"),
            rad_scale: r.get("rad_scale"),
            rh_col: r.get("rh_col"),
            rh_unit: r.get("rh_unit"),
            rh_scale: r.get("rh_scale"),
            created_at,
        })
    }
}
