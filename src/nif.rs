//! NIF (Native Implemented Functions) for Elixir integration
//! 
//! This module provides Rustler NIF functions that expose the SCA library
//! functionality to Elixir, following the "Rust calculates, Elixir orchestrates" pattern.

use rustler::{Encoder, Env, Error, Term};
use serde_json;
use std::collections::HashMap;

use crate::ai::*;
use crate::langs::LANG;

/// Calculate AI-optimized complexity score for learning
#[rustler::nif]
pub fn calculate_ai_complexity_score(code: String, language_hint: String) -> Result<f64, Error> {
    let language = parse_language_hint(&language_hint);
    Ok(calculate_ai_complexity_score(&code, language))
}

/// Extract complexity features from code
#[rustler::nif]
pub fn extract_complexity_features(code: String, language_hint: String) -> Result<HashMap<String, serde_json::Value>, Error> {
    let language = parse_language_hint(&language_hint);
    let features = extract_complexity_features(&code, language);
    
    let mut result = HashMap::new();
    result.insert("total_lines".to_string(), serde_json::Value::Number(features.total_lines.into()));
    result.insert("non_empty_lines".to_string(), serde_json::Value::Number(features.non_empty_lines.into()));
    result.insert("function_count".to_string(), serde_json::Value::Number(features.function_count.into()));
    result.insert("control_flow_count".to_string(), serde_json::Value::Number(features.control_flow_count.into()));
    result.insert("nesting_depth".to_string(), serde_json::Value::Number(features.nesting_depth.into()));
    result.insert("operator_count".to_string(), serde_json::Value::Number(features.operator_count.into()));
    result.insert("comment_ratio".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(features.comment_ratio).unwrap()));
    result.insert("identifier_length_avg".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(features.identifier_length_avg).unwrap()));
    result.insert("cyclomatic_complexity".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(features.cyclomatic_complexity).unwrap()));
    
    Ok(result)
}

/// Calculate code evolution trends
#[rustler::nif]
pub fn calculate_evolution_trends(before_metrics: HashMap<String, serde_json::Value>, after_metrics: HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>, Error> {
    // Convert HashMap to CodeMetrics structs
    let before = hashmap_to_code_metrics(&before_metrics)?;
    let after = hashmap_to_code_metrics(&after_metrics)?;
    
    let (complexity_trend, maintainability_trend, quality_trend) = calculate_evolution_trends(&before, &after);
    
    let mut result = HashMap::new();
    result.insert("complexity_trend".to_string(), serde_json::Value::String(format!("{:?}", complexity_trend)));
    result.insert("maintainability_trend".to_string(), serde_json::Value::String(format!("{:?}", maintainability_trend)));
    result.insert("quality_trend".to_string(), serde_json::Value::String(format!("{:?}", quality_trend)));
    
    Ok(result)
}

/// Predict AI-generated code quality
#[rustler::nif]
pub fn predict_ai_code_quality(code_features: HashMap<String, serde_json::Value>, language_hint: String, model_name: String) -> Result<HashMap<String, serde_json::Value>, Error> {
    let language = parse_language_hint(&language_hint);
    let features = hashmap_to_code_features(&code_features)?;
    
    let prediction = predict_ai_code_quality(&features, language, &model_name);
    
    let mut result = HashMap::new();
    result.insert("predicted_quality".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(prediction.predicted_quality.overall).unwrap()));
    result.insert("confidence".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(prediction.confidence).unwrap()));
    result.insert("risk_factors".to_string(), serde_json::Value::Array(
        prediction.risk_factors.iter().map(|rf| {
            let mut rf_map = HashMap::new();
            rf_map.insert("factor_type".to_string(), serde_json::Value::String(format!("{:?}", rf.factor_type)));
            rf_map.insert("severity".to_string(), serde_json::Value::String(format!("{:?}", rf.severity)));
            rf_map.insert("description".to_string(), serde_json::Value::String(rf.description.clone()));
            serde_json::Value::Object(serde_json::Map::from_iter(rf_map))
        }).collect()
    ));
    
    Ok(result)
}

/// Calculate pattern effectiveness for AI learning
#[rustler::nif]
pub fn calculate_pattern_effectiveness(pattern: String, metrics: HashMap<String, serde_json::Value>) -> Result<f64, Error> {
    let features = hashmap_to_complexity_features(&metrics)?;
    Ok(calculate_pattern_effectiveness(&pattern, &features))
}

/// Calculate supervision complexity for BEAM languages
#[rustler::nif]
pub fn calculate_supervision_complexity(modules: Vec<String>) -> Result<f64, Error> {
    Ok(calculate_supervision_complexity(&modules))
}

/// Calculate actor complexity for BEAM languages
#[rustler::nif]
pub fn calculate_actor_complexity(functions: Vec<String>) -> Result<f64, Error> {
    Ok(calculate_actor_complexity(&functions))
}

/// Parse language hint string to LANG enum
fn parse_language_hint(hint: &str) -> LANG {
    match hint.to_lowercase().as_str() {
        "elixir" => LANG::Elixir,
        "rust" => LANG::Rust,
        "python" => LANG::Python,
        "javascript" | "js" => LANG::Javascript,
        "typescript" | "ts" => LANG::Typescript,
        "java" => LANG::Java,
        "cpp" | "c++" => LANG::Cpp,
        "c" => LANG::C,
        "go" | "golang" => LANG::Go,
        "erlang" => LANG::Erlang,
        "gleam" => LANG::Gleam,
        "lua" => LANG::Lua,
        _ => LANG::Rust, // Default fallback
    }
}

/// Convert HashMap to CodeMetrics struct
fn hashmap_to_code_metrics(map: &HashMap<String, serde_json::Value>) -> Result<CodeMetrics, Error> {
    Ok(CodeMetrics {
        cyclomatic_complexity: map.get("cyclomatic_complexity")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        maintainability_index: map.get("maintainability_index")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        lines_of_code: map.get("lines_of_code")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize,
        cognitive_complexity: map.get("cognitive_complexity")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        halstead_difficulty: map.get("halstead_difficulty")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
    })
}

/// Convert HashMap to CodeFeatures struct
fn hashmap_to_code_features(map: &HashMap<String, serde_json::Value>) -> Result<CodeFeatures, Error> {
    Ok(CodeFeatures {
        lines_of_code: map.get("lines_of_code")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize,
        cyclomatic_complexity: map.get("cyclomatic_complexity")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        cognitive_complexity: map.get("cognitive_complexity")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        nesting_depth: map.get("nesting_depth")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize,
        function_count: map.get("function_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize,
        comment_ratio: map.get("comment_ratio")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        identifier_length_avg: map.get("identifier_length_avg")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        complexity_level: map.get("complexity_level")
            .and_then(|v| v.as_str())
            .map(|s| match s {
                "simple" => ComplexityLevel::Simple,
                "medium" => ComplexityLevel::Medium,
                "complex" => ComplexityLevel::Complex,
                _ => ComplexityLevel::Medium,
            })
            .unwrap_or(ComplexityLevel::Medium),
    })
}

/// Convert HashMap to ComplexityFeatures struct
fn hashmap_to_complexity_features(map: &HashMap<String, serde_json::Value>) -> Result<ComplexityFeatures, Error> {
    Ok(ComplexityFeatures {
        total_lines: map.get("total_lines")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize,
        non_empty_lines: map.get("non_empty_lines")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize,
        function_count: map.get("function_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize,
        control_flow_count: map.get("control_flow_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize,
        nesting_depth: map.get("nesting_depth")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize,
        operator_count: map.get("operator_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize,
        comment_ratio: map.get("comment_ratio")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        identifier_length_avg: map.get("identifier_length_avg")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        cyclomatic_complexity: map.get("cyclomatic_complexity")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
    })
}

rustler::init!(
    "Elixir.Singularity.CodeAnalyzer.Native",
    [
        calculate_ai_complexity_score,
        extract_complexity_features,
        calculate_evolution_trends,
        predict_ai_code_quality,
        calculate_pattern_effectiveness,
        calculate_supervision_complexity,
        calculate_actor_complexity
    ]
);