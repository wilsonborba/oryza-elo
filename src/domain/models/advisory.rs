//! # Oryza-Elo Architecture Guardrail: Farmer Advisory Model
//!
//! Domain structures for actionable, farmer-friendly agronomic recommendations.

use crate::domain::models::locale::Locale;
use crate::domain::models::phenology::PhenologyStage;
use serde::{Deserialize, Serialize};

/// Actionable advisory recommendations tailored to farmers in their native language.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FarmerAdvisory {
    /// Target language locale
    pub locale: Locale,
    /// Phenological stage this advisory corresponds to
    pub stage: PhenologyStage,
    /// High-level summary / badge (e.g. "Floração em Andamento: Cuidado com Pulverizações")
    pub headline: String,
    /// Plain-language description of crop development and nutritional demand
    pub plain_text: String,
    /// Urgent operational constraints and warnings that must be highlighted in red/amber
    pub urgent_warnings: Vec<String>,
    /// Practical management actions (irrigation, fertilization, scouting)
    pub management_tips: Vec<String>,
}

impl FarmerAdvisory {
    pub fn new(
        locale: Locale,
        stage: PhenologyStage,
        headline: impl Into<String>,
        plain_text: impl Into<String>,
        urgent_warnings: Vec<String>,
        management_tips: Vec<String>,
    ) -> Self {
        Self {
            locale,
            stage,
            headline: headline.into(),
            plain_text: plain_text.into(),
            urgent_warnings,
            management_tips,
        }
    }
}
