//! # Oryza-Elo Architecture Guardrail: Data Access Layer (DAL)
//!
//! ## Responsabilidade:
//! Acesso a dados de todas as fontes:
//! - `local/`: Adaptadores de arquivos locais, cache, leitor de CSV/banco de dados.
//! - `remote/`: Adaptador de consumo de APIs remotas (NASA POWER API, etc.).
//! - `data/`: Datasets físicos oficiais (`raw/` e `processed/`). NENHUM dado pode ficar fora de `dal/data/`.
//! - `research/`: Relatórios técnicos, análises exploratórias (EDA) e validações estatísticas.
//!   NENHUMA pesquisa ou análise exploratória pode ficar fora de `dal/research/`.
//!
//! ## Regra Estrita para Agentes de IA:
//! 1. É terminantemente PROIBIDO criar pastas paralelas de dados ou pesquisa na raiz ou fora de `dal/`.
//! 2. Todo dado, CSV ou relatório exploratório DEVE viver estritamente dentro de `dal/`.
//! 3. Se um agente planeja criar ou alterar qualquer diretório que não foi explicitamente
//!    alinhado com o usuário, ele DEVE PARAR E PEDIR AUTORIZAÇÃO ANTES.

pub mod local;
pub mod remote;
