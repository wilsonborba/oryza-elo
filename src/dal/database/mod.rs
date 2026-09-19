//! # Oryza-Elo Architecture Guardrail: DAL Database Module
//!
//! SQLite persistence layer, connection pool management and repositories.

pub mod connection;
pub mod repositories;

pub use connection::*;
pub use repositories::*;
