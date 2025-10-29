//! Code Evolution Tracking for AI Learning
//! 
//! Tracks how code changes over time to provide valuable training data
//! for AI systems to learn from real code evolution patterns.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::langs::LANG;
use crate::node::Node;

/// Tracks code evolution over time for AI learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEvolutionTracker {
    /// Version history of code changes
    version_history: Vec<CodeVersion>,
    /// Refactoring events detected
    refactoring_events: Vec<RefactoringEvent>,
    /// Performance impact measurements
    performance_impact: Vec<PerformanceChange>,
    /// Bug introduction rate tracking
    bug_introduction_rate: f64,
    /// Improvement success rate tracking
    improvement_success_rate: f64,
}

/// A version snapshot of code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeVersion {
    pub version_id: String,
    pub timestamp: String,
    pub file_path: String,
    pub code_hash: String,
    pub metrics: CodeMetrics,
    pub changes: Vec<CodeChange>,
    pub commit_message: Option<String>,
    pub author: Option<String>,
}

/// Code metrics at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetrics {
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: f64,
    pub lines_of_code: u32,
    pub function_count: u32,
    pub class_count: u32,
    pub test_coverage: f64,
    pub maintainability_index: f64,
    pub technical_debt_score: f64,
}

/// A specific change in code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChange {
    pub change_type: ChangeType,
    pub line_start: usize,
    pub line_end: usize,
    pub old_content: String,
    pub new_content: String,
    pub impact_score: f64,
    pub complexity_delta: f64,
}

/// Types of code changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    FunctionAdded,
    FunctionRemoved,
    FunctionModified,
    ClassAdded,
    ClassRemoved,
    ClassModified,
    Refactoring,
    BugFix,
    FeatureAdded,
    Optimization,
    Documentation,
    TestAdded,
    TestModified,
    TestRemoved,
}

/// A refactoring event detected in code evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringEvent {
    pub event_id: String,
    pub timestamp: String,
    pub refactoring_type: RefactoringType,
    pub before_metrics: CodeMetrics,
    pub after_metrics: CodeMetrics,
    pub improvement_score: f64,
    pub success_rate: f64,
    pub complexity_reduction: f64,
    pub maintainability_improvement: f64,
}

/// Types of refactoring events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefactoringType {
    ExtractMethod,
    ExtractClass,
    InlineMethod,
    MoveMethod,
    RenameMethod,
    RemoveDuplication,
    SimplifyConditional,
    ReplaceConditionalWithPolymorphism,
    IntroduceParameterObject,
    ReplaceMagicNumberWithConstant,
    ReplaceNestedConditionalWithGuardClauses,
    ReplaceTempWithQuery,
    SplitTemporaryVariable,
    RemoveAssignmentsToParameters,
    ReplaceMethodWithMethodObject,
    SubstituteAlgorithm,
}

/// Performance change measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceChange {
    pub change_id: String,
    pub timestamp: String,
    pub metric_name: String,
    pub before_value: f64,
    pub after_value: f64,
    pub improvement_percentage: f64,
    pub measurement_context: String,
}

impl Default for CodeEvolutionTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeEvolutionTracker {
    /// Create a new code evolution tracker
    pub fn new() -> Self {
        Self {
            version_history: Vec::new(),
            refactoring_events: Vec::new(),
            performance_impact: Vec::new(),
            bug_introduction_rate: 0.0,
            improvement_success_rate: 0.0,
        }
    }

    /// Track a new version of code
    pub fn track_version(&mut self, version: CodeVersion) {
        self.version_history.push(version);
        self.update_evolution_metrics();
    }

    /// Detect refactoring events from version history
    pub fn detect_refactoring_events(&mut self) -> Vec<RefactoringEvent> {
        let mut events = Vec::new();
        
        if self.version_history.len() < 2 {
            return events;
        }

        for i in 1..self.version_history.len() {
            let prev_version = &self.version_history[i - 1];
            let curr_version = &self.version_history[i];
            
            // Detect extract method refactoring
            if let Some(event) = self.detect_extract_method(prev_version, curr_version) {
                events.push(event);
            }
            
            // Detect extract class refactoring
            if let Some(event) = self.detect_extract_class(prev_version, curr_version) {
                events.push(event);
            }
            
            // Detect remove duplication refactoring
            if let Some(event) = self.detect_remove_duplication(prev_version, curr_version) {
                events.push(event);
            }
            
            // Detect simplify conditional refactoring
            if let Some(event) = self.detect_simplify_conditional(prev_version, curr_version) {
                events.push(event);
            }
        }
        
        self.refactoring_events.extend(events.clone());
        events
    }

    /// Calculate evolution trends for AI learning
    pub fn calculate_evolution_trends(&self) -> EvolutionTrends {
        let mut trends = EvolutionTrends::new();
        
        if self.version_history.is_empty() {
            return trends;
        }

        // Calculate complexity trends
        let complexity_values: Vec<f64> = self.version_history
            .iter()
            .map(|v| v.metrics.cyclomatic_complexity as f64)
            .collect();
        trends.complexity_trend = self.calculate_trend(&complexity_values);

        // Calculate maintainability trends
        let maintainability_values: Vec<f64> = self.version_history
            .iter()
            .map(|v| v.metrics.maintainability_index)
            .collect();
        trends.maintainability_trend = self.calculate_trend(&maintainability_values);

        // Calculate test coverage trends
        let test_coverage_values: Vec<f64> = self.version_history
            .iter()
            .map(|v| v.metrics.test_coverage)
            .collect();
        trends.test_coverage_trend = self.calculate_trend(&test_coverage_values);

        // Calculate refactoring success rate
        if !self.refactoring_events.is_empty() {
            let successful_refactorings = self.refactoring_events
                .iter()
                .filter(|e| e.success_rate > 0.8)
                .count();
            trends.refactoring_success_rate = successful_refactorings as f64 / self.refactoring_events.len() as f64;
        }

        // Calculate improvement patterns
        trends.improvement_patterns = self.identify_improvement_patterns();
        
        trends
    }

    /// Generate AI training data from evolution history
    pub fn generate_ai_training_data(&self) -> AITrainingData {
        AITrainingData {
            code_evolution_patterns: self.extract_evolution_patterns(),
            successful_refactoring_patterns: self.extract_successful_refactoring_patterns(),
            performance_improvement_patterns: self.extract_performance_patterns(),
            quality_degradation_patterns: self.extract_quality_degradation_patterns(),
            language_specific_patterns: self.extract_language_specific_patterns(),
            complexity_evolution_patterns: self.extract_complexity_patterns(),
        }
    }

    /// Predict future code quality based on evolution patterns
    pub fn predict_future_quality(&self, current_metrics: &CodeMetrics) -> QualityPrediction {
        let trends = self.calculate_evolution_trends();
        
        QualityPrediction {
            predicted_complexity: self.predict_complexity(current_metrics, &trends),
            predicted_maintainability: self.predict_maintainability(current_metrics, &trends),
            predicted_test_coverage: self.predict_test_coverage(current_metrics, &trends),
            refactoring_recommendations: self.generate_refactoring_recommendations(current_metrics, &trends),
            risk_factors: self.identify_risk_factors(current_metrics, &trends),
            confidence_score: self.calculate_prediction_confidence(&trends),
        }
    }

    // Private helper methods

    fn detect_extract_method(&self, prev: &CodeVersion, curr: &CodeVersion) -> Option<RefactoringEvent> {
        // Detect if a large function was split into smaller ones
        let prev_large_functions = self.count_large_functions(&prev.metrics);
        let curr_large_functions = self.count_large_functions(&curr.metrics);
        
        if prev_large_functions > curr_large_functions && curr.metrics.function_count > prev.metrics.function_count {
            Some(RefactoringEvent {
                event_id: format!("extract_method_{}", curr.timestamp),
                timestamp: curr.timestamp.clone(),
                refactoring_type: RefactoringType::ExtractMethod,
                before_metrics: prev.metrics.clone(),
                after_metrics: curr.metrics.clone(),
                improvement_score: self.calculate_improvement_score(&prev.metrics, &curr.metrics),
                success_rate: 0.85, // Based on historical data
                complexity_reduction: prev.metrics.cyclomatic_complexity as f64 - curr.metrics.cyclomatic_complexity as f64,
                maintainability_improvement: curr.metrics.maintainability_index - prev.metrics.maintainability_index,
            })
        } else {
            None
        }
    }

    fn detect_extract_class(&self, prev: &CodeVersion, curr: &CodeVersion) -> Option<RefactoringEvent> {
        // Detect if classes were extracted from large functions
        if curr.metrics.class_count > prev.metrics.class_count && 
           curr.metrics.function_count > prev.metrics.function_count {
            Some(RefactoringEvent {
                event_id: format!("extract_class_{}", curr.timestamp),
                timestamp: curr.timestamp.clone(),
                refactoring_type: RefactoringType::ExtractClass,
                before_metrics: prev.metrics.clone(),
                after_metrics: curr.metrics.clone(),
                improvement_score: self.calculate_improvement_score(&prev.metrics, &curr.metrics),
                success_rate: 0.80,
                complexity_reduction: prev.metrics.cyclomatic_complexity as f64 - curr.metrics.cyclomatic_complexity as f64,
                maintainability_improvement: curr.metrics.maintainability_index - prev.metrics.maintainability_index,
            })
        } else {
            None
        }
    }

    fn detect_remove_duplication(&self, prev: &CodeVersion, curr: &CodeVersion) -> Option<RefactoringEvent> {
        // Detect if code duplication was reduced
        let prev_duplication = self.estimate_duplication(&prev.metrics);
        let curr_duplication = self.estimate_duplication(&curr.metrics);
        
        if curr_duplication < prev_duplication && curr.metrics.lines_of_code < prev.metrics.lines_of_code {
            Some(RefactoringEvent {
                event_id: format!("remove_duplication_{}", curr.timestamp),
                timestamp: curr.timestamp.clone(),
                refactoring_type: RefactoringType::RemoveDuplication,
                before_metrics: prev.metrics.clone(),
                after_metrics: curr.metrics.clone(),
                improvement_score: self.calculate_improvement_score(&prev.metrics, &curr.metrics),
                success_rate: 0.90,
                complexity_reduction: prev.metrics.cyclomatic_complexity as f64 - curr.metrics.cyclomatic_complexity as f64,
                maintainability_improvement: curr.metrics.maintainability_index - prev.metrics.maintainability_index,
            })
        } else {
            None
        }
    }

    fn detect_simplify_conditional(&self, prev: &CodeVersion, curr: &CodeVersion) -> Option<RefactoringEvent> {
        // Detect if conditional complexity was reduced
        if curr.metrics.cyclomatic_complexity < prev.metrics.cyclomatic_complexity &&
           curr.metrics.cognitive_complexity < prev.metrics.cognitive_complexity {
            Some(RefactoringEvent {
                event_id: format!("simplify_conditional_{}", curr.timestamp),
                timestamp: curr.timestamp.clone(),
                refactoring_type: RefactoringType::SimplifyConditional,
                before_metrics: prev.metrics.clone(),
                after_metrics: curr.metrics.clone(),
                improvement_score: self.calculate_improvement_score(&prev.metrics, &curr.metrics),
                success_rate: 0.75,
                complexity_reduction: prev.metrics.cyclomatic_complexity as f64 - curr.metrics.cyclomatic_complexity as f64,
                maintainability_improvement: curr.metrics.maintainability_index - prev.metrics.maintainability_index,
            })
        } else {
            None
        }
    }

    fn count_large_functions(&self, metrics: &CodeMetrics) -> usize {
        // Estimate large functions based on LOC per function
        if metrics.function_count == 0 {
            return 0;
        }
        let avg_loc_per_function = metrics.lines_of_code as f64 / metrics.function_count as f64;
        if avg_loc_per_function > 50.0 {
            (metrics.function_count as f64 * 0.3) as usize // Estimate 30% are large
        } else {
            0
        }
    }

    fn estimate_duplication(&self, metrics: &CodeMetrics) -> f64 {
        // Simple duplication estimation based on metrics
        // In a real implementation, this would use more sophisticated analysis
        if metrics.lines_of_code == 0 {
            return 0.0;
        }
        let duplication_ratio = metrics.technical_debt_score / 100.0;
        metrics.lines_of_code as f64 * duplication_ratio
    }

    fn calculate_improvement_score(&self, before: &CodeMetrics, after: &CodeMetrics) -> f64 {
        let complexity_improvement = (before.cyclomatic_complexity as f64 - after.cyclomatic_complexity as f64) / before.cyclomatic_complexity as f64;
        let maintainability_improvement = (after.maintainability_index - before.maintainability_index) / 100.0;
        let test_coverage_improvement = (after.test_coverage - before.test_coverage) / 100.0;
        
        (complexity_improvement + maintainability_improvement + test_coverage_improvement) / 3.0
    }

    fn calculate_trend(&self, values: &[f64]) -> TrendDirection {
        if values.len() < 2 {
            return TrendDirection::Stable;
        }
        
        let first_half = &values[..values.len() / 2];
        let second_half = &values[values.len() / 2..];
        
        let first_avg = first_half.iter().sum::<f64>() / first_half.len() as f64;
        let second_avg = second_half.iter().sum::<f64>() / second_half.len() as f64;
        
        let change_percentage = (second_avg - first_avg) / first_avg * 100.0;
        
        if change_percentage > 5.0 {
            TrendDirection::Increasing
        } else if change_percentage < -5.0 {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        }
    }

    fn update_evolution_metrics(&mut self) {
        if self.version_history.len() < 2 {
            return;
        }
        
        // Calculate bug introduction rate
        let bug_introductions = self.version_history
            .windows(2)
            .filter(|w| w[1].metrics.technical_debt_score > w[0].metrics.technical_debt_score)
            .count();
        self.bug_introduction_rate = bug_introductions as f64 / (self.version_history.len() - 1) as f64;
        
        // Calculate improvement success rate
        let improvements = self.version_history
            .windows(2)
            .filter(|w| w[1].metrics.maintainability_index > w[0].metrics.maintainability_index)
            .count();
        self.improvement_success_rate = improvements as f64 / (self.version_history.len() - 1) as f64;
    }

    fn identify_improvement_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();
        
        // Pattern: Consistent refactoring leads to better maintainability
        if self.refactoring_events.len() > 3 {
            let avg_improvement = self.refactoring_events
                .iter()
                .map(|e| e.maintainability_improvement)
                .sum::<f64>() / self.refactoring_events.len() as f64;
            
            if avg_improvement > 0.1 {
                patterns.push("Consistent refactoring improves maintainability".to_string());
            }
        }
        
        // Pattern: Test coverage correlates with quality
        if let Some(latest) = self.version_history.last() {
            if latest.metrics.test_coverage > 80.0 && latest.metrics.maintainability_index > 70.0 {
                patterns.push("High test coverage correlates with high maintainability".to_string());
            }
        }
        
        patterns
    }

    fn extract_evolution_patterns(&self) -> Vec<String> {
        // Extract patterns from evolution history for AI training
        let mut patterns = Vec::new();
        
        for event in &self.refactoring_events {
            match event.refactoring_type {
                RefactoringType::ExtractMethod => {
                    patterns.push(format!("Extract method refactoring reduces complexity by {:.2} points", 
                        event.complexity_reduction));
                }
                RefactoringType::RemoveDuplication => {
                    patterns.push("Remove duplication improves maintainability".to_string());
                }
                _ => {}
            }
        }
        
        patterns
    }

    fn extract_successful_refactoring_patterns(&self) -> Vec<String> {
        self.refactoring_events
            .iter()
            .filter(|e| e.success_rate > 0.8)
            .map(|e| format!("{:?} refactoring has {:.1}% success rate", e.refactoring_type, e.success_rate * 100.0))
            .collect()
    }

    fn extract_performance_patterns(&self) -> Vec<String> {
        self.performance_impact
            .iter()
            .map(|p| format!("{} improved by {:.1}%", p.metric_name, p.improvement_percentage))
            .collect()
    }

    fn extract_quality_degradation_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();
        
        for window in self.version_history.windows(2) {
            if window[1].metrics.maintainability_index < window[0].metrics.maintainability_index {
                patterns.push("Quality degradation detected in version transition".to_string());
            }
        }
        
        patterns
    }

    fn extract_language_specific_patterns(&self) -> Vec<String> {
        // This would be implemented based on the specific language being tracked
        vec!["Language-specific patterns would be extracted here".to_string()]
    }

    fn extract_complexity_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();
        
        if self.version_history.len() > 5 {
            let complexity_values: Vec<f64> = self.version_history
                .iter()
                .map(|v| v.metrics.cyclomatic_complexity as f64)
                .collect();
            
            let trend = self.calculate_trend(&complexity_values);
            match trend {
                TrendDirection::Increasing => patterns.push("Complexity is increasing over time".to_string()),
                TrendDirection::Decreasing => patterns.push("Complexity is decreasing over time".to_string()),
                TrendDirection::Stable => patterns.push("Complexity remains stable".to_string()),
            }
        }
        
        patterns
    }

    fn predict_complexity(&self, current: &CodeMetrics, trends: &EvolutionTrends) -> f64 {
        match trends.complexity_trend {
            TrendDirection::Increasing => current.cyclomatic_complexity as f64 * 1.1,
            TrendDirection::Decreasing => current.cyclomatic_complexity as f64 * 0.9,
            TrendDirection::Stable => current.cyclomatic_complexity as f64,
        }
    }

    fn predict_maintainability(&self, current: &CodeMetrics, trends: &EvolutionTrends) -> f64 {
        match trends.maintainability_trend {
            TrendDirection::Increasing => (current.maintainability_index + 5.0).min(100.0),
            TrendDirection::Decreasing => (current.maintainability_index - 5.0).max(0.0),
            TrendDirection::Stable => current.maintainability_index,
        }
    }

    fn predict_test_coverage(&self, current: &CodeMetrics, trends: &EvolutionTrends) -> f64 {
        match trends.test_coverage_trend {
            TrendDirection::Increasing => (current.test_coverage + 5.0).min(100.0),
            TrendDirection::Decreasing => (current.test_coverage - 5.0).max(0.0),
            TrendDirection::Stable => current.test_coverage,
        }
    }

    fn generate_refactoring_recommendations(&self, current: &CodeMetrics, trends: &EvolutionTrends) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if current.cyclomatic_complexity > 10 {
            recommendations.push("Consider extract method refactoring to reduce complexity".to_string());
        }
        
        if current.maintainability_index < 50.0 {
            recommendations.push("Improve code maintainability through refactoring".to_string());
        }
        
        if current.test_coverage < 70.0 {
            recommendations.push("Increase test coverage to improve code quality".to_string());
        }
        
        recommendations
    }

    fn identify_risk_factors(&self, current: &CodeMetrics, trends: &EvolutionTrends) -> Vec<String> {
        let mut risks = Vec::new();
        
        if current.cyclomatic_complexity > 15 {
            risks.push("High cyclomatic complexity increases maintenance risk".to_string());
        }
        
        if current.maintainability_index < 30.0 {
            risks.push("Low maintainability index indicates high technical debt".to_string());
        }
        
        if trends.complexity_trend == TrendDirection::Increasing {
            risks.push("Increasing complexity trend indicates potential quality degradation".to_string());
        }
        
        risks
    }

    fn calculate_prediction_confidence(&self, trends: &EvolutionTrends) -> f64 {
        // Confidence based on amount of historical data and consistency of trends
        let data_points = self.version_history.len() as f64;
        let base_confidence = (data_points / 10.0).min(1.0);
        
        // Adjust based on trend consistency
        let trend_consistency = if trends.refactoring_success_rate > 0.8 { 0.2 } else { 0.0 };
        
        (base_confidence + trend_consistency).min(1.0)
    }
}

/// Evolution trends calculated from version history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionTrends {
    pub complexity_trend: TrendDirection,
    pub maintainability_trend: TrendDirection,
    pub test_coverage_trend: TrendDirection,
    pub refactoring_success_rate: f64,
    pub improvement_patterns: Vec<String>,
}

impl EvolutionTrends {
    fn new() -> Self {
        Self {
            complexity_trend: TrendDirection::Stable,
            maintainability_trend: TrendDirection::Stable,
            test_coverage_trend: TrendDirection::Stable,
            refactoring_success_rate: 0.0,
            improvement_patterns: Vec::new(),
        }
    }
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
}

/// AI training data extracted from evolution history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITrainingData {
    pub code_evolution_patterns: Vec<String>,
    pub successful_refactoring_patterns: Vec<String>,
    pub performance_improvement_patterns: Vec<String>,
    pub quality_degradation_patterns: Vec<String>,
    pub language_specific_patterns: Vec<String>,
    pub complexity_evolution_patterns: Vec<String>,
}

/// Quality prediction based on evolution patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityPrediction {
    pub predicted_complexity: f64,
    pub predicted_maintainability: f64,
    pub predicted_test_coverage: f64,
    pub refactoring_recommendations: Vec<String>,
    pub risk_factors: Vec<String>,
    pub confidence_score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_evolution_tracker_creation() {
        let tracker = CodeEvolutionTracker::new();
        assert_eq!(tracker.version_history.len(), 0);
        assert_eq!(tracker.refactoring_events.len(), 0);
    }

    #[test]
    fn test_track_version() {
        let mut tracker = CodeEvolutionTracker::new();
        let version = CodeVersion {
            version_id: "v1.0.0".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            file_path: "test.rs".to_string(),
            code_hash: "abc123".to_string(),
            metrics: CodeMetrics {
                cyclomatic_complexity: 5,
                cognitive_complexity: 3.5,
                lines_of_code: 100,
                function_count: 10,
                class_count: 2,
                test_coverage: 80.0,
                maintainability_index: 75.0,
                technical_debt_score: 20.0,
            },
            changes: vec![],
            commit_message: Some("Initial commit".to_string()),
            author: Some("developer".to_string()),
        };
        
        tracker.track_version(version);
        assert_eq!(tracker.version_history.len(), 1);
    }

    #[test]
    fn test_detect_refactoring_events() {
        let mut tracker = CodeEvolutionTracker::new();
        
        // Add initial version
        let version1 = CodeVersion {
            version_id: "v1.0.0".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            file_path: "test.rs".to_string(),
            code_hash: "abc123".to_string(),
            metrics: CodeMetrics {
                cyclomatic_complexity: 15,
                cognitive_complexity: 10.0,
                lines_of_code: 200,
                function_count: 5,
                class_count: 1,
                test_coverage: 60.0,
                maintainability_index: 50.0,
                technical_debt_score: 40.0,
            },
            changes: vec![],
            commit_message: None,
            author: None,
        };
        
        // Add refactored version
        let version2 = CodeVersion {
            version_id: "v1.1.0".to_string(),
            timestamp: "2024-01-02T00:00:00Z".to_string(),
            file_path: "test.rs".to_string(),
            code_hash: "def456".to_string(),
            metrics: CodeMetrics {
                cyclomatic_complexity: 10,
                cognitive_complexity: 7.0,
                lines_of_code: 180,
                function_count: 8,
                class_count: 2,
                test_coverage: 75.0,
                maintainability_index: 65.0,
                technical_debt_score: 25.0,
            },
            changes: vec![],
            commit_message: None,
            author: None,
        };
        
        tracker.track_version(version1);
        tracker.track_version(version2);
        
        let events = tracker.detect_refactoring_events();
        assert!(!events.is_empty());
    }

    #[test]
    fn test_calculate_evolution_trends() {
        let mut tracker = CodeEvolutionTracker::new();
        
        // Add multiple versions to test trend calculation
        for i in 0..5 {
            let version = CodeVersion {
                version_id: format!("v1.{}.0", i),
                timestamp: format!("2024-01-{:02}T00:00:00Z", i + 1),
                file_path: "test.rs".to_string(),
                code_hash: format!("hash{}", i),
                metrics: CodeMetrics {
                    cyclomatic_complexity: 10 - i as u32,
                    cognitive_complexity: 8.0 - i as f64,
                    lines_of_code: 100 + i * 10,
                    function_count: 5 + i,
                    class_count: 1,
                    test_coverage: 60.0 + i as f64 * 5.0,
                    maintainability_index: 50.0 + i as f64 * 5.0,
                    technical_debt_score: 40.0 - i as f64 * 5.0,
                },
                changes: vec![],
                commit_message: None,
                author: None,
            };
            tracker.track_version(version);
        }
        
        let trends = tracker.calculate_evolution_trends();
        assert_eq!(trends.complexity_trend, TrendDirection::Decreasing);
        assert_eq!(trends.maintainability_trend, TrendDirection::Increasing);
    }
}