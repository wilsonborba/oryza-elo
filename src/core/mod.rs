//! # Oryza-Elo Architecture Guardrail: Core Layer
//!
//! ## Responsabilidade:
//! Configurações essenciais (settings), logging estruturado e funções utilitárias compartilhadas.
//! Não contém lógica de negócio (domain) nem persistência de dados (dal).
//!
//! ## Regra Estrita para Agentes de IA:
//! 1. É terminantemente PROIBIDO criar pastas paralelas, temporárias ou auxiliares fora da estrutura oficial.
//! 2. Configurações não-secretas e reutilizáveis vivem em `settings.rs`, NUNCA no `.env`.
//! 3. Se um agente planeja criar ou alterar qualquer diretório que não foi explicitamente
//!    alinhado com o usuário, ele DEVE PARAR E PEDIR AUTORIZAÇÃO ANTES.

pub mod settings;
