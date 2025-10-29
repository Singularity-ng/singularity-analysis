//! Semantic Code Analysis for AI/LLM Systems
//! 
//! Provides semantic understanding of code through embeddings,
//! pattern recognition, and intelligent analysis.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::node::Node;
use crate::langs::LANG;

/// Semantic analyzer for code understanding
#[derive(Debug, Clone)]
pub struct SemanticAnalyzer {
    /// Code embeddings for similarity search
    code_vectors: HashMap<String, Vec<f32>>,
    /// Similarity threshold for pattern matching
    similarity_threshold: f32,
    /// Language-specific patterns
    language_patterns: HashMap<LANG, Vec<CodePattern>>,
}

/// Code pattern representation
#[derive(Debug, Clone)]
pub struct CodePattern {
    pub name: String,
    pub description: String,
    pub pattern_type: PatternType,
    pub complexity_score: f32,
    pub language: LANG,
    pub example: String,
}

/// Types of code patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    DesignPattern,
    AntiPattern,
    CodeSmell,
    BestPractice,
    RefactoringOpportunity,
}

/// Code smell detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSmell {
    pub name: String,
    pub description: String,
    pub severity: Severity,
    pub location: CodeLocation,
    pub suggestion: String,
}

/// Refactoring suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringSuggestion {
    pub name: String,
    pub description: String,
    pub priority: Priority,
    pub effort: EffortLevel,
    pub benefits: Vec<String>,
    pub code_example: String,
}

/// Code location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLocation {
    pub file_path: String,
    pub line_start: usize,
    pub line_end: usize,
    pub column_start: usize,
    pub column_end: usize,
}

/// Severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

/// Effort levels for refactoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer
    pub fn new() -> Self {
        Self {
            code_vectors: HashMap::new(),
            similarity_threshold: 0.8,
            language_patterns: HashMap::new(),
        }
    }

    /// Create with custom similarity threshold
    pub fn with_threshold(threshold: f32) -> Self {
        Self {
            code_vectors: HashMap::new(),
            similarity_threshold: threshold,
            language_patterns: HashMap::new(),
        }
    }

    /// Generate embeddings for code blocks
    /// This is a simplified implementation - in production, you'd use
    /// a proper embedding model like sentence-transformers or OpenAI embeddings
    #[inline(always)]
    pub fn embed_code(&self, code: &str) -> Vec<f32> {
        // Simplified embedding generation based on character frequency
        // In production, replace with actual embedding model
        let mut embedding = vec![0.0; 128]; // 128-dimensional embedding
        
        for (i, ch) in code.chars().enumerate() {
            if i < 128 {
                embedding[i] = (ch as u32) as f32 / 127.0; // Normalize to 0-1
            }
        }
        
        // Add some semantic features
        let lines = code.lines().count() as f32;
        let functions = code.matches("fn ").count() as f32;
        let classes = code.matches("class ").count() as f32;
        
        // Add these as additional dimensions
        if embedding.len() > 100 {
            embedding[100] = lines / 100.0; // Normalize line count
        }
        if embedding.len() > 101 {
            embedding[101] = functions / 10.0; // Normalize function count
        }
        if embedding.len() > 102 {
            embedding[102] = classes / 5.0; // Normalize class count
        }
        
        embedding
    }

    /// Find semantically similar code patterns
    pub fn find_similar_patterns(&self, query: &str) -> Vec<CodePattern> {
        let query_embedding = self.embed_code(query);
        let mut similar_patterns = Vec::new();
        
        // Calculate similarity with stored patterns
        for (pattern_id, pattern_embedding) in &self.code_vectors {
            let similarity = self.cosine_similarity(&query_embedding, pattern_embedding);
            
            if similarity >= self.similarity_threshold {
                // In a real implementation, you'd retrieve the actual pattern
                // from a database using the pattern_id
                similar_patterns.push(CodePattern {
                    name: format!("Pattern_{}", pattern_id),
                    description: "Similar pattern found".to_string(),
                    pattern_type: PatternType::DesignPattern,
                    complexity_score: similarity,
                    language: LANG::Rust, // Default language
                    example: query.to_string(),
                });
            }
        }
        
        // Sort by similarity score
        similar_patterns.sort_by(|a, b| b.complexity_score.partial_cmp(&a.complexity_score).unwrap());
        similar_patterns
    }

    /// Detect code smells and anti-patterns
    pub fn detect_code_smells(&self, code: &str) -> Vec<CodeSmell> {
        let mut code_smells = Vec::new();
        
        // Detect long functions (more than 50 lines)
        let lines = code.lines().count();
        if lines > 50 {
            code_smells.push(CodeSmell {
                name: "Long Function".to_string(),
                description: format!("Function has {} lines, consider breaking it down", lines),
                severity: Severity::Medium,
                location: CodeLocation {
                    file_path: "unknown".to_string(),
                    line_start: 1,
                    line_end: lines,
                    column_start: 1,
                    column_end: 1,
                },
                suggestion: "Break the function into smaller, more focused functions".to_string(),
            });
        }
        
        // Detect deep nesting (more than 4 levels)
        let nesting_level = self.calculate_nesting_level(code);
        if nesting_level > 4 {
            code_smells.push(CodeSmell {
                name: "Deep Nesting".to_string(),
                description: format!("Code has {} levels of nesting", nesting_level),
                severity: Severity::High,
                location: CodeLocation {
                    file_path: "unknown".to_string(),
                    line_start: 1,
                    line_end: lines,
                    column_start: 1,
                    column_end: 1,
                },
                suggestion: "Refactor to reduce nesting using early returns or guard clauses".to_string(),
            });
        }
        
        // Detect duplicate code patterns
        let duplicates = self.detect_duplicate_code(code);
        for duplicate in duplicates {
            code_smells.push(CodeSmell {
                name: "Duplicate Code".to_string(),
                description: "Similar code blocks detected".to_string(),
                severity: Severity::Medium,
                location: duplicate,
                suggestion: "Extract common code into a reusable function".to_string(),
            });
        }
        
        code_smells
    }

    /// Suggest refactoring opportunities
    pub fn suggest_refactoring(&self, code: &str) -> Vec<RefactoringSuggestion> {
        let mut suggestions = Vec::new();
        
        // Suggest extracting long functions
        let lines = code.lines().count();
        if lines > 30 {
            suggestions.push(RefactoringSuggestion {
                name: "Extract Method".to_string(),
                description: "Function is too long and should be broken down".to_string(),
                priority: Priority::High,
                effort: EffortLevel::Medium,
                benefits: vec![
                    "Improved readability".to_string(),
                    "Better testability".to_string(),
                    "Reduced complexity".to_string(),
                ],
                code_example: "// Extract logic into smaller functions".to_string(),
            });
        }
        
        // Suggest reducing nesting
        let nesting_level = self.calculate_nesting_level(code);
        if nesting_level > 3 {
            suggestions.push(RefactoringSuggestion {
                name: "Reduce Nesting".to_string(),
                description: "Deep nesting makes code hard to read and maintain".to_string(),
                priority: Priority::Medium,
                effort: EffortLevel::Low,
                benefits: vec![
                    "Improved readability".to_string(),
                    "Easier to test".to_string(),
                    "Reduced cognitive load".to_string(),
                ],
                code_example: "// Use early returns or guard clauses".to_string(),
            });
        }
        
        // Suggest removing duplicate code
        let duplicates = self.detect_duplicate_code(code);
        if !duplicates.is_empty() {
            suggestions.push(RefactoringSuggestion {
                name: "Remove Duplication".to_string(),
                description: "Duplicate code should be extracted into reusable functions".to_string(),
                priority: Priority::Medium,
                effort: EffortLevel::High,
                benefits: vec![
                    "DRY principle".to_string(),
                    "Easier maintenance".to_string(),
                    "Consistent behavior".to_string(),
                ],
                code_example: "// Extract common code into a shared function".to_string(),
            });
        }
        
        suggestions
    }

    /// Calculate cosine similarity between two vectors
    #[inline(always)]
    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }
        
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }

    /// Calculate nesting level in code
    fn calculate_nesting_level(&self, code: &str) -> usize {
        let mut max_nesting = 0;
        let mut current_nesting = 0;
        
        for line in code.lines() {
            let trimmed = line.trim();
            
            // Count opening braces/brackets
            for ch in trimmed.chars() {
                match ch {
                    '{' | '[' | '(' => current_nesting += 1,
                    '}' | ']' | ')' => {
                        if current_nesting > 0 {
                            current_nesting -= 1;
                        }
                    }
                    _ => {}
                }
            }
            
            max_nesting = max_nesting.max(current_nesting);
        }
        
        max_nesting
    }

    /// Detect duplicate code patterns
    fn detect_duplicate_code(&self, code: &str) -> Vec<CodeLocation> {
        let mut duplicates = Vec::new();
        let lines: Vec<&str> = code.lines().collect();
        
        // Simple duplicate detection based on line similarity
        for i in 0..lines.len() {
            for j in (i + 1)..lines.len() {
                if lines[i] == lines[j] && !lines[i].trim().is_empty() {
                    duplicates.push(CodeLocation {
                        file_path: "unknown".to_string(),
                        line_start: i + 1,
                        line_end: i + 1,
                        column_start: 1,
                        column_end: lines[i].len(),
                    });
                }
            }
        }
        
        duplicates
    }

    /// Add a code pattern to the analyzer
    pub fn add_pattern(&mut self, pattern: CodePattern) {
        let embedding = self.embed_code(&pattern.example);
        let pattern_id = format!("{:?}_{}", pattern.language, pattern.name);
        self.code_vectors.insert(pattern_id, embedding);
        
        self.language_patterns
            .entry(pattern.language)
            .or_insert_with(Vec::new)
            .push(pattern);
    }

    /// Get patterns for a specific language
    pub fn get_patterns_for_language(&self, language: LANG) -> Vec<&CodePattern> {
        self.language_patterns
            .get(&language)
            .map(|patterns| patterns.iter().collect())
            .unwrap_or_default()
    }

    /// Update similarity threshold
    pub fn set_similarity_threshold(&mut self, threshold: f32) {
        self.similarity_threshold = threshold.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_code() {
        let analyzer = SemanticAnalyzer::new();
        let code = "fn main() {\n    println!(\"Hello, world!\");\n}";
        let embedding = analyzer.embed_code(code);
        
        assert_eq!(embedding.len(), 128);
        assert!(embedding.iter().all(|&x| x >= 0.0 && x <= 1.0));
    }

    #[test]
    fn test_cosine_similarity() {
        let analyzer = SemanticAnalyzer::new();
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let c = vec![0.0, 1.0, 0.0];
        
        assert_eq!(analyzer.cosine_similarity(&a, &b), 1.0);
        assert_eq!(analyzer.cosine_similarity(&a, &c), 0.0);
    }

    #[test]
    fn test_detect_code_smells() {
        let analyzer = SemanticAnalyzer::new();
        let long_code = "fn long_function() {\n".repeat(60) + "}";
        let smells = analyzer.detect_code_smells(&long_code);
        
        assert!(!smells.is_empty());
        assert!(smells.iter().any(|s| s.name == "Long Function"));
    }

    #[test]
    fn test_suggest_refactoring() {
        let analyzer = SemanticAnalyzer::new();
        let nested_code = r#"
        fn complex_function() {
            if condition1 {
                if condition2 {
                    if condition3 {
                        if condition4 {
                            // Deep nesting
                        }
                    }
                }
            }
        }
        "#;
        let suggestions = analyzer.suggest_refactoring(nested_code);
        
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.name == "Reduce Nesting"));
    }
}
