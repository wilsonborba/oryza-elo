//! # Oryza-Elo Architecture Guardrail: Domain Services
//!
//! ## Responsabilidade:
//! Lógica de negócio pura: serviço de cálculo agronômico de GDD, serviço de inferência
//! fenológica, orquestração entre adaptadores DAL e respostas para presentation.

pub mod agronomic_advisor;

pub use agronomic_advisor::*;
