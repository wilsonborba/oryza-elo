//! # Oryza-Elo Architecture Guardrail: Locale Model
//!
//! Trilingual localization model supporting Brazilian Portuguese (pt-BR),
//! English (en), and Thai (th).

use crate::core::error::DomainError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// Supported locales for farmer agronomic advisories and edge interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Locale {
    /// Portuguese (Brazil) - Academic & ESALQ/USP research agronomy standard
    PtBr,
    /// English - Asodya commercial & international standard
    En,
    /// Thai - Field operations & Thai Rice Department standard
    Th,
}

impl Locale {
    /// Canonical ISO-style code string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Locale::PtBr => "pt-BR",
            Locale::En => "en",
            Locale::Th => "th",
        }
    }

    /// List of all supported locales
    pub fn all() -> &'static [Locale] {
        &[Locale::PtBr, Locale::En, Locale::Th]
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Locale {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().replace('_', "-").as_str() {
            "pt-br" | "pt" | "pt_br" | "portuguese" => Ok(Locale::PtBr),
            "en" | "en-us" | "en-gb" | "english" => Ok(Locale::En),
            "th" | "th-th" | "thai" => Ok(Locale::Th),
            other => Err(DomainError::UnknownLocale(other.to_string())),
        }
    }
}

impl Serialize for Locale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Locale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Locale::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Default for Locale {
    fn default() -> Self {
        Locale::En
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_parsing() {
        assert_eq!("pt-BR".parse::<Locale>().unwrap(), Locale::PtBr);
        assert_eq!("pt_br".parse::<Locale>().unwrap(), Locale::PtBr);
        assert_eq!("pt".parse::<Locale>().unwrap(), Locale::PtBr);
        assert_eq!("en".parse::<Locale>().unwrap(), Locale::En);
        assert_eq!("th".parse::<Locale>().unwrap(), Locale::Th);
        assert_eq!("thai".parse::<Locale>().unwrap(), Locale::Th);
        assert!("fr".parse::<Locale>().is_err());
    }

    #[test]
    fn test_locale_serde() {
        let loc = Locale::PtBr;
        let json = serde_json::to_string(&loc).unwrap();
        assert_eq!(json, "\"pt-BR\"");

        let deserialized: Locale = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Locale::PtBr);
    }
}
