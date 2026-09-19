//! # Oryza-Elo Architecture Guardrail: Phenology Domain Models
//!
//! Phenological stages, macro-phases, classification metrics, and prediction outputs.

use crate::core::error::DomainError;
use crate::domain::models::advisory::FarmerAdvisory;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

/// Macro-phases of the rice crop according to BBCH monograph and IRRI standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroPhase {
    /// BBCH 10–29: Seedling to maximum tillering
    Vegetative,
    /// BBCH 40–69: Panicle initiation/booting, heading, anthesis/flowering
    Reproductive,
    /// BBCH 70–99: Milk/dough ripening to physiological maturity and harvest
    Ripening,
}

impl MacroPhase {
    pub fn code(&self) -> &'static str {
        match self {
            MacroPhase::Vegetative => "1_Vegetative",
            MacroPhase::Reproductive => "2_Reproductive",
            MacroPhase::Ripening => "3_Ripening",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            MacroPhase::Vegetative => "Vegetative Phase (BBCH 10–29)",
            MacroPhase::Reproductive => "Reproductive Phase (BBCH 40–69)",
            MacroPhase::Ripening => "Ripening & Senescence Phase (BBCH 70–99)",
        }
    }
}

/// 7 Granular phenological stages of the rice crop aligned with Thai Rice Department dataset,
/// IRRI standard, and BBCH monograph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhenologyStage {
    /// BBCH 10–19: Plântula / Seedling / ระยะกล้า (Model Index 3)
    Seedling,
    /// BBCH 20–29: Perfilhamento / Tillering / แตกกอ (Model Index 6)
    Tillering,
    /// BBCH 40–49: Emborrachamento / Booting / ตั้งท้อง (Model Index 2)
    Booting,
    /// BBCH 50–59: Espigamento / Heading / ออกรวง (Model Index 5)
    Heading,
    /// BBCH 60–69: Floração / Flowering / ออกดอก (Model Index 4)
    Flowering,
    /// BBCH 70–89: Maturação Pré-Colheita / Pre-Harvest / ก่อนเก็บเกี่ยว (Model Index 0)
    PreHarvest,
    /// BBCH 90–99: Colheita Plena / Harvest-Ready / ก่อนเก็บเกี่ยวเกี่ยว (Model Index 1)
    HarvestReady,
}

impl PhenologyStage {
    /// Array of all 7 stages in chronological development order.
    pub const ALL_CHRONOLOGICAL: [PhenologyStage; 7] = [
        PhenologyStage::Seedling,
        PhenologyStage::Tillering,
        PhenologyStage::Booting,
        PhenologyStage::Heading,
        PhenologyStage::Flowering,
        PhenologyStage::PreHarvest,
        PhenologyStage::HarvestReady,
    ];

    /// Array of all 7 stages indexed strictly by CatBoost/ONNX model class output index.
    /// Derived from `classes_7` in model_metadata.json:
    /// [0: "ก่อนเก็บเกี่ยว", 1: "ก่อนเก็บเกี่ยวเกี่ยว", 2: "ตั้งท้อง", 3: "ระยะกล้า", 4: "ออกดอก", 5: "ออกรวง", 6: "แตกกอ"]
    pub const MODEL_CLASS_ORDER: [PhenologyStage; 7] = [
        PhenologyStage::PreHarvest,
        PhenologyStage::HarvestReady,
        PhenologyStage::Booting,
        PhenologyStage::Seedling,
        PhenologyStage::Flowering,
        PhenologyStage::Heading,
        PhenologyStage::Tillering,
    ];

    /// Resolve stage from the 0-indexed CatBoost/ONNX model output index.
    pub fn from_model_index(idx: usize) -> Result<Self, DomainError> {
        Self::MODEL_CLASS_ORDER
            .get(idx)
            .copied()
            .ok_or_else(|| DomainError::UnknownPhenologyStage(format!("Model index {} out of range [0, 6]", idx)))
    }

    /// Convert stage into its 0-indexed CatBoost/ONNX model output index.
    pub fn to_model_index(&self) -> usize {
        match self {
            PhenologyStage::PreHarvest => 0,
            PhenologyStage::HarvestReady => 1,
            PhenologyStage::Booting => 2,
            PhenologyStage::Seedling => 3,
            PhenologyStage::Flowering => 4,
            PhenologyStage::Heading => 5,
            PhenologyStage::Tillering => 6,
        }
    }

    /// Standard BBCH scale range.
    pub fn bbch_code(&self) -> &'static str {
        match self {
            PhenologyStage::Seedling => "10-19",
            PhenologyStage::Tillering => "20-29",
            PhenologyStage::Booting => "40-49",
            PhenologyStage::Heading => "50-59",
            PhenologyStage::Flowering => "60-69",
            PhenologyStage::PreHarvest => "70-89",
            PhenologyStage::HarvestReady => "90-99",
        }
    }

    /// Macro-phase corresponding to this granular stage.
    pub fn macro_phase(&self) -> MacroPhase {
        match self {
            PhenologyStage::Seedling | PhenologyStage::Tillering => MacroPhase::Vegetative,
            PhenologyStage::Booting | PhenologyStage::Heading | PhenologyStage::Flowering => {
                MacroPhase::Reproductive
            }
            PhenologyStage::PreHarvest | PhenologyStage::HarvestReady => MacroPhase::Ripening,
        }
    }

    /// Exact Thai label from Thai Rice Department ground truth survey.
    pub fn thai_label(&self) -> &'static str {
        match self {
            PhenologyStage::Seedling => "ระยะกล้า",
            PhenologyStage::Tillering => "แตกกอ",
            PhenologyStage::Booting => "ตั้งท้อง",
            PhenologyStage::Heading => "ออกรวง",
            PhenologyStage::Flowering => "ออกดอก",
            PhenologyStage::PreHarvest => "ก่อนเก็บเกี่ยว",
            PhenologyStage::HarvestReady => "ก่อนเก็บเกี่ยวเกี่ยว",
        }
    }

    /// English agronomic stage name.
    pub fn english_name(&self) -> &'static str {
        match self {
            PhenologyStage::Seedling => "Seedling",
            PhenologyStage::Tillering => "Tillering",
            PhenologyStage::Booting => "Booting",
            PhenologyStage::Heading => "Heading / Panicle Exsertion",
            PhenologyStage::Flowering => "Flowering / Anthesis",
            PhenologyStage::PreHarvest => "Pre-Harvest / Ripening",
            PhenologyStage::HarvestReady => "Late Ripening / Harvest-ready",
        }
    }

    /// Portuguese agronomic stage name.
    pub fn portuguese_name(&self) -> &'static str {
        match self {
            PhenologyStage::Seedling => "Plântula",
            PhenologyStage::Tillering => "Perfilhamento",
            PhenologyStage::Booting => "Emborrachamento",
            PhenologyStage::Heading => "Espigamento",
            PhenologyStage::Flowering => "Floração / Antese",
            PhenologyStage::PreHarvest => "Maturação Pré-Colheita",
            PhenologyStage::HarvestReady => "Colheita Plena",
        }
    }

    /// Parse from Thai Rice Department label string.
    pub fn from_thai_label(s: &str) -> Result<Self, DomainError> {
        match s.trim() {
            "ระยะกล้า" => Ok(PhenologyStage::Seedling),
            "แตกกอ" => Ok(PhenologyStage::Tillering),
            "ตั้งท้อง" => Ok(PhenologyStage::Booting),
            "ออกรวง" => Ok(PhenologyStage::Heading),
            "ออกดอก" => Ok(PhenologyStage::Flowering),
            "ก่อนเก็บเกี่ยว" => Ok(PhenologyStage::PreHarvest),
            "ก่อนเก็บเกี่ยวเกี่ยว" => Ok(PhenologyStage::HarvestReady),
            other => Err(DomainError::UnknownPhenologyStage(other.to_string())),
        }
    }

    /// Chronological sequence order (1 to 7).
    pub fn sequence_order(&self) -> u8 {
        match self {
            PhenologyStage::Seedling => 1,
            PhenologyStage::Tillering => 2,
            PhenologyStage::Booting => 3,
            PhenologyStage::Heading => 4,
            PhenologyStage::Flowering => 5,
            PhenologyStage::PreHarvest => 6,
            PhenologyStage::HarvestReady => 7,
        }
    }
}

impl fmt::Display for PhenologyStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.english_name())
    }
}

impl FromStr for PhenologyStage {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cleaned = s.trim().to_lowercase();
        match cleaned.as_str() {
            "seedling" | "plantula" | "plântula" | "10-19" => Ok(PhenologyStage::Seedling),
            "tillering" | "perfilhamento" | "20-29" => Ok(PhenologyStage::Tillering),
            "booting" | "emborrachamento" | "40-49" => Ok(PhenologyStage::Booting),
            "heading" | "espigamento" | "panicle" | "50-59" => Ok(PhenologyStage::Heading),
            "flowering" | "floracao" | "floração" | "anthesis" | "60-69" => Ok(PhenologyStage::Flowering),
            "pre_harvest" | "preharvest" | "maturacao" | "maturação" | "ripening" | "70-89" => {
                Ok(PhenologyStage::PreHarvest)
            }
            "harvest_ready" | "harvestready" | "colheita" | "senescence" | "90-99" => {
                Ok(PhenologyStage::HarvestReady)
            }
            _ => Self::from_thai_label(s),
        }
    }
}

/// Technical metrics produced alongside inference for ML auditing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalMetrics {
    pub model_version: String,
    pub inference_latency_ms: f64,
    pub entropy: f64,
    pub margin: f64,
}

/// Consolidated phenology prediction output for farm parcel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhenologyPrediction {
    pub evaluated_at: DateTime<Utc>,
    pub parcel_id: String,
    pub macro_phase: MacroPhase,
    pub granular_stage: PhenologyStage,
    pub confidence: f64,
    pub is_transitioning: bool,
    pub probabilities: BTreeMap<String, f64>,
    pub advisory: FarmerAdvisory,
    pub all_translations: BTreeMap<String, FarmerAdvisory>,
    pub technical_metrics: TechnicalMetrics,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phenology_stage_mappings() {
        for (idx, &stage) in PhenologyStage::MODEL_CLASS_ORDER.iter().enumerate() {
            assert_eq!(PhenologyStage::from_model_index(idx).unwrap(), stage);
            assert_eq!(stage.to_model_index(), idx);
        }
    }

    #[test]
    fn test_thai_label_roundtrip() {
        for stage in PhenologyStage::ALL_CHRONOLOGICAL {
            let thai = stage.thai_label();
            let parsed = PhenologyStage::from_thai_label(thai).unwrap();
            assert_eq!(parsed, stage);
        }
    }

    #[test]
    fn test_macro_phase_alignment() {
        assert_eq!(PhenologyStage::Seedling.macro_phase(), MacroPhase::Vegetative);
        assert_eq!(PhenologyStage::Tillering.macro_phase(), MacroPhase::Vegetative);
        assert_eq!(PhenologyStage::Booting.macro_phase(), MacroPhase::Reproductive);
        assert_eq!(PhenologyStage::Heading.macro_phase(), MacroPhase::Reproductive);
        assert_eq!(PhenologyStage::Flowering.macro_phase(), MacroPhase::Reproductive);
        assert_eq!(PhenologyStage::PreHarvest.macro_phase(), MacroPhase::Ripening);
        assert_eq!(PhenologyStage::HarvestReady.macro_phase(), MacroPhase::Ripening);
    }
}
