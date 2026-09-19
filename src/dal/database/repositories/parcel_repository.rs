//! # Oryza-Elo Architecture Guardrail: Parcel Repository
//!
//! Persistence operations for agricultural field parcels in SQLite.

use crate::core::error::AppError;
use crate::domain::models::farm::FarmParcel;
use chrono::{NaiveDate, Utc};
use sqlx::{Row, SqlitePool};

#[derive(Clone)]
pub struct ParcelRepository {
    pool: SqlitePool,
}

impl ParcelRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, parcel: &FarmParcel) -> Result<(), AppError> {
        parcel.validate().map_err(AppError::Domain)?;

        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"
            INSERT INTO parcels (id, name, rice_variety, rice_ecosystem, latitude, longitude, planting_date, area_hectares, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&parcel.id)
        .bind(&parcel.name)
        .bind(&parcel.rice_variety)
        .bind(&parcel.rice_ecosystem)
        .bind(parcel.latitude)
        .bind(parcel.longitude)
        .bind(parcel.planting_date.to_string())
        .bind(parcel.area_hectares)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to insert parcel: {}", e)))?;

        Ok(())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<FarmParcel>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, rice_variety, rice_ecosystem, latitude, longitude, planting_date, area_hectares
            FROM parcels WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to fetch parcel {}: {}", id, e)))?;

        match row {
            Some(r) => {
                let p_date_str: String = r.get("planting_date");
                let planting_date = NaiveDate::parse_from_str(&p_date_str, "%Y-%m-%d")
                    .map_err(|e| AppError::Database(format!("Invalid date in db: {}", e)))?;

                Ok(Some(FarmParcel {
                    id: r.get("id"),
                    name: r.get("name"),
                    rice_variety: r.get("rice_variety"),
                    rice_ecosystem: r.get("rice_ecosystem"),
                    latitude: r.get("latitude"),
                    longitude: r.get("longitude"),
                    planting_date,
                    area_hectares: r.get("area_hectares"),
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn list_all(&self) -> Result<Vec<FarmParcel>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, rice_variety, rice_ecosystem, latitude, longitude, planting_date, area_hectares
            FROM parcels ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to list parcels: {}", e)))?;

        let mut parcels = Vec::with_capacity(rows.len());
        for r in rows {
            let p_date_str: String = r.get("planting_date");
            let planting_date = NaiveDate::parse_from_str(&p_date_str, "%Y-%m-%d")
                .map_err(|e| AppError::Database(format!("Invalid date in db: {}", e)))?;

            parcels.push(FarmParcel {
                id: r.get("id"),
                name: r.get("name"),
                rice_variety: r.get("rice_variety"),
                rice_ecosystem: r.get("rice_ecosystem"),
                latitude: r.get("latitude"),
                longitude: r.get("longitude"),
                planting_date,
                area_hectares: r.get("area_hectares"),
            });
        }

        Ok(parcels)
    }

    pub async fn update(&self, parcel: &FarmParcel) -> Result<(), AppError> {
        parcel.validate().map_err(AppError::Domain)?;

        let now = Utc::now().to_rfc3339();
        let rows_affected = sqlx::query(
            r#"
            UPDATE parcels SET
                name = ?, rice_variety = ?, rice_ecosystem = ?,
                latitude = ?, longitude = ?, planting_date = ?,
                area_hectares = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&parcel.name)
        .bind(&parcel.rice_variety)
        .bind(&parcel.rice_ecosystem)
        .bind(parcel.latitude)
        .bind(parcel.longitude)
        .bind(parcel.planting_date.to_string())
        .bind(parcel.area_hectares)
        .bind(&now)
        .bind(&parcel.id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to update parcel {}: {}", parcel.id, e)))?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Parcel {} not found", parcel.id)));
        }

        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<bool, AppError> {
        let rows = sqlx::query("DELETE FROM parcels WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(format!("Failed to delete parcel {}: {}", id, e)))?
            .rows_affected();

        Ok(rows > 0)
    }
}
