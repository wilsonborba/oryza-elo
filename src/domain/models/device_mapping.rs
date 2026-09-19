//! # Oryza-Elo Architecture Guardrail: Device Mapping Domain Model
//!
//! Device profiles and schema mapping defining column names, physical units,
//! and scale multipliers for agricultural sensor stations and telemetry loggers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Column specification, unit, and scale multiplier for a single sensor variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorColumnMapping {
    pub column_name: String,
    pub unit: String,
    pub scale: f64,
}

impl SensorColumnMapping {
    pub fn new(col: impl Into<String>, unit: impl Into<String>, scale: f64) -> Self {
        Self {
            column_name: col.into(),
            unit: unit.into(),
            scale,
        }
    }
}

/// Device mapping configuration mapping vendor-specific sensor columns to canonical domain variables.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceMapping {
    pub id: String,
    pub device_name: String,
    pub manufacturer: String,
    pub is_preset: bool,

    pub date_col: String,
    pub date_format: String,

    pub t_max_col: String,
    pub t_max_unit: String,
    pub t_max_scale: f64,

    pub t_min_col: String,
    pub t_min_unit: String,
    pub t_min_scale: f64,

    pub rain_col: String,
    pub rain_unit: String,
    pub rain_scale: f64,

    pub rad_col: String,
    pub rad_unit: String,
    pub rad_scale: f64,

    pub rh_col: String,
    pub rh_unit: String,
    pub rh_scale: f64,

    pub created_at: DateTime<Utc>,
}

impl DeviceMapping {
    /// Converts a raw temperature measurement into standard Celsius (°C).
    pub fn convert_temp(&self, raw: f64, is_max: bool) -> f64 {
        let (unit, scale) = if is_max {
            (&self.t_max_unit, self.t_max_scale)
        } else {
            (&self.t_min_unit, self.t_min_scale)
        };

        let scaled = raw * scale;
        match unit.trim().to_uppercase().as_str() {
            "F" | "FAHRENHEIT" | "DEG_F" => (scaled - 32.0) * 5.0 / 9.0,
            _ => scaled,
        }
    }

    /// Converts raw rain into standard daily millimeters (mm).
    pub fn convert_rain(&self, raw: f64) -> f64 {
        let scaled = raw * self.rain_scale;
        match self.rain_unit.trim().to_lowercase().as_str() {
            "in" | "inch" | "inches" => scaled * 25.4,
            _ => scaled,
        }
    }

    /// Converts raw solar radiation into standard cumulative daily MegaJoules per square meter (MJ/m²/day).
    pub fn convert_radiation(&self, raw: f64) -> f64 {
        let scaled = raw * self.rad_scale;
        match self.rad_unit.trim().to_lowercase().as_str() {
            // Instantaneous or daily mean W/m² integrated over 24 hours (86,400 s / 1e6 = 0.0864 MJ/m²)
            "w/m2" | "w/m^2" | "watt/m2" => scaled * 0.0864,
            // Kilowatt-hours per square meter (1 kWh = 3.6 MJ)
            "kwh/m2" | "kwh/m^2" => scaled * 3.6,
            _ => scaled,
        }
    }

    /// Converts raw relative humidity into percentage [0.0, 100.0].
    pub fn convert_rh(&self, raw: f64) -> f64 {
        raw * self.rh_scale
    }

    /// Factory preset: Pessl Instruments (iMetos) - Austria / Germany
    pub fn pessl_preset() -> Self {
        Self {
            id: "preset_pessl_imetos".into(),
            device_name: "Pessl iMetos 3.3 (Standard Agro)".into(),
            manufacturer: "Pessl Instruments (Austria/Germany)".into(),
            is_preset: true,
            date_col: "Timestamp".into(),
            date_format: "%Y-%m-%d".into(),
            t_max_col: "AirTemp_Max".into(),
            t_max_unit: "C".into(),
            t_max_scale: 1.0,
            t_min_col: "AirTemp_Min".into(),
            t_min_unit: "C".into(),
            t_min_scale: 1.0,
            rain_col: "Precipitation".into(),
            rain_unit: "mm".into(),
            rain_scale: 1.0,
            rad_col: "SolarRad".into(),
            rad_unit: "W/m2".into(),
            rad_scale: 1.0,
            rh_col: "RelHumidity".into(),
            rh_unit: "%".into(),
            rh_scale: 1.0,
            created_at: Utc::now(),
        }
    }

    /// Factory preset: Dragino / Renke RS485 Modbus Telemetry - China
    pub fn dragino_renke_preset() -> Self {
        Self {
            id: "preset_dragino_renke".into(),
            device_name: "Dragino / Renke RS485 Modbus Telemetry".into(),
            manufacturer: "Dragino / Renke (China)".into(),
            is_preset: true,
            date_col: "date".into(),
            date_format: "%Y-%m-%d".into(),
            t_max_col: "temp_max_raw".into(),
            t_max_unit: "C".into(),
            t_max_scale: 0.1, // Integer e.g. 325 -> 32.5°C
            t_min_col: "temp_min_raw".into(),
            t_min_unit: "C".into(),
            t_min_scale: 0.1, // Integer e.g. 240 -> 24.0°C
            rain_col: "pulse_count".into(),
            rain_unit: "mm".into(),
            rain_scale: 0.2, // Tipping bucket 0.2 mm per pulse
            rad_col: "radiation_raw".into(),
            rad_unit: "MJ/m2".into(),
            rad_scale: 0.1,
            rh_col: "humidity_raw".into(),
            rh_unit: "%".into(),
            rh_scale: 0.1, // Integer e.g. 785 -> 78.5%
            created_at: Utc::now(),
        }
    }

    /// Factory preset: Davis Instruments (WeatherLink) - USA
    pub fn davis_preset() -> Self {
        Self {
            id: "preset_davis_vantage".into(),
            device_name: "Davis Vantage Pro2 (WeatherLink CSV)".into(),
            manufacturer: "Davis Instruments (USA)".into(),
            is_preset: true,
            date_col: "Date".into(),
            date_format: "%m/%d/%Y".into(),
            t_max_col: "Temp High".into(),
            t_max_unit: "F".into(),
            t_max_scale: 1.0,
            t_min_col: "Temp Low".into(),
            t_min_unit: "F".into(),
            t_min_scale: 1.0,
            rain_col: "Rain".into(),
            rain_unit: "in".into(),
            rain_scale: 1.0, // Will be converted using 25.4 mm/inch
            rad_col: "Solar Rad".into(),
            rad_unit: "MJ/m2".into(),
            rad_scale: 1.0,
            rh_col: "Hum High".into(),
            rh_unit: "%".into(),
            rh_scale: 1.0,
            created_at: Utc::now(),
        }
    }

    /// Factory preset: NASA POWER Daily Agrometeorological CSV
    pub fn nasa_power_preset() -> Self {
        Self {
            id: "preset_nasa_power".into(),
            device_name: "NASA POWER Daily Point CSV".into(),
            manufacturer: "NASA Langley Research Center".into(),
            is_preset: true,
            date_col: "YEARMODA".into(),
            date_format: "%Y%m%d".into(),
            t_max_col: "T2M_MAX".into(),
            t_max_unit: "C".into(),
            t_max_scale: 1.0,
            t_min_col: "T2M_MIN".into(),
            t_min_unit: "C".into(),
            t_min_scale: 1.0,
            rain_col: "PRECTOTCORR".into(),
            rain_unit: "mm".into(),
            rain_scale: 1.0,
            rad_col: "ALLSKY_SFC_SW_DWN".into(),
            rad_unit: "MJ/m2".into(),
            rad_scale: 1.0,
            rh_col: "RH2M".into(),
            rh_unit: "%".into(),
            rh_scale: 1.0,
            created_at: Utc::now(),
        }
    }

    /// Returns the collection of all 4 official factory presets.
    pub fn all_presets() -> Vec<Self> {
        vec![
            Self::pessl_preset(),
            Self::dragino_renke_preset(),
            Self::davis_preset(),
            Self::nasa_power_preset(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_davis_unit_conversions() {
        let davis = DeviceMapping::davis_preset();

        // 86°F -> 30°C
        let t_c = davis.convert_temp(86.0, true);
        assert!((t_c - 30.0).abs() < 1e-4);

        // 1.0 inch -> 25.4 mm
        let rain_mm = davis.convert_rain(1.0);
        assert!((rain_mm - 25.4).abs() < 1e-4);
    }

    #[test]
    fn test_dragino_renke_scale_conversions() {
        let dragino = DeviceMapping::dragino_renke_preset();

        // Raw 325 -> 32.5°C
        let t_max = dragino.convert_temp(325.0, true);
        assert!((t_max - 32.5).abs() < 1e-4);

        // 15 pulses * 0.2 = 3.0 mm
        let rain = dragino.convert_rain(15.0);
        assert!((rain - 3.0).abs() < 1e-4);

        // Raw 815 -> 81.5%
        let rh = dragino.convert_rh(815.0);
        assert!((rh - 81.5).abs() < 1e-4);
    }
}
