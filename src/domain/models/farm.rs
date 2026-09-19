//! # Oryza-Elo Architecture Guardrail: Farm Parcel Domain Model
//!
//! Representation of agricultural field parcels with geolocation and crop attributes.

use crate::core::error::{DomainError, DomainResult};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Agricultural field parcel representation for rice cultivation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FarmParcel {
    /// Unique parcel identifier (UUID or business key)
    pub id: String,
    /// Farmer-friendly parcel name (e.g. "Talhão 4 - Várzea Sul")
    pub name: String,
    /// Cultivated rice variety (e.g. "ขาวดอกมะลิ 105", "RD43", "IR64")
    pub rice_variety: String,
    /// Rice agro-ecosystem (e.g. "นาชลประทาน", "นาน้ำฝน")
    pub rice_ecosystem: String,
    /// Centroid latitude in decimal degrees (-90.0 to 90.0)
    pub latitude: f64,
    /// Centroid longitude in decimal degrees (-180.0 to 180.0)
    pub longitude: f64,
    /// Sowing or transplanting date
    pub planting_date: NaiveDate,
    /// Parcel area in hectares (optional)
    pub area_hectares: Option<f64>,
}

impl FarmParcel {
    /// Validates all business invariants for the parcel.
    pub fn validate(&self) -> DomainResult<()> {
        if self.id.trim().is_empty() {
            return Err(DomainError::ValidationError("Parcel ID cannot be empty".into()));
        }
        if self.name.trim().is_empty() {
            return Err(DomainError::ValidationError("Parcel name cannot be empty".into()));
        }
        if self.latitude < -90.0 || self.latitude > 90.0 || self.longitude < -180.0 || self.longitude > 180.0 {
            return Err(DomainError::InvalidCoordinate {
                lat: self.latitude,
                lon: self.longitude,
            });
        }
        if let Some(area) = self.area_hectares {
            if area <= 0.0 {
                return Err(DomainError::ValidationError("Parcel area must be positive".into()));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_farm_parcel() {
        let parcel = FarmParcel {
            id: "parcel-01".into(),
            name: "Parcel Alpha".into(),
            rice_variety: "KDML 105".into(),
            rice_ecosystem: "นาชลประทาน".into(),
            latitude: 14.88,
            longitude: 100.45,
            planting_date: NaiveDate::from_ymd_opt(2026, 6, 15).unwrap(),
            area_hectares: Some(5.5),
        };
        assert!(parcel.validate().is_ok());
    }

    #[test]
    fn test_invalid_coordinates() {
        let parcel = FarmParcel {
            id: "parcel-02".into(),
            name: "Invalid Coord".into(),
            rice_variety: "RD43".into(),
            rice_ecosystem: "นาน้ำฝน".into(),
            latitude: 95.0,
            longitude: 100.0,
            planting_date: NaiveDate::from_ymd_opt(2026, 6, 15).unwrap(),
            area_hectares: None,
        };
        assert!(parcel.validate().is_err());
    }
}
