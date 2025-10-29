//! rust-code-analysis is a library to analyze and extract information
//! from source codes written in many different programming languages.
//!
//! You can find the source code of this software on
//! <a href="https://github.com/singularity/rust-code-analysis/" target="_blank">GitHub</a>,
//! while issues and feature requests can be posted on the respective
//! <a href="https://github.com/singularity/rust-code-analysis/issues/" target="_blank">GitHub Issue Tracker</a>.
//!
//! ## Supported Languages
//!
//! - C++
//! - C#
//! - CSS
//! - Go
//! - HTML
//! - Java
//! - JavaScript
//! - The JavaScript used in Firefox internal
//! - Python
//! - Rust
//! - Typescript
//!
//! ## Supported Metrics
//!
//! ### Traditional Metrics
//! - **CC**: Cyclomatic Complexity - code complexity via control flow
//! - **SLOC**: Source Lines of Code
//! - **PLOC**: Physical Lines of Code
//! - **LLOC**: Logical Lines of Code
//! - **CLOC**: Comment Lines of Code
//! - **BLANK**: Blank Lines
//! - **HALSTEAD**: Effort, bugs, time, difficulty
//! - **MI**: Maintainability Index
//! - **NOM**: Number of Methods/Functions
//! - **NEXITS**: Exit points
//! - **NARGS**: Function arguments
//!
//! ### AI-Powered Metrics (NEW)
//! - **Semantic Complexity**: Language-aware complexity analysis
//! - **Code Smell Density**: Anti-pattern detection
//! - **Refactoring Readiness**: Opportunity scoring
//! - **AI Code Quality**: Weighted quality factors
//! - **Testability Score**: Test-ability prediction
//! - **Type Safety**: Type coverage & safety analysis
//! - **Dependency Coupling**: Inter-module coupling strength
//! - **Error Handling Coverage**: Exception path robustness

#![allow(clippy::upper_case_acronyms)]

mod c_langs_macros;
mod c_macro;
mod getter;
pub use crate::getter::*;
mod macros;

mod alterator;
pub use alterator::*;

mod node;
pub use crate::node::*;

mod metrics;
pub use metrics::*;

mod languages;
pub(crate) use languages::*;

mod checker;
pub(crate) use checker::*;

mod output;
pub use output::*;

mod spaces;
pub use crate::spaces::*;

mod ops;
pub use crate::ops::*;

mod find;
pub use crate::find::*;

mod function;
pub use crate::function::*;

mod ast;
pub use crate::ast::*;

mod analysis_context;
pub(crate) use analysis_context::*;

mod count;
pub use crate::count::*;

mod preproc;
pub use crate::preproc::*;

mod langs;
pub use crate::langs::*;

mod tools;
pub use crate::tools::*;

mod concurrent_files;
pub use crate::concurrent_files::*;

mod traits;
pub use crate::traits::*;

mod parser;
pub use crate::parser::*;

mod parser_registry;
pub use crate::parser_registry::*;

mod comment_rm;
pub use crate::comment_rm::*;
