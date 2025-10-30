# singularity-code-analysis

[![Crates.io](https://img.shields.io/crates/v/singularity-code-analysis.svg)](https://crates.io/crates/singularity-code-analysis)
[![Documentation](https://docs.rs/singularity-code-analysis/badge.svg)](https://docs.rs/singularity-code-analysis)
[![CI](https://github.com/mikkhugo/singularity-incubation/workflows/CI/badge.svg)](https://github.com/mikkhugo/singularity-incubation/actions)

**singularity-code-analysis** is a Rust library to analyze and extract information
from source code written in many different programming languages.
It is based on a parser generator tool and an incremental parsing library
called
<a href="https://tree-sitter.github.io/tree-sitter/" target="_blank">Tree Sitter</a>.

This library is a fork of [Mozilla rust-code-analysis](https://github.com/mozilla/rust-code-analysis)
with enhanced support for BEAM languages (Elixir, Erlang, Gleam) and additional
code complexity metrics.

## Features

- **Multi-language support**: Rust, Python, JavaScript/TypeScript, Java, C/C++, Elixir, Erlang, Gleam, and more
- **Comprehensive metrics**: Cyclomatic complexity, Halstead metrics, Maintainability Index, Lines of Code
- **BEAM language support**: Enhanced parsing and analysis for Elixir, Erlang, and Gleam
- **Tree-sitter integration**: Fast, incremental parsing with Tree-sitter 0.25.10
- **Command-line interface**: Easy-to-use CLI for analyzing codebases
- **Web API**: REST API for web-based code analysis
- **Production-ready**: Comprehensive error handling, extensive test coverage, and CI/CD

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
singularity-code-analysis = "0.1.0"
```

## Quick Start

### Basic Usage

```rust
use singularity_code_analysis::{get_function_spaces, LANG};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Analyze a Rust file
    let source_code = r#"
fn main() {
    println!("Hello, world!");
}

fn calculate(x: i32, y: i32) -> i32 {
    if x > 0 {
        x + y
    } else {
        x - y
    }
}
"#;

    let path = Path::new("example.rs");

    // Get function spaces with error handling
    match get_function_spaces(&LANG::Rust, source_code.as_bytes().to_vec(), &path, None) {
        Some(func_space) => {
            println!("Found {} functions:", func_space.spaces.len());
            for space in &func_space.spaces {
                println!("  - {}: lines {}-{}",
                    space.name.as_ref().unwrap_or(&"unnamed".to_string()),
                    space.start_line,
                    space.end_line
                );
            }

            // Access metrics
            println!("Cyclomatic complexity: {}", func_space.metrics.cyclomatic.cyclomatic);
            println!("Lines of code: {}", func_space.metrics.loc.sloc);
        }
        None => {
            println!("No functions found in the code");
        }
    }

    Ok(())
}
```

### Supported Languages

| Language | Status | Metrics | Function Detection |
|----------|--------|---------|-------------------|
| **Rust** | ✅ Full | ✅ Complete | ✅ Full |
| **Python** | ✅ Full | ✅ Complete | ✅ Full |
| **JavaScript/TypeScript** | ✅ Full | ✅ Complete | ✅ Full |
| **Java** | ✅ Full | ✅ Complete | ✅ Full |
| **C/C++** | ✅ Full | ✅ Complete | ✅ Full |
| **Elixir** | ✅ Full | ⚠️ Partial | ✅ Full |
| **Erlang** | ✅ Full | ⚠️ Partial | ✅ Full |
| **Gleam** | ✅ Full | ⚠️ Partial | ✅ Full |
| **Lua** | ✅ Full | ⚠️ Partial | ✅ Full |

## Metrics

The library computes various software metrics:

### Core Metrics
- **Cyclomatic Complexity**: Measures code complexity based on control flow
- **Halstead Metrics**: Volume, difficulty, effort, and time estimates
- **Maintainability Index**: Overall code maintainability score
- **Lines of Code**: Physical and logical LOC metrics
- **Function Count**: Number of functions and closures

### Advanced Metrics
- **Cognitive Complexity**: Measures human comprehension difficulty
- **Nesting Depth**: Maximum nesting level in functions
- **ABC Metric**: Assignments, Branches, Conditions complexity

## API Reference

### Core Functions

#### `get_function_spaces(lang, source, path, preproc) -> Option<FuncSpace>`

Analyzes source code and returns function spaces with metrics.

**Parameters:**
- `lang`: Language enum (e.g., `LANG::Rust`)
- `source`: Source code as byte vector
- `path`: File path for context
- `preproc`: Optional preprocessor results

**Returns:** Function space data or `None` if no functions found

#### `action<T: Callback>(lang, source, path, preproc, config)`

Runs custom analysis callbacks on source code.

### Data Structures

#### `FuncSpace`
Represents a function or code space with metrics.

```rust
pub struct FuncSpace {
    pub name: Option<String>,
    pub start_line: usize,
    pub end_line: usize,
    pub kind: SpaceKind,
    pub spaces: Vec<FuncSpace>,  // Nested functions
    pub metrics: CodeMetrics,
}
```

#### `CodeMetrics`
Contains all computed metrics for a code space.

```rust
pub struct CodeMetrics {
    pub nargs: nargs::Stats,
    pub nexits: exit::Stats,
    pub cognitive: cognitive::Stats,
    pub cyclomatic: cyclomatic::Stats,
    pub halstead: halstead::Stats,
    pub loc: loc::Stats,
    pub nom: nom::Stats,
    // ... additional metrics
}
```

## Error Handling

The library uses `Option` and `Result` types for error handling:

- Functions return `Option<T>` when "no data" is a valid result
- Parsing errors are handled gracefully
- Invalid input returns `None` rather than panicking

## Performance

- **Fast parsing**: Tree-sitter provides incremental parsing
- **Memory efficient**: Minimal allocations for large codebases
- **Concurrent processing**: Support for parallel analysis

## Building from Source

To build the `singularity-code-analysis` library:

```console
cargo build --release
```

To build with CLI:

```console
cargo build --release --bin singularity-code-analysis-cli
```

## Testing

Run the comprehensive test suite:

```console
cargo test
```

Run specific language tests:

```console
cargo test --lib -- cognitive  # Test cognitive complexity
cargo test --lib -- halstead   # Test Halstead metrics
```

## BEAM Language Enhancements

This fork includes special enhancements for BEAM languages:

- **Elixir**: Improved macro analysis and module structure detection
- **Erlang**: Enhanced function clause and pattern matching analysis
- **Gleam**: Support for modern functional programming constructs

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/mikkhugo/singularity-incubation.git
cd singularity-incubation/packages/singularity-analysis

# Run tests
cargo test

# Run benchmarks
cargo bench

# Check code quality
cargo clippy
cargo fmt --check
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

This library is a fork of [Mozilla rust-code-analysis](https://github.com/mozilla/rust-code-analysis).
Special thanks to the Mozilla team for their excellent work on code analysis tools.
