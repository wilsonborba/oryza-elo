//! # Oryza-Elo Architecture Guardrail: Presentation Layer
//!
//! ## Responsabilidade:
//! Camada de interação com o usuário, CLI e interfaces de rede:
//! - `cli/`: Comandos de terminal, handlers e formatters.
//! - `api/`: Servidor HTTP de borda (Axum) com latência < 5ms e rotas REST.
//!
//! ## Regra Estrita para Agentes de IA:
//! 1. Não coloque lógica de negócio diretamente nos handlers ou rotas da API.
//! 2. Presentation chama domain services, e domain services usam DAL adapters.
//! 3. Se um agente planeja criar ou alterar qualquer diretório que não foi explicitamente
//!    alinhado com o usuário, ele DEVE PARAR E PEDIR AUTORIZAÇÃO ANTES.

pub mod api;
pub mod cli;
