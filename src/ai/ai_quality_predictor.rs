//! AI-Generated Code Quality Prediction
//! 
//! Predicts the quality of AI-generated code before it's written,
//! helping AI systems make better generation decisions.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::langs::LANG;

/// Predicts quality of AI-generated code before generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AICodeQualityPredictor {
    /// Historical quality data for pattern matching
    quality_patterns: HashMap<String, QualityPattern>,
    /// Language-specific quality baselines
    language_baselines: HashMap<LANG, QualityBaseline>,
    /// Model-specific performance data
    model_performance: HashMap<String, ModelPerformance>,
}

/// A quality pattern learned from historical data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityPattern {
    pub pattern_id: String,
    pub pattern_type: QualityPatternType,
    pub code_features: CodeFeatures,
    pub expected_quality: QualityScore,
    pub confidence: f64,
    pub success_rate: f64,
    pub failure_reasons: Vec<String>,
}

/// Types of quality patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityPatternType {
    FunctionGeneration,
    ClassGeneration,
    ErrorHandling,
    DataStructure,
    Algorithm,
    DesignPattern,
    TestGeneration,
    Documentation,
}

/// Code features that influence quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFeatures {
    pub complexity_level: ComplexityLevel,
    pub language: LANG,
    pub function_count: u32,
    pub class_count: u32,
    pub nesting_depth: u32,
    pub parameter_count: u32,
    pub return_type_complexity: f64,
    pub error_handling_present: bool,
    pub documentation_present: bool,
    pub test_coverage: f64,
    pub naming_convention_score: f64,
    pub design_pattern_usage: Vec<String>,
}

/// Complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Simple,
    Medium,
    Complex,
    VeryComplex,
}

/// Quality score prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityScore {
    pub overall_score: f64,
    pub maintainability: f64,
    pub readability: f64,
    pub testability: f64,
    pub performance: f64,
    pub security: f64,
    pub reliability: f64,
}

/// Language-specific quality baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityBaseline {
    pub language: LANG,
    pub average_complexity: f64,
    pub average_maintainability: f64,
    pub average_readability: f64,
    pub best_practices: Vec<String>,
    pub common_anti_patterns: Vec<String>,
    pub quality_thresholds: QualityThresholds,
}

/// Quality thresholds for different languages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityThresholds {
    pub min_maintainability: f64,
    pub min_readability: f64,
    pub max_complexity: f64,
    pub min_test_coverage: f64,
}

/// Model performance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPerformance {
    pub model_name: String,
    pub language_performance: HashMap<LANG, f64>,
    pub pattern_success_rates: HashMap<String, f64>,
    pub average_quality_score: f64,
    pub common_failure_modes: Vec<String>,
}

/// Quality prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityPrediction {
    pub predicted_quality: QualityScore,
    pub confidence_score: f64,
    pub risk_factors: Vec<RiskFactor>,
    pub improvement_suggestions: Vec<String>,
    pub alternative_approaches: Vec<AlternativeApproach>,
    pub expected_issues: Vec<ExpectedIssue>,
}

/// Risk factors that could affect quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor_type: RiskFactorType,
    pub description: String,
    pub severity: RiskSeverity,
    pub mitigation: String,
}

/// Types of risk factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskFactorType {
    HighComplexity,
    PoorNaming,
    MissingErrorHandling,
    InsufficientDocumentation,
    LowTestability,
    PerformanceIssues,
    SecurityVulnerabilities,
    MaintainabilityConcerns,
}

/// Risk severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Alternative approach suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeApproach {
    pub approach_name: String,
    pub description: String,
    pub expected_quality: QualityScore,
    pub implementation_effort: EffortLevel,
    pub benefits: Vec<String>,
}

/// Effort levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Expected issues in generated code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedIssue {
    pub issue_type: IssueType,
    pub description: String,
    pub probability: f64,
    pub impact: IssueImpact,
    pub prevention: String,
}

/// Types of expected issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    LogicError,
    PerformanceBottleneck,
    SecurityVulnerability,
    MaintainabilityIssue,
    TestabilityProblem,
    DocumentationGap,
}

/// Issue impact levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueImpact {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for AICodeQualityPredictor {
    fn default() -> Self {
        Self::new()
    }
}

impl AICodeQualityPredictor {
    /// Create a new AI code quality predictor
    pub fn new() -> Self {
        let mut predictor = Self {
            quality_patterns: HashMap::new(),
            language_baselines: HashMap::new(),
            model_performance: HashMap::new(),
        };
        
        // Initialize with default language baselines
        predictor.initialize_language_baselines();
        predictor.initialize_quality_patterns();
        
        predictor
    }

    /// Predict quality of AI-generated code before generation
    pub fn predict_quality(&self, 
                          code_spec: &CodeSpecification, 
                          model_name: &str,
                          language: LANG) -> QualityPrediction {
        let code_features = self.extract_features_from_spec(code_spec, language);
        let baseline = self.get_language_baseline(language);
        let model_perf = self.get_model_performance(model_name);
        
        let predicted_quality = self.calculate_predicted_quality(&code_features, baseline, model_perf);
        let confidence_score = self.calculate_confidence(&code_features, model_perf);
        let risk_factors = self.identify_risk_factors(&code_features, baseline);
        let improvement_suggestions = self.generate_improvement_suggestions(&code_features, baseline);
        let alternative_approaches = self.suggest_alternatives(&code_features, language);
        let expected_issues = self.predict_issues(&code_features, model_perf);
        
        QualityPrediction {
            predicted_quality,
            confidence_score,
            risk_factors,
            improvement_suggestions,
            alternative_approaches,
            expected_issues,
        }
    }

    /// Learn from successful code generation patterns
    pub fn learn_from_success(&mut self, 
                             code_features: &CodeFeatures, 
                             actual_quality: &QualityScore,
                             model_name: &str) {
        let pattern_id = self.generate_pattern_id(&code_features);
        
        if let Some(pattern) = self.quality_patterns.get_mut(&pattern_id) {
            // Update existing pattern with new success data
            pattern.success_rate = (pattern.success_rate + 1.0) / 2.0;
            pattern.expected_quality = self.average_quality_scores(&[&pattern.expected_quality, actual_quality]);
            pattern.confidence = (pattern.confidence + 0.1).min(1.0);
        } else {
            // Create new pattern
            let new_pattern = QualityPattern {
                pattern_id: pattern_id.clone(),
                pattern_type: self.classify_pattern_type(&code_features),
                code_features: code_features.clone(),
                expected_quality: actual_quality.clone(),
                confidence: 0.8,
                success_rate: 1.0,
                failure_reasons: Vec::new(),
            };
            self.quality_patterns.insert(pattern_id, new_pattern);
        }
        
        // Update model performance
        self.update_model_performance(model_name, actual_quality, true);
    }

    /// Learn from failed code generation patterns
    pub fn learn_from_failure(&mut self, 
                             code_features: &CodeFeatures, 
                             failure_reason: &str,
                             model_name: &str) {
        let pattern_id = self.generate_pattern_id(&code_features);
        
        if let Some(pattern) = self.quality_patterns.get_mut(&pattern_id) {
            pattern.success_rate = (pattern.success_rate * 0.9).max(0.0);
            pattern.failure_reasons.push(failure_reason.to_string());
            pattern.confidence = (pattern.confidence - 0.1).max(0.0);
        }
        
        // Update model performance
        self.update_model_performance(model_name, &QualityScore::default(), false);
    }

    /// Get quality recommendations for code generation
    pub fn get_generation_recommendations(&self, 
                                        code_spec: &CodeSpecification,
                                        language: LANG) -> GenerationRecommendations {
        let features = self.extract_features_from_spec(code_spec, language);
        let baseline = self.get_language_baseline(language);
        
        GenerationRecommendations {
            recommended_approach: self.recommend_approach(&features, baseline),
            quality_targets: self.suggest_quality_targets(&features, baseline),
            implementation_guidelines: self.generate_guidelines(&features, language),
            testing_strategy: self.suggest_testing_strategy(&features),
            documentation_requirements: self.suggest_documentation(&features),
        }
    }

    // Private helper methods

    fn initialize_language_baselines(&mut self) {
        // Rust baseline
        self.language_baselines.insert(LANG::Rust, QualityBaseline {
            language: LANG::Rust,
            average_complexity: 5.0,
            average_maintainability: 80.0,
            average_readability: 85.0,
            best_practices: vec![
                "Use Result<T, E> for error handling".to_string(),
                "Prefer immutability".to_string(),
                "Use meaningful variable names".to_string(),
                "Write comprehensive tests".to_string(),
            ],
            common_anti_patterns: vec![
                "Using unwrap() in production code".to_string(),
                "Ignoring clippy warnings".to_string(),
                "Not handling errors properly".to_string(),
            ],
            quality_thresholds: QualityThresholds {
                min_maintainability: 70.0,
                min_readability: 75.0,
                max_complexity: 10.0,
                min_test_coverage: 80.0,
            },
        });

        // JavaScript baseline
        self.language_baselines.insert(LANG::JavaScript, QualityBaseline {
            language: LANG::JavaScript,
            average_complexity: 6.0,
            average_maintainability: 75.0,
            average_readability: 80.0,
            best_practices: vec![
                "Use const and let instead of var".to_string(),
                "Handle promises properly".to_string(),
                "Use meaningful function names".to_string(),
                "Write unit tests".to_string(),
            ],
            common_anti_patterns: vec![
                "Using var declarations".to_string(),
                "Callback hell".to_string(),
                "Not handling async errors".to_string(),
            ],
            quality_thresholds: QualityThresholds {
                min_maintainability: 65.0,
                min_readability: 70.0,
                max_complexity: 12.0,
                min_test_coverage: 70.0,
            },
        });

        // Python baseline
        self.language_baselines.insert(LANG::Python, QualityBaseline {
            language: LANG::Python,
            average_complexity: 4.0,
            average_maintainability: 85.0,
            average_readability: 90.0,
            best_practices: vec![
                "Follow PEP 8 style guide".to_string(),
                "Use type hints".to_string(),
                "Write docstrings".to_string(),
                "Use list comprehensions appropriately".to_string(),
            ],
            common_anti_patterns: vec![
                "Not using type hints".to_string(),
                "Overly complex list comprehensions".to_string(),
                "Not handling exceptions".to_string(),
            ],
            quality_thresholds: QualityThresholds {
                min_maintainability: 75.0,
                min_readability: 80.0,
                max_complexity: 8.0,
                min_test_coverage: 85.0,
            },
        });
    }

    fn initialize_quality_patterns(&mut self) {
        // Initialize with some common patterns
        let patterns = vec![
            ("simple_function", QualityPattern {
                pattern_id: "simple_function".to_string(),
                pattern_type: QualityPatternType::FunctionGeneration,
                code_features: CodeFeatures {
                    complexity_level: ComplexityLevel::Simple,
                    language: LANG::Rust,
                    function_count: 1,
                    class_count: 0,
                    nesting_depth: 1,
                    parameter_count: 2,
                    return_type_complexity: 1.0,
                    error_handling_present: true,
                    documentation_present: true,
                    test_coverage: 90.0,
                    naming_convention_score: 0.9,
                    design_pattern_usage: vec![],
                },
                expected_quality: QualityScore {
                    overall_score: 85.0,
                    maintainability: 80.0,
                    readability: 90.0,
                    testability: 85.0,
                    performance: 80.0,
                    security: 85.0,
                    reliability: 90.0,
                },
                confidence: 0.9,
                success_rate: 0.95,
                failure_reasons: vec![],
            }),
        ];

        for (id, pattern) in patterns {
            self.quality_patterns.insert(id.to_string(), pattern);
        }
    }

    fn extract_features_from_spec(&self, spec: &CodeSpecification, language: LANG) -> CodeFeatures {
        // Extract features from code specification
        CodeFeatures {
            complexity_level: self.estimate_complexity_level(spec),
            language,
            function_count: self.estimate_function_count(spec),
            class_count: self.estimate_class_count(spec),
            nesting_depth: self.estimate_nesting_depth(spec),
            parameter_count: self.estimate_parameter_count(spec),
            return_type_complexity: self.estimate_return_type_complexity(spec),
            error_handling_present: self.requires_error_handling(spec),
            documentation_present: spec.requires_documentation,
            test_coverage: spec.expected_test_coverage,
            naming_convention_score: self.assess_naming_convention(spec),
            design_pattern_usage: self.identify_design_patterns(spec),
        }
    }

    fn estimate_complexity_level(&self, spec: &CodeSpecification) -> ComplexityLevel {
        match spec.complexity_hint.as_str() {
            "simple" => ComplexityLevel::Simple,
            "medium" => ComplexityLevel::Medium,
            "complex" => ComplexityLevel::Complex,
            "very_complex" => ComplexityLevel::VeryComplex,
            _ => {
                // Estimate based on other factors
                if spec.expected_function_count > 10 || spec.expected_nesting_depth > 3 {
                    ComplexityLevel::Complex
                } else if spec.expected_function_count > 5 || spec.expected_nesting_depth > 2 {
                    ComplexityLevel::Medium
                } else {
                    ComplexityLevel::Simple
                }
            }
        }
    }

    fn estimate_function_count(&self, spec: &CodeSpecification) -> u32 {
        spec.expected_function_count
    }

    fn estimate_class_count(&self, spec: &CodeSpecification) -> u32 {
        spec.expected_class_count
    }

    fn estimate_nesting_depth(&self, spec: &CodeSpecification) -> u32 {
        spec.expected_nesting_depth
    }

    fn estimate_parameter_count(&self, spec: &CodeSpecification) -> u32 {
        spec.expected_parameter_count
    }

    fn estimate_return_type_complexity(&self, spec: &CodeSpecification) -> f64 {
        match spec.return_type_complexity.as_str() {
            "simple" => 1.0,
            "medium" => 2.0,
            "complex" => 3.0,
            _ => 1.5,
        }
    }

    fn requires_error_handling(&self, spec: &CodeSpecification) -> bool {
        spec.requires_error_handling
    }

    fn assess_naming_convention(&self, spec: &CodeSpecification) -> f64 {
        // Assess based on specification quality
        if spec.description.len() > 50 && spec.description.contains("function") {
            0.8
        } else {
            0.6
        }
    }

    fn identify_design_patterns(&self, spec: &CodeSpecification) -> Vec<String> {
        let mut patterns = Vec::new();
        
        if spec.description.contains("singleton") {
            patterns.push("Singleton".to_string());
        }
        if spec.description.contains("factory") {
            patterns.push("Factory".to_string());
        }
        if spec.description.contains("observer") {
            patterns.push("Observer".to_string());
        }
        
        patterns
    }

    fn get_language_baseline(&self, language: LANG) -> &QualityBaseline {
        self.language_baselines.get(&language)
            .unwrap_or_else(|| self.language_baselines.get(&LANG::Rust).unwrap())
    }

    fn get_model_performance(&self, model_name: &str) -> Option<&ModelPerformance> {
        self.model_performance.get(model_name)
    }

    fn calculate_predicted_quality(&self, 
                                  features: &CodeFeatures, 
                                  baseline: &QualityBaseline,
                                  model_perf: Option<&ModelPerformance>) -> QualityScore {
        let mut quality = QualityScore {
            overall_score: baseline.average_maintainability,
            maintainability: baseline.average_maintainability,
            readability: baseline.average_readability,
            testability: 70.0,
            performance: 75.0,
            security: 80.0,
            reliability: 75.0,
        };

        // Adjust based on features
        match features.complexity_level {
            ComplexityLevel::Simple => {
                quality.maintainability += 10.0;
                quality.readability += 15.0;
            }
            ComplexityLevel::Medium => {
                quality.maintainability += 5.0;
                quality.readability += 5.0;
            }
            ComplexityLevel::Complex => {
                quality.maintainability -= 10.0;
                quality.readability -= 5.0;
            }
            ComplexityLevel::VeryComplex => {
                quality.maintainability -= 20.0;
                quality.readability -= 15.0;
            }
        }

        // Adjust based on error handling
        if features.error_handling_present {
            quality.reliability += 10.0;
            quality.security += 5.0;
        }

        // Adjust based on documentation
        if features.documentation_present {
            quality.readability += 10.0;
            quality.maintainability += 5.0;
        }

        // Adjust based on test coverage
        quality.testability = features.test_coverage;

        // Adjust based on model performance
        if let Some(perf) = model_perf {
            let model_factor = perf.average_quality_score / 100.0;
            quality.overall_score *= model_factor;
        }

        // Calculate overall score
        quality.overall_score = (
            quality.maintainability + 
            quality.readability + 
            quality.testability + 
            quality.performance + 
            quality.security + 
            quality.reliability
        ) / 6.0;

        quality
    }

    fn calculate_confidence(&self, features: &CodeFeatures, model_perf: Option<&ModelPerformance>) -> f64 {
        let mut confidence = 0.7; // Base confidence

        // Increase confidence for simpler code
        match features.complexity_level {
            ComplexityLevel::Simple => confidence += 0.2,
            ComplexityLevel::Medium => confidence += 0.1,
            ComplexityLevel::Complex => confidence -= 0.1,
            ComplexityLevel::VeryComplex => confidence -= 0.2,
        }

        // Increase confidence if we have model performance data
        if model_perf.is_some() {
            confidence += 0.1;
        }

        // Increase confidence for well-documented specifications
        if features.documentation_present {
            confidence += 0.05;
        }

        confidence.min(1.0).max(0.0)
    }

    fn identify_risk_factors(&self, features: &CodeFeatures, baseline: &QualityBaseline) -> Vec<RiskFactor> {
        let mut risks = Vec::new();

        if features.complexity_level == ComplexityLevel::VeryComplex {
            risks.push(RiskFactor {
                factor_type: RiskFactorType::HighComplexity,
                description: "Very complex code may be difficult to maintain".to_string(),
                severity: RiskSeverity::High,
                mitigation: "Consider breaking into smaller, simpler components".to_string(),
            });
        }

        if features.naming_convention_score < 0.7 {
            risks.push(RiskFactor {
                factor_type: RiskFactorType::PoorNaming,
                description: "Poor naming conventions may reduce readability".to_string(),
                severity: RiskSeverity::Medium,
                mitigation: "Use clear, descriptive names for functions and variables".to_string(),
            });
        }

        if !features.error_handling_present {
            risks.push(RiskFactor {
                factor_type: RiskFactorType::MissingErrorHandling,
                description: "Missing error handling may cause runtime failures".to_string(),
                severity: RiskSeverity::High,
                mitigation: "Implement proper error handling and validation".to_string(),
            });
        }

        if !features.documentation_present {
            risks.push(RiskFactor {
                factor_type: RiskFactorType::InsufficientDocumentation,
                description: "Lack of documentation may reduce maintainability".to_string(),
                severity: RiskSeverity::Medium,
                mitigation: "Add comprehensive documentation and comments".to_string(),
            });
        }

        if features.test_coverage < baseline.quality_thresholds.min_test_coverage {
            risks.push(RiskFactor {
                factor_type: RiskFactorType::LowTestability,
                description: "Low test coverage may indicate poor testability".to_string(),
                severity: RiskSeverity::Medium,
                mitigation: "Increase test coverage and improve testability".to_string(),
            });
        }

        risks
    }

    fn generate_improvement_suggestions(&self, features: &CodeFeatures, baseline: &QualityBaseline) -> Vec<String> {
        let mut suggestions = Vec::new();

        if features.complexity_level == ComplexityLevel::VeryComplex {
            suggestions.push("Break down complex logic into smaller, focused functions".to_string());
        }

        if features.nesting_depth > 3 {
            suggestions.push("Reduce nesting depth using early returns or guard clauses".to_string());
        }

        if !features.error_handling_present {
            suggestions.push("Add comprehensive error handling and validation".to_string());
        }

        if !features.documentation_present {
            suggestions.push("Include detailed documentation and code comments".to_string());
        }

        if features.test_coverage < 80.0 {
            suggestions.push("Ensure comprehensive test coverage for all code paths".to_string());
        }

        suggestions
    }

    fn suggest_alternatives(&self, features: &CodeFeatures, language: LANG) -> Vec<AlternativeApproach> {
        let mut alternatives = Vec::new();

        if features.complexity_level == ComplexityLevel::VeryComplex {
            alternatives.push(AlternativeApproach {
                approach_name: "Modular Approach".to_string(),
                description: "Break the complex functionality into smaller, manageable modules".to_string(),
                expected_quality: QualityScore {
                    overall_score: 85.0,
                    maintainability: 90.0,
                    readability: 85.0,
                    testability: 80.0,
                    performance: 75.0,
                    security: 80.0,
                    reliability: 85.0,
                },
                implementation_effort: EffortLevel::Medium,
                benefits: vec![
                    "Easier to maintain".to_string(),
                    "Better testability".to_string(),
                    "Improved readability".to_string(),
                ],
            });
        }

        if !features.error_handling_present {
            alternatives.push(AlternativeApproach {
                approach_name: "Defensive Programming".to_string(),
                description: "Implement comprehensive error handling and input validation".to_string(),
                expected_quality: QualityScore {
                    overall_score: 80.0,
                    maintainability: 75.0,
                    readability: 80.0,
                    testability: 85.0,
                    performance: 70.0,
                    security: 90.0,
                    reliability: 95.0,
                },
                implementation_effort: EffortLevel::Low,
                benefits: vec![
                    "Higher reliability".to_string(),
                    "Better security".to_string(),
                    "Easier debugging".to_string(),
                ],
            });
        }

        alternatives
    }

    fn predict_issues(&self, features: &CodeFeatures, model_perf: Option<&ModelPerformance>) -> Vec<ExpectedIssue> {
        let mut issues = Vec::new();

        if features.complexity_level == ComplexityLevel::VeryComplex {
            issues.push(ExpectedIssue {
                issue_type: IssueType::MaintainabilityIssue,
                description: "High complexity may lead to maintenance difficulties".to_string(),
                probability: 0.8,
                impact: IssueImpact::High,
                prevention: "Simplify the logic and break into smaller functions".to_string(),
            });
        }

        if !features.error_handling_present {
            issues.push(ExpectedIssue {
                issue_type: IssueType::LogicError,
                description: "Missing error handling may cause unexpected failures".to_string(),
                probability: 0.6,
                impact: IssueImpact::Medium,
                prevention: "Add proper error handling and validation".to_string(),
            });
        }

        if features.test_coverage < 70.0 {
            issues.push(ExpectedIssue {
                issue_type: IssueType::TestabilityProblem,
                description: "Low test coverage may indicate untested edge cases".to_string(),
                probability: 0.7,
                impact: IssueImpact::Medium,
                prevention: "Increase test coverage and improve testability".to_string(),
            });
        }

        issues
    }

    fn generate_pattern_id(&self, features: &CodeFeatures) -> String {
        format!("{}_{:?}_{}", 
                features.language, 
                features.complexity_level, 
                features.function_count)
    }

    fn classify_pattern_type(&self, features: &CodeFeatures) -> QualityPatternType {
        if features.function_count > features.class_count {
            QualityPatternType::FunctionGeneration
        } else if features.class_count > 0 {
            QualityPatternType::ClassGeneration
        } else {
            QualityPatternType::FunctionGeneration
        }
    }

    fn average_quality_scores(&self, scores: &[&QualityScore]) -> QualityScore {
        let count = scores.len() as f64;
        QualityScore {
            overall_score: scores.iter().map(|s| s.overall_score).sum::<f64>() / count,
            maintainability: scores.iter().map(|s| s.maintainability).sum::<f64>() / count,
            readability: scores.iter().map(|s| s.readability).sum::<f64>() / count,
            testability: scores.iter().map(|s| s.testability).sum::<f64>() / count,
            performance: scores.iter().map(|s| s.performance).sum::<f64>() / count,
            security: scores.iter().map(|s| s.security).sum::<f64>() / count,
            reliability: scores.iter().map(|s| s.reliability).sum::<f64>() / count,
        }
    }

    fn update_model_performance(&mut self, model_name: &str, quality: &QualityScore, success: bool) {
        let entry = self.model_performance.entry(model_name.to_string())
            .or_insert_with(|| ModelPerformance {
                model_name: model_name.to_string(),
                language_performance: HashMap::new(),
                pattern_success_rates: HashMap::new(),
                average_quality_score: 0.0,
                common_failure_modes: Vec::new(),
            });

        if success {
            entry.average_quality_score = (entry.average_quality_score + quality.overall_score) / 2.0;
        } else {
            entry.average_quality_score = (entry.average_quality_score * 0.9).max(0.0);
        }
    }

    fn recommend_approach(&self, features: &CodeFeatures, baseline: &QualityBaseline) -> String {
        match features.complexity_level {
            ComplexityLevel::Simple => "Use straightforward implementation with clear naming".to_string(),
            ComplexityLevel::Medium => "Consider using design patterns and modular structure".to_string(),
            ComplexityLevel::Complex => "Break into smaller components and use advanced patterns".to_string(),
            ComplexityLevel::VeryComplex => "Consider refactoring into multiple modules or services".to_string(),
        }
    }

    fn suggest_quality_targets(&self, features: &CodeFeatures, baseline: &QualityBaseline) -> QualityScore {
        QualityScore {
            overall_score: baseline.average_maintainability,
            maintainability: baseline.quality_thresholds.min_maintainability,
            readability: baseline.quality_thresholds.min_readability,
            testability: baseline.quality_thresholds.min_test_coverage,
            performance: 80.0,
            security: 85.0,
            reliability: 90.0,
        }
    }

    fn generate_guidelines(&self, features: &CodeFeatures, language: LANG) -> Vec<String> {
        let mut guidelines = Vec::new();
        
        if let Some(baseline) = self.language_baselines.get(&language) {
            guidelines.extend(baseline.best_practices.clone());
        }

        if features.complexity_level == ComplexityLevel::VeryComplex {
            guidelines.push("Use clear separation of concerns".to_string());
            guidelines.push("Implement comprehensive logging".to_string());
        }

        guidelines
    }

    fn suggest_testing_strategy(&self, features: &CodeFeatures) -> String {
        if features.test_coverage < 70.0 {
            "Implement comprehensive unit tests with high coverage".to_string()
        } else {
            "Maintain existing test coverage and add integration tests".to_string()
        }
    }

    fn suggest_documentation(&self, features: &CodeFeatures) -> String {
        if features.documentation_present {
            "Maintain comprehensive documentation and update as needed".to_string()
        } else {
            "Add detailed documentation including API docs and code comments".to_string()
        }
    }
}

impl Default for QualityScore {
    fn default() -> Self {
        Self {
            overall_score: 0.0,
            maintainability: 0.0,
            readability: 0.0,
            testability: 0.0,
            performance: 0.0,
            security: 0.0,
            reliability: 0.0,
        }
    }
}

/// Code specification for quality prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSpecification {
    pub description: String,
    pub complexity_hint: String,
    pub expected_function_count: u32,
    pub expected_class_count: u32,
    pub expected_nesting_depth: u32,
    pub expected_parameter_count: u32,
    pub return_type_complexity: String,
    pub requires_error_handling: bool,
    pub requires_documentation: bool,
    pub expected_test_coverage: f64,
}

/// Generation recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRecommendations {
    pub recommended_approach: String,
    pub quality_targets: QualityScore,
    pub implementation_guidelines: Vec<String>,
    pub testing_strategy: String,
    pub documentation_requirements: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_quality_predictor_creation() {
        let predictor = AICodeQualityPredictor::new();
        assert!(!predictor.language_baselines.is_empty());
        assert!(!predictor.quality_patterns.is_empty());
    }

    #[test]
    fn test_predict_quality() {
        let predictor = AICodeQualityPredictor::new();
        let spec = CodeSpecification {
            description: "A simple function to add two numbers".to_string(),
            complexity_hint: "simple".to_string(),
            expected_function_count: 1,
            expected_class_count: 0,
            expected_nesting_depth: 1,
            expected_parameter_count: 2,
            return_type_complexity: "simple".to_string(),
            requires_error_handling: true,
            requires_documentation: true,
            expected_test_coverage: 90.0,
        };

        let prediction = predictor.predict_quality(&spec, "claude-sonnet-4.5", LANG::Rust);
        assert!(prediction.predicted_quality.overall_score > 0.0);
        assert!(prediction.confidence_score > 0.0);
    }

    #[test]
    fn test_learn_from_success() {
        let mut predictor = AICodeQualityPredictor::new();
        let features = CodeFeatures {
            complexity_level: ComplexityLevel::Simple,
            language: LANG::Rust,
            function_count: 1,
            class_count: 0,
            nesting_depth: 1,
            parameter_count: 2,
            return_type_complexity: 1.0,
            error_handling_present: true,
            documentation_present: true,
            test_coverage: 90.0,
            naming_convention_score: 0.9,
            design_pattern_usage: vec![],
        };
        let quality = QualityScore {
            overall_score: 85.0,
            maintainability: 80.0,
            readability: 90.0,
            testability: 85.0,
            performance: 80.0,
            security: 85.0,
            reliability: 90.0,
        };

        predictor.learn_from_success(&features, &quality, "claude-sonnet-4.5");
        assert!(predictor.quality_patterns.len() > 1); // Should have added a new pattern
    }
}