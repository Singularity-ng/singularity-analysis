# singularity-code-analysis

[![Crates.io](https://img.shields.io/crates/v/singularity-code-analysis.svg)](https://crates.io/crates/singularity-code-analysis)

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

## Usage

**singularity-code-analysis** supports many types of programming languages and
computes a great variety of metrics. You can find up to date documentation at
<a href="https://docs.rs/singularity-code-analysis" target="_blank">Documentation</a>.

## Building

To build the `singularity-code-analysis` library, you need to run the following
command:

```console
cargo build
```

If you want to build the `cli`:

```console
cargo build --bin singularity-code-analysis-cli
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
singularity-code-analysis = "0.1.0"
```

## Examples

### Basic Usage

```rust
use singularity_code_analysis::{get_function_spaces, get_metrics};

fn main() {
    // Analyze a Rust file
    let source_code = r#"
fn main() {
    println!("Hello, world!");
}
"#;

    // Get metrics
    let metrics = get_metrics("rust", source_code, "main.rs").unwrap();
    println!("Cyclomatic complexity: {}", metrics.cyclomatic.cyclomatic);

    // Get function spaces
    let spaces = get_function_spaces("rust", source_code, "main.rs").unwrap();
    for space in spaces {
        println!("Function: {}", space.name);
    }
}
```

### Supported Languages

- **Rust** - Full support with advanced metrics
- **Python** - Complete analysis including function complexity
- **JavaScript/TypeScript** - Modern JS/TS support
- **Java** - Object-oriented metrics
- **C/C++** - System programming analysis
- **Elixir** - Enhanced BEAM language support
- **Erlang** - Functional programming metrics
- **Gleam** - Modern functional language analysis
- **And many more...**

## BEAM Language Enhancements

This fork includes special enhancements for BEAM languages:

- **Elixir**: Improved macro analysis and module structure detection
- **Erlang**: Enhanced function clause and pattern matching analysis
- **Gleam**: Support for modern functional programming constructs

## Metrics

The library computes various software metrics:

- **Cyclomatic Complexity**: Measures code complexity based on control flow
- **Halstead Metrics**: Volume, difficulty, effort, and time estimates
- **Maintainability Index**: Overall code maintainability score
- **Lines of Code**: Physical and logical LOC metrics
- **Function Spaces**: Detailed function-level analysis

## Testing

To verify whether all tests pass, run the `cargo test` command.

```console
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

This library is a fork of [Mozilla rust-code-analysis](https://github.com/mozilla/rust-code-analysis).
Special thanks to the Mozilla team for their excellent work on code analysis tools.
