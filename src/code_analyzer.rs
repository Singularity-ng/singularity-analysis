use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::parser_registry::ParserRegistry;
use crate::preproc::PreprocResults;
use crate::{get_function_spaces, spaces::FuncSpace, LANG};

/// Error returned by the [`SingularityCodeAnalyzer`].
#[derive(Debug)]
pub enum AnalyzerError {
    /// The requested language is not supported by the analyzer.
    UnsupportedLanguage(String),
    /// The underlying metrics pipeline failed to produce data.
    AnalysisFailed { language: LANG, reason: String },
    /// I/O error while reading the source under analysis.
    Io(std::io::Error),
}

impl fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalyzerError::UnsupportedLanguage(lang) => {
                write!(f, "language `{}` is not supported by Singularity Code Analyzer", lang)
            }
            AnalyzerError::AnalysisFailed { language, reason } => write!(
                f,
                "failed to compute metrics for {:?}: {}",
                language, reason
            ),
            AnalyzerError::Io(err) => write!(f, "failed to read source: {}", err),
        }
    }
}

impl std::error::Error for AnalyzerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AnalyzerError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AnalyzerError {
    fn from(value: std::io::Error) -> Self {
        AnalyzerError::Io(value)
    }
}

/// Result of a language analysis request.
#[derive(Debug, Clone)]
pub struct AnalyzerResult {
    /// Language that was analyzed.
    pub language: LANG,
    /// Root function space containing nested spaces and metrics.
    pub root_space: FuncSpace,
}

impl AnalyzerResult {
    /// Borrow the aggregated metrics for the analyzed space.
    pub fn metrics(&self) -> &crate::spaces::CodeMetrics {
        &self.root_space.metrics
    }
}

/// Options for running the analyzer over in-memory content.
#[derive(Debug, Clone)]
pub struct AnalyzeOptions<'a> {
    /// Optional virtual path to associate with the content.
    pub virtual_path: Option<&'a Path>,
    /// Optional preprocessing results (macros, includes, ...).
    pub preprocessor: Option<Arc<PreprocResults>>,
}

impl<'a> Default for AnalyzeOptions<'a> {
    fn default() -> Self {
        Self {
            virtual_path: None,
            preprocessor: None,
        }
    }
}

/// High-level façade for running Singularity's multi-language metrics engine.
///
/// This wrapper provides a stable API around the low-level parser/metrics
/// primitives exposed by the crate and always routes language dispatch through
/// the shared [`ParserRegistry`].
pub struct SingularityCodeAnalyzer {
    registry: ParserRegistry,
}

impl Default for SingularityCodeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SingularityCodeAnalyzer {
    /// Create a new analyzer with all built-in languages registered.
    pub fn new() -> Self {
        Self {
            registry: ParserRegistry::with_builtins(),
        }
    }

    /// Create a new analyzer using a custom parser registry.
    pub fn with_registry(registry: ParserRegistry) -> Self {
        Self { registry }
    }

    /// Return the set of languages supported by the analyzer.
    pub fn supported_languages(&self) -> Vec<LANG> {
        let mut langs = self.registry.supported_languages();
        langs.sort_unstable();
        langs
    }

    /// Attempt to map the provided language identifier to an internal [`LANG`].
    ///
    /// Matching is case-insensitive and accepts both enum variants (`"Rust"`)
    /// and display names (`"rust"`).
    pub fn language_from_str(&self, value: &str) -> Option<LANG> {
        let normalized = value.trim().to_lowercase();
        LANG::into_enum_iter().find(|lang| {
            lang.get_name() == normalized || format!("{:?}", lang).to_lowercase() == normalized
        })
    }

    /// Detect the language for the given file path using the registry's extension table.
    pub fn detect_language_from_path(&self, path: &Path) -> Option<LANG> {
        self.registry.detect_language_from_path(path)
    }

    /// Analyze the provided source buffer for the specified language.
    pub fn analyze_language<'a>(
        &self,
        language: LANG,
        source: impl AsRef<[u8]>,
        options: AnalyzeOptions<'a>,
    ) -> Result<AnalyzerResult, AnalyzerError> {
        if self.registry.get_factory(&language).is_none() {
            return Err(AnalyzerError::UnsupportedLanguage(language.get_name().to_string()));
        }

        let path_buf = options
            .virtual_path
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(format!("memory.{}", language.get_name())));

        let buffer = source.as_ref().to_vec();
        let root_space = get_function_spaces(&language, buffer, &path_buf, options.preprocessor)
            .ok_or_else(|| AnalyzerError::AnalysisFailed {
                language,
                reason: "metric pipeline returned no data".to_string(),
            })?;

        Ok(AnalyzerResult {
            language,
            root_space,
        })
    }

    /// Analyze a file on disk. The language is detected from the file extension if possible.
    pub fn analyze_file(&self, path: &Path) -> Result<AnalyzerResult, AnalyzerError> {
        let contents = std::fs::read(path)?;
        let language = self
            .detect_language_from_path(path)
            .ok_or_else(|| AnalyzerError::UnsupportedLanguage(path.display().to_string()))?;

        self.analyze_language(language, contents, AnalyzeOptions::default())
    }
}
