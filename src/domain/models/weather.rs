//! # Oryza-Elo Architecture Guardrail: Weather & Biomet Models
//!
//! Daily agrometeorological observations and engineered biometeorological feature vectors.

use crate::core::error::{DomainError, DomainResult};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Daily agrometeorological ground record from sensors, stations or remote caches.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyWeatherRecord {
    /// Observation date
    pub date: NaiveDate,
    /// Maximum 2-meter air temperature (°C)
    pub t_max: f64,
    /// Minimum 2-meter air temperature (°C)
    pub t_min: f64,
    /// Daily cumulative precipitation (mm)
    pub precipitation_mm: f64,
    /// Daily surface all-sky solar radiation (MJ/m²/day)
    pub radiation_mj_m2: f64,
    /// Mean 2-meter relative humidity (%)
    pub relative_humidity_pct: f64,
    /// Ingestion source (e.g., "Pessl_iMetos", "Davis_VantagePro2", "NASA_POWER", "Dragino_RS485")
    pub source: String,
}

impl DailyWeatherRecord {
    /// Validates physical laws and domain invariants.
    pub fn validate(&self) -> DomainResult<()> {
        if self.t_min > self.t_max {
            return Err(DomainError::InvalidWeatherRecord {
                message: format!(
                    "T_min ({:.2}°C) cannot exceed T_max ({:.2}°C) on date {}",
                    self.t_min, self.t_max, self.date
                ),
            });
        }
        if self.precipitation_mm < 0.0 {
            return Err(DomainError::InvalidWeatherRecord {
                message: format!(
                    "Precipitation ({:.2} mm) cannot be negative on date {}",
                    self.precipitation_mm, self.date
                ),
            });
        }
        if self.radiation_mj_m2 < 0.0 {
            return Err(DomainError::InvalidWeatherRecord {
                message: format!(
                    "Solar radiation ({:.2} MJ/m²) cannot be negative on date {}",
                    self.radiation_mj_m2, self.date
                ),
            });
        }
        if !(0.0..=100.0).contains(&self.relative_humidity_pct) {
            return Err(DomainError::InvalidWeatherRecord {
                message: format!(
                    "Relative humidity ({:.1}%) must be within [0.0, 100.0] on date {}",
                    self.relative_humidity_pct, self.date
                ),
            });
        }
        if self.t_min < -40.0 || self.t_max > 60.0 {
            return Err(DomainError::InvalidWeatherRecord {
                message: format!(
                    "Extreme temperature outlier detected: T_min={:.2}°C, T_max={:.2}°C on date {}",
                    self.t_min, self.t_max, self.date
                ),
            });
        }
        Ok(())
    }

    /// Calculates Growing Degree Days (GDD) for this single day given base temperature.
    /// Formula: GDD = max(0.0, ((T_max + T_min) / 2.0) - T_base)
    pub fn daily_gdd(&self, base_temp_celsius: f64) -> f64 {
        let t_mean = (self.t_max + self.t_min) / 2.0;
        (t_mean - base_temp_celsius).max(0.0)
    }

    /// Calculates Diurnal Temperature Range (DTR) = T_max - T_min.
    pub fn dtr(&self) -> f64 {
        (self.t_max - self.t_min).max(0.0)
    }
}

/// The complete 44-feature vector strictly ordered for the CatBoost ONNX inference session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BiometFeatures {
    // 32 Retrospective Rolling Window Features (7d, 14d, 30d, 60d)
    pub gdd_cum_7d: f64,
    pub gdd_cum_14d: f64,
    pub gdd_cum_30d: f64,
    pub gdd_cum_60d: f64,

    pub rain_cum_7d: f64,
    pub rain_cum_14d: f64,
    pub rain_cum_30d: f64,
    pub rain_cum_60d: f64,

    pub rain_max_7d: f64,
    pub rain_max_14d: f64,
    pub rain_max_30d: f64,
    pub rain_max_60d: f64,

    pub cdd_7d: f64,
    pub cdd_14d: f64,
    pub cdd_30d: f64,
    pub cdd_60d: f64,

    pub dtr_mean_7d: f64,
    pub dtr_mean_14d: f64,
    pub dtr_mean_30d: f64,
    pub dtr_mean_60d: f64,

    pub dtr_std_7d: f64,
    pub dtr_std_14d: f64,
    pub dtr_std_30d: f64,
    pub dtr_std_60d: f64,

    pub rad_cum_7d: f64,
    pub rad_cum_14d: f64,
    pub rad_cum_30d: f64,
    pub rad_cum_60d: f64,

    pub rh_mean_7d: f64,
    pub rh_mean_14d: f64,
    pub rh_mean_30d: f64,
    pub rh_mean_60d: f64,

    // 9 Derived & Agronomic Features
    pub month: f64,
    pub day_of_year: f64,
    pub photoperiod_hours: f64,
    pub ptq_30d: f64,
    pub ptq_60d: f64,
    pub vpd_proxy_14d: f64,
    pub vpd_proxy_30d: f64,
    pub lat_clean: f64,
    pub lon_clean: f64,

    // 3 Categorical Encodings (integer codes converted to float for tensor)
    pub rice_ecosystem_code: f64,
    pub rice_variety_code: f64,
    pub province_code: f64,
}

impl BiometFeatures {
    /// Serializes features in exact order of `features_order` defined in `model_metadata.json`
    /// as a flat `Vec<f32>` matching ONNX tensor input layout [1, 44].
    pub fn to_tensor_vec(&self) -> Vec<f32> {
        vec![
            self.gdd_cum_7d as f32,
            self.gdd_cum_14d as f32,
            self.gdd_cum_30d as f32,
            self.gdd_cum_60d as f32,
            self.rain_cum_7d as f32,
            self.rain_cum_14d as f32,
            self.rain_cum_30d as f32,
            self.rain_cum_60d as f32,
            self.rain_max_7d as f32,
            self.rain_max_14d as f32,
            self.rain_max_30d as f32,
            self.rain_max_60d as f32,
            self.cdd_7d as f32,
            self.cdd_14d as f32,
            self.cdd_30d as f32,
            self.cdd_60d as f32,
            self.dtr_mean_7d as f32,
            self.dtr_mean_14d as f32,
            self.dtr_mean_30d as f32,
            self.dtr_mean_60d as f32,
            self.dtr_std_7d as f32,
            self.dtr_std_14d as f32,
            self.dtr_std_30d as f32,
            self.dtr_std_60d as f32,
            self.rad_cum_7d as f32,
            self.rad_cum_14d as f32,
            self.rad_cum_30d as f32,
            self.rad_cum_60d as f32,
            self.rh_mean_7d as f32,
            self.rh_mean_14d as f32,
            self.rh_mean_30d as f32,
            self.rh_mean_60d as f32,
            self.month as f32,
            self.day_of_year as f32,
            self.photoperiod_hours as f32,
            self.ptq_30d as f32,
            self.ptq_60d as f32,
            self.vpd_proxy_14d as f32,
            self.vpd_proxy_30d as f32,
            self.lat_clean as f32,
            self.lon_clean as f32,
            self.rice_ecosystem_code as f32,
            self.rice_variety_code as f32,
            self.province_code as f32,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_weather_record() {
        let record = DailyWeatherRecord {
            date: NaiveDate::from_ymd_opt(2026, 7, 10).unwrap(),
            t_max: 32.5,
            t_min: 24.1,
            precipitation_mm: 12.4,
            radiation_mj_m2: 19.8,
            relative_humidity_pct: 78.5,
            source: "TestStation".into(),
        };
        assert!(record.validate().is_ok());
        assert!((record.daily_gdd(10.0) - 18.3).abs() < 1e-4);
        assert!((record.dtr() - 8.4).abs() < 1e-4);
    }

    #[test]
    fn test_weather_record_tmin_exceeds_tmax() {
        let record = DailyWeatherRecord {
            date: NaiveDate::from_ymd_opt(2026, 7, 10).unwrap(),
            t_max: 20.0,
            t_min: 25.0,
            precipitation_mm: 0.0,
            radiation_mj_m2: 15.0,
            relative_humidity_pct: 70.0,
            source: "CorruptedSensor".into(),
        };
        assert!(record.validate().is_err());
    }

    #[test]
    fn test_tensor_length_equals_44() {
        let dummy = BiometFeatures {
            gdd_cum_7d: 1.0, gdd_cum_14d: 2.0, gdd_cum_30d: 3.0, gdd_cum_60d: 4.0,
            rain_cum_7d: 5.0, rain_cum_14d: 6.0, rain_cum_30d: 7.0, rain_cum_60d: 8.0,
            rain_max_7d: 9.0, rain_max_14d: 10.0, rain_max_30d: 11.0, rain_max_60d: 12.0,
            cdd_7d: 13.0, cdd_14d: 14.0, cdd_30d: 15.0, cdd_60d: 16.0,
            dtr_mean_7d: 17.0, dtr_mean_14d: 18.0, dtr_mean_30d: 19.0, dtr_mean_60d: 20.0,
            dtr_std_7d: 21.0, dtr_std_14d: 22.0, dtr_std_30d: 23.0, dtr_std_60d: 24.0,
            rad_cum_7d: 25.0, rad_cum_14d: 26.0, rad_cum_30d: 27.0, rad_cum_60d: 28.0,
            rh_mean_7d: 29.0, rh_mean_14d: 30.0, rh_mean_30d: 31.0, rh_mean_60d: 32.0,
            month: 7.0, day_of_year: 190.0, photoperiod_hours: 12.6,
            ptq_30d: 1.2, ptq_60d: 1.1, vpd_proxy_14d: 1.5, vpd_proxy_30d: 1.4,
            lat_clean: 14.0, lon_clean: 100.0,
            rice_ecosystem_code: 4.0, rice_variety_code: 73.0, province_code: 6.0,
        };
        let vec = dummy.to_tensor_vec();
        assert_eq!(vec.len(), 44);
    }
}
