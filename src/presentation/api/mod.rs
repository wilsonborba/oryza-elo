//! # Oryza-Elo Architecture Guardrail: API Presentation
//!
//! ## Responsabilidade:
//! Servidor HTTP Axum de borda, rotas REST, medição de latência em microssegundos (< 5ms).
//!
//! ## Dualidade de Apresentação (Local vs Nuvem):
//! - No Raspberry Pi (modo local/borda): se `static_dir` estiver configurado no `.env` e o
//!   diretório existir no disco, o Axum monta o `ServeDir` como fallback para servir o
//!   frontend Flutter Web localmente com zero dependências externas.
//! - Na Nuvem Asodya: `static_dir` permanece `None`, com o frontend servido via Cloudflare Pages
//!   e o Axum atuando puramente como API headless.
//!
//! ## Regra Estrita para Agentes de IA:
//! Não crie pastas auxiliares ou paralelas. Peça autorização antes de qualquer alteração estrutural.

use axum::Router;
use std::path::Path;
use tower_http::services::ServeDir;
use crate::core::settings::app_settings;

/// Constrói o roteador HTTP Axum da aplicação de borda
pub fn create_router() -> Router {
    let settings = app_settings();

    // Rotas REST da API de borda
    let api_routes = Router::new()
        .route("/health", axum::routing::get(health_check));

    let mut router = Router::new()
        .nest("/api/v1", api_routes);

    // Servir arquivos estáticos do Flutter Web condicionalmente (Modo Raspberry Pi / Local)
    if let Some(ref static_path) = settings.static_dir {
        if Path::new(static_path).exists() {
            tracing::info!(
                static_dir = %static_path,
                "Montando fallback de assets estáticos do Flutter Web (modo local/Raspberry Pi)"
            );
            router = router.fallback_service(ServeDir::new(static_path));
        } else {
            tracing::warn!(
                static_dir = %static_path,
                "STATIC_DIR configurado, mas diretório não encontrado no disco. Servindo apenas API."
            );
        }
    }

    router
}

/// Endpoint de verificação de integridade da API de borda
async fn health_check() -> &'static str {
    "OK"
}
