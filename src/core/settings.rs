//! # Oryza-Elo Architecture Guardrail: Settings & Configuration
//!
//! ## Regras Fundamentais (AI_AGENT_Working_Rules.md, Regra 1):
//! 1. **Configurações Reutilizáveis e Não-Secretas Globais**:
//!    - Devem viver aqui no código (`settings.rs`).
//!    - Exemplos: URLs públicas de APIs (como NASA POWER), parâmetros agronômicos (temperatura base
//!      de 10°C) e limites de latência da tese (< 200 ms).
//! 2. **Segredos, Credenciais, Senhas e Caminhos Locais Específicos de Máquina**:
//!    - Paths locais, keys, senhas e tokens DEVEM ser definidos no `.env` e ter valores
//!      estritamente NULOS (`Option::None`) aqui em `settings.rs` por padrão.
//!    - Cada pessoa/máquina que clona o projeto define seus próprios caminhos e credenciais no `.env`.
//!    - Em `settings.rs`, qualquer variável de caminho local ou segredo nasce como `None`.
//! 3. **Arquitetura de Apresentação (Frontend no Cloudflare)**:
//!    - Não servimos arquivos estáticos pelo Axum Rust (sem static_dir ou templates_dir no backend),
//!      pois o frontend/simulador vive e é servido independentemente via Cloudflare.

use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;

/// Nome oficial da aplicação
pub const DEFAULT_APP_NAME: &str = "oryza-elo";

/// Host padrão do servidor
pub const DEFAULT_SERVER_HOST: &str = "0.0.0.0";

/// Porta padrão do servidor de borda Axum
pub const DEFAULT_PORT: u16 = 8005;

/// Nível padrão de log estruturado
pub const DEFAULT_LOG_LEVEL: &str = "INFO";

/// Endpoint base público da API NASA POWER (Constante pública da aplicação)
pub const NASA_POWER_BASE_URL: &str = "https://power.larc.nasa.gov/api/temporal/daily/point";

/// Parâmetros Agronômicos Canônicos (Orizicultura)
/// Temperatura base fisiológica do arroz para cálculo de Graus-Dia Acumulados (GDD)
pub const RICE_BASE_TEMPERATURE_CELSIUS: f64 = 10.0;

/// Janelas temporais retrospectivas padrão para extração de séries climáticas (dias)
pub const RETROSPECTIVE_WINDOWS_DAYS: [u32; 4] = [7, 14, 30, 60];

/// Teto máximo de latência de inferência em hardware de borda (Comentário 2 do orientador)
pub const EDGE_LATENCY_CEILING_MS: u64 = 200;

/// Meta de latência interna do backend em Rust (submilissegundos / microssegundos)
pub const RUST_EDGE_TARGET_LATENCY_MS: u64 = 5;

/// Struct para armazenar configurações em tempo de execução
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Settings {
    // Configurações do servidor
    pub app_name: String,
    pub server_host: String,
    pub server_port: u16,
    pub log_level: String,

    // Parâmetros científicos e de APIs públicas
    pub nasa_power_url: String,
    pub base_temp_celsius: f64,

    // =========================================================================
    // VARIÁVEIS COM VALOR DEFAULT ESTRITAMENTE NULO (Option::None)
    // Devem ser fornecidas exclusivamente via .env local de cada máquina/usuário
    // =========================================================================
    /// Caminho local para o dataset bruto (definido no .env de quem clonou; default NULO)
    pub raw_data_path: Option<String>,

    /// Caminho local para o dataset processado (definido no .env de quem clonou; default NULO)
    pub processed_data_path: Option<String>,

    /// Chave ou token de API privada (definido no .env; default NULO)
    pub private_api_token: Option<String>,
}

pub type AppSettings = Settings;

impl Default for Settings {
    fn default() -> Self {
        Self {
            app_name: DEFAULT_APP_NAME.to_string(),
            server_host: DEFAULT_SERVER_HOST.to_string(),
            server_port: DEFAULT_PORT,
            log_level: DEFAULT_LOG_LEVEL.to_string(),
            nasa_power_url: NASA_POWER_BASE_URL.to_string(),
            base_temp_celsius: RICE_BASE_TEMPERATURE_CELSIUS,
            // Paths locais e segredos são estritamente nulos por padrão no código
            raw_data_path: None,
            processed_data_path: None,
            private_api_token: None,
        }
    }
}

lazy_static! {
    pub static ref SETTINGS: Settings = {
        // Carrega variáveis do .env local se existir
        let _ = dotenv();

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| DEFAULT_SERVER_HOST.to_string());
        let server_port = env::var("SERVER_PORT")
            .or_else(|_| env::var("PORT"))
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(DEFAULT_PORT);
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| DEFAULT_LOG_LEVEL.to_string());
        let nasa_power_url = env::var("NASA_POWER_BASE_URL").unwrap_or_else(|_| NASA_POWER_BASE_URL.to_string());
        let base_temp_celsius = env::var("RICE_BASE_TEMPERATURE_CELSIUS")
            .ok()
            .and_then(|t| t.parse::<f64>().ok())
            .unwrap_or(RICE_BASE_TEMPERATURE_CELSIUS);

        // Preenchidos apenas a partir do .env local da máquina (default None / nulo)
        let raw_data_path = env::var("RAW_DATA_PATH").ok();
        let processed_data_path = env::var("PROCESSED_DATA_PATH").ok();
        let private_api_token = env::var("PRIVATE_API_TOKEN").ok();

        Settings {
            app_name: DEFAULT_APP_NAME.to_string(),
            server_host,
            server_port,
            log_level,
            nasa_power_url,
            base_temp_celsius,
            raw_data_path,
            processed_data_path,
            private_api_token,
        }
    };
}

#[allow(dead_code)]
pub fn app_settings() -> &'static Settings {
    &SETTINGS
}
