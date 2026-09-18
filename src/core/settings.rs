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

/// Nome oficial da aplicação
pub const DEFAULT_APP_NAME: &str = "oryza-elo";

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

/// Estrutura de configurações em tempo de execução
#[derive(Debug, Clone)]
pub struct AppSettings {
    pub app_name: String,
    pub port: u16,
    pub log_level: String,
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

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            app_name: DEFAULT_APP_NAME.to_string(),
            port: DEFAULT_PORT,
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

impl AppSettings {
    /// Carrega configurações lendo o .env local para preencher paths e segredos de máquina
    pub fn load_from_env() -> Self {
        let _ = dotenvy::dotenv();

        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(DEFAULT_PORT);

        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| DEFAULT_LOG_LEVEL.to_string());

        // Preenchidos a partir do .env local de quem clonou o projeto
        let raw_data_path = std::env::var("RAW_DATA_PATH").ok();
        let processed_data_path = std::env::var("PROCESSED_DATA_PATH").ok();
        let private_api_token = std::env::var("PRIVATE_API_TOKEN").ok();

        Self {
            port,
            log_level,
            raw_data_path,
            processed_data_path,
            private_api_token,
            ..Default::default()
        }
    }
}
