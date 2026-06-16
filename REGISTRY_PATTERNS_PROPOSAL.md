# Language Registry Pattern Signatures - Comprehensive Proposal

## Overview

This document proposes comprehensive pattern signatures for the `singularity-language-registry` to support all three Singularity engines:

1. **Analysis Engine** - Metrics, complexity, semantic analysis
2. **Parser Engine** - AST parsing, dependency extraction, framework detection
3. **Linting Engine** - Quality gates, pattern-based linting, AI detection

## Current vs Proposed Pattern Categories

### Current (v0.1.0)
```rust
pub struct PatternSignatures {
    pub error_handling_syntax: Vec<String>,
    pub async_syntax: Vec<String>,
    pub testing_syntax: Vec<String>,
    pub pattern_matching_syntax: Vec<String>,
    pub module_syntax: Vec<String>,
}
```

### Proposed (Extended)
```rust
pub struct PatternSignatures {
    // === CORE LANGUAGE SYNTAX (existing) ===
    /// Error handling SYNTAX (language keywords)
    /// Examples: Result<, ?, try, except, {:error,
    pub error_handling_syntax: Vec<String>,

    /// Async/concurrency SYNTAX (language keywords)
    /// Examples: async, await, spawn, Task, go, chan
    pub async_syntax: Vec<String>,

    /// Testing SYNTAX (language built-ins)
    /// Examples: #[test], deftest, assert, unittest
    pub testing_syntax: Vec<String>,

    /// Pattern matching SYNTAX
    /// Examples: match, case, when, with, switch
    pub pattern_matching_syntax: Vec<String>,

    /// Module/import SYNTAX
    /// Examples: use, import, require, alias, from
    pub module_syntax: Vec<String>,

    // === EXTENDED PATTERNS (new) ===
    /// Type annotation SYNTAX
    /// Examples: : i32, -> String, :: type(), : Type
    pub type_annotation_syntax: Vec<String>,

    /// Function definition SYNTAX
    /// Examples: fn, def, func, function, defp
    pub function_syntax: Vec<String>,

    /// Class/struct definition SYNTAX
    /// Examples: class, struct, defmodule, interface
    pub class_syntax: Vec<String>,

    /// Comment/documentation SYNTAX
    /// Examples: //, #, ///, @doc, """
    pub comment_syntax: Vec<String>,

    /// Generic/parametric types SYNTAX
    /// Examples: <T>, [T], {T}
    pub generic_syntax: Vec<String>,

    /// Null/nil handling SYNTAX
    /// Examples: Option<, Maybe, null, nil, None
    pub null_handling_syntax: Vec<String>,

    /// Loop constructs SYNTAX
    /// Examples: for, while, loop, each, Enum.map
    pub loop_syntax: Vec<String>,

    /// Conditional SYNTAX
    /// Examples: if, else, cond, unless, switch
    pub conditional_syntax: Vec<String>,

    // === DATABASE PATTERNS ===
    /// Database library patterns (PostgreSQL, MySQL, etc.)
    /// Examples: Ecto., diesel::, sqlalchemy, gorm
    pub database_patterns: Vec<String>,

    /// SQL query patterns
    /// Examples: SELECT, INSERT, UPDATE, DELETE, from(
    pub sql_patterns: Vec<String>,

    // === API/HTTP PATTERNS ===
    /// HTTP client patterns
    /// Examples: fetch(, axios., http.get, HTTPoison
    pub http_client_patterns: Vec<String>,

    /// HTTP server patterns
    /// Examples: @app.route, router., Phoenix.Router
    pub http_server_patterns: Vec<String>,

    /// GraphQL patterns
    /// Examples: query, mutation, subscription, graphql
    pub graphql_patterns: Vec<String>,

    // === FRAMEWORK DETECTION ===
    /// Web framework patterns
    /// Examples: express, phoenix, django, spring
    pub framework_patterns: Vec<String>,

    /// ORM patterns
    /// Examples: Ecto., ActiveRecord, Hibernate, GORM
    pub orm_patterns: Vec<String>,

    // === PACKAGE/DEPENDENCY PATTERNS ===
    /// Package manager file patterns
    /// Examples: package.json, Cargo.toml, mix.exs, requirements.txt
    pub package_files: Vec<String>,

    /// Import statement patterns (more detailed than module_syntax)
    /// Examples: import {, require(, use , alias
    pub import_patterns: Vec<String>,

    // === CODE QUALITY PATTERNS ===
    /// Common code smells
    /// Examples: god class indicators, long parameter lists
    pub code_smell_indicators: Vec<String>,

    /// Performance antipatterns
    /// Examples: N+1 query indicators, nested loops
    pub performance_antipatterns: Vec<String>,

    /// Security antipatterns
    /// Examples: eval(, exec(, raw SQL concatenation
    pub security_antipatterns: Vec<String>,
}
```

## Engine-Specific Pattern Usage

### Analysis Engine Uses:
- ✅ `error_handling_syntax` - Error handling metrics
- ✅ `async_syntax` - Semantic complexity
- ✅ `testing_syntax` - Testability score
- ✅ `pattern_matching_syntax` - Complexity calculator
- ✅ `module_syntax` - Dependency coupling
- ✅ `type_annotation_syntax` - Type safety metrics
- ✅ `database_patterns` - Database enriched metrics
- ✅ `sql_patterns` - PostgreSQL enriched metrics
- ✅ `http_client_patterns` - API integration detection
- ✅ `code_smell_indicators` - Code smell density

### Parser Engine Uses:
- ✅ `module_syntax` - Dependency extraction
- ✅ `import_patterns` - Import analysis
- ✅ `package_files` - Package manager detection
- ✅ `framework_patterns` - Framework identification
- ✅ `orm_patterns` - ORM detection
- ✅ `function_syntax` - AST function extraction
- ✅ `class_syntax` - AST class extraction

### Linting Engine Uses:
- ✅ `security_antipatterns` - Security linting rules
- ✅ `performance_antipatterns` - Performance linting
- ✅ `code_smell_indicators` - Quality gate enforcement
- ✅ `testing_syntax` - Test coverage validation
- ✅ All syntax patterns - AI pattern detection

## Implementation Priority

### Phase 1: Core Syntax (Already Exists - Needs Data)
1. Fill in missing patterns for all 18 languages
2. Mark BEAM languages as `rca_supported: true`

### Phase 2: Extended Syntax
1. Add type_annotation_syntax
2. Add function_syntax
3. Add class_syntax
4. Add comment_syntax
5. Add null_handling_syntax

### Phase 3: Framework & Ecosystem
1. Add database_patterns
2. Add http_client_patterns
3. Add framework_patterns
4. Add orm_patterns

### Phase 4: Quality & Security
1. Add code_smell_indicators
2. Add security_antipatterns
3. Add performance_antipatterns

## Next Steps

1. ✅ Identify pattern categories (complete)
2. ⏳ Draft pattern data for all languages (in progress)
3. ⏳ Update language registry struct
4. ⏳ Update all three engines to use registry patterns
5. ⏳ Add validation tests
6. ⏳ Update registry to v0.2.0

## Language Coverage

All 18+ languages need patterns:
- Rust ✅ (partially complete in v0.1.0)
- C/C++ ⏳
- Go ⏳
- Python ⏳
- JavaScript/TypeScript ⏳
- Java ⏳
- Kotlin ⏳
- C# ⏳
- Elixir ⏳
- Erlang ⏳
- Gleam ⏳
- Lua ⏳
- Bash ⏳
- SQL ⏳
- JSON/YAML/TOML ⏳
- Markdown ⏳
- Dockerfile ⏳
