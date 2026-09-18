//! # Oryza-Elo Architecture Guardrail: Settings & Configuration
//!
//! ## Regras Fundamentais de Segredos e Configurações (AI_AGENT_Working_Rules.md, Regra 1):
//! 1. **Configurações Reutilizáveis e Não-Secretas**:
//!    - Devem viver SEMPRE aqui no código (`settings.rs`), NUNCA no `.env`.
//!    - Exemplos: URLs públicas de APIs (como NASA POWER), portas padrão, caminhos canônicos
//!      dentro do repositório, parâmetros agronômicos (temperatura base de 10°C) e limites
//!      de latência (< 200 ms).
//! 2. **Segredos, Credenciais e Tokens Privados**:
//!    - Devem viver EXCLUSIVAMENTE no `.env` (arquivo local, fora do Git).
//!    - No código em `settings.rs`, qualquer variável de credencial/segredo DEVE ter valor
//!      default NULO (`Option::None`), sendo preenchida apenas em tempo de execução via `.env`.
//! 3. **Caminhos Locais Específicos de Máquina**:
//!    - Caminhos que mudam de um usuário/desenvolvedor para outro pertencem ao `.env`.
//!    - Caminhos relativos padrão do repositório pertencem a `settings.rs`.
//! 4. **Sobrescrita via .env**:
//!    - O `.env` pode sobrescrever variáveis não-secretas de `settings.rs` durante desenvolvimento
//!      local, mas `settings.rs` é SEMPRE a fonte oficial e canônica de verdade para defaults.

/// Nome oficial da aplicação
pub const DEFAULT_APP_NAME: &str = "oryza-elo";

/// Porta padrão do servidor de borda Axum
pub const DEFAULT_PORT: u16 = 8005;

/// Nível padrão de log estruturado
pub const DEFAULT_LOG_LEVEL: &str = "INFO";

/// Endpoint base público da API NASA POWER (Constante pública da aplicação, nunca segredo)
pub const NASA_POWER_BASE_URL: &str = "https://power.larc.nasa.gov/api/temporal/daily/point";

/// Caminhos canônicos relativos dentro do repositório
pub const DEFAULT_RAW_DATA_PATH: &str = "src/dal/data/raw/ricepest_survey_combined_66_68.csv";
pub const DEFAULT_PROCESSED_DATA_PATH: &str = "src/dal/data/processed/rice_survey_climate_enriched.csv";

/// Parâmetros Agronômicos Canônicos (Orizicultura)
/// Temperatura base fisiológica do arroz para cálculo de Graus-Dia Acumulados (GDD)
pub const RICE_BASE_TEMPERATURE_CELSIUS: f64 = 10.0;

/// Janelas temporais retrospectivas padrão para extração de séries climáticas (dias)
pub const RETROSPECTIVE_WINDOWS_DAYS: [u32; 4] = [7, 14, 30, 60];

/// Teto máximo de latência de inferência em hardware de borda (Comentário 2 do Prof. Alexandre Duarte)
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
    pub raw_data_path: String,
    pub processed_data_path: String,
    pub base_temp_celsius: f64,
    /// Credenciais e tokens privados possuem valor default NULO (Option::None)
    pub private_api_token: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            app_name: DEFAULT_APP_NAME.to_string(),
            port: DEFAULT_PORT,
            log_level: DEFAULT_LOG_LEVEL.to_string(),
            nasa_power_url: NASA_POWER_BASE_URL.to_string(),
            raw_data_path: DEFAULT_RAW_DATA_PATH.to_string(),
            processed_data_path: DEFAULT_PROCESSED_DATA_PATH.to_string(),
            base_temp_celsius: RICE_BASE_TEMPERATURE_CELSIUS,
            private_api_token: None, // Sempre None por padrão; carregado apenas se presente no .env
        }
    }
}

impl AppSettings {
    /// Carrega configurações lendo o .env local para eventuais sobrescritas de máquina
    pub fn load_from_env() -> Self {
        let _ = dotenvy::dotenv(); // Carrega .env se existir, sem falhar se ausente

        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(DEFAULT_PORT);

        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| DEFAULT_LOG_LEVEL.to_string());

        let private_api_token = std::env::var("PRIVATE_API_TOKEN").ok();

        Self {
            port,
            log_level,
            private_api_token,
            ..Default::default()
        }
    }
}
