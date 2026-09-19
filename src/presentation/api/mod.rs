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

pub mod handlers;
pub mod routes;
pub mod state;

pub use handlers::*;
pub use routes::*;
pub use state::*;
