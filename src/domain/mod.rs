//! # Oryza-Elo Architecture Guardrail: Domain Layer
//!
//! ## Responsabilidade:
//! Núcleo das regras de negócio, serviços centrais e estruturas de dados do domínio.
//! - `models/`: Estruturas de dados (structs), contratos de entrada/saída. Modelos de ML NUNCA entram aqui.
//! - `services/`: Serviços centrais (cálculo de GDD, inferência de estágio fenológico, validação).
//! - `tasks/`: Tarefas periódicas e pipelines agendados.
//!
//! ## Regra Estrita para Agentes de IA:
//! 1. É terminantemente PROIBIDO criar pastas paralelas fora de models, services e tasks.
//! 2. Se um agente planeja criar ou alterar qualquer diretório que não foi explicitamente
//!    alinhado com o usuário, ele DEVE PARAR E PEDIR AUTORIZAÇÃO ANTES.

pub mod models;
pub mod services;
pub mod tasks;
