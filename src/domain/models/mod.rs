//! # Oryza-Elo Architecture Guardrail: Domain Models
//!
//! ## Responsabilidade:
//! Estruturas de dados (structs), contratos de entrada/saída (inputs/outputs).
//! IMPORTANTE: Modelos de Machine Learning (pesos, estimadores) NUNCA entram aqui.

pub mod advisory;
pub mod farm;
pub mod locale;
pub mod phenology;
pub mod weather;

pub use advisory::*;
pub use farm::*;
pub use locale::*;
pub use phenology::*;
pub use weather::*;
