//! AI/LLM-Powered Metrics for Best-in-Class Code Analysis
//!
//! This module provides advanced AI-powered metrics that complement
//! traditional code analysis metrics with semantic understanding,
//! pattern recognition, and intelligent insights.
//!
//! ## Metric Categories
//!
//! ### Complexity & Maintainability
//! - `semantic_complexity` - Language-aware complexity analysis
//! - `refactoring_readiness` - Identifies refactoring opportunities
//! - `code_smell_density` - Detects and quantifies code smells
//!
//! ### Quality & Architecture
//! - `ai_code_quality` - Composite quality score with weighted factors
//! - `testability_score` - Predicts test-ability and modularity
//! - `type_safety` - Type coverage and safety analysis
//!
//! ### Dependencies & Structure
//! - `dependency_coupling` - Measures inter-module coupling strength
//! - `error_handling` - Error path coverage and robustness
//!
//! ### Database Integration
//! - `postgresql_enriched` - PostgreSQL-backed pattern learning

pub mod ai_code_quality;
pub mod code_smell_density;
pub mod dependency_coupling;
pub mod error_handling;
pub mod postgresql_enriched;
pub mod refactoring_readiness;
pub mod semantic_complexity;
pub mod testability_score;
pub mod type_safety;

pub use ai_code_quality::*;
pub use code_smell_density::*;
pub use dependency_coupling::*;
pub use error_handling::*;
pub use postgresql_enriched::*;
pub use refactoring_readiness::*;
pub use semantic_complexity::*;
pub use testability_score::*;
pub use type_safety::*;
