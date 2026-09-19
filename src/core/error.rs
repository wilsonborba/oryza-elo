//! # Oryza-Elo Architecture Guardrail: Core Errors
//!
//! Error types for domain, ingestion, inference and application layers.

use thiserror::Error;

/// Domain-level errors representing invariant violations in business logic.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum DomainError {
    #[error("Invalid coordinate: latitude {lat} must be in [-90, 90], longitude {lon} in [-180, 180]")]
    InvalidCoordinate { lat: f64, lon: f64 },

    #[error("Invalid weather record: {message}")]
    InvalidWeatherRecord { message: String },

    #[error("Unknown phenology stage: {0}")]
    UnknownPhenologyStage(String),

    #[error("Unknown locale: '{0}'. Supported locales: 'pt-BR', 'en', 'th'")]
    UnknownLocale(String),

    #[error("Insufficient historical weather data: required at least {required} days, but only {available} provided")]
    InsufficientWeatherData { required: usize, available: usize },

    #[error("Domain validation error: {0}")]
    ValidationError(String),
}

/// Top-level application error covering domain, persistence, inference, and transport.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Inference engine error: {0}")]
    Inference(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
pub type DomainResult<T> = std::result::Result<T, DomainError>;
