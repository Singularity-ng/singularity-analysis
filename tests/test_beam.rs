use singularity_code_analysis::{get_function_spaces, LANG};
use std::path::Path;

#[test]
fn test_beam_function_spaces() {
    let elixir_code = r#"
defmodule Test do
  def hello(name) do
    "Hello, #{name}!"
  end
end
"#;

    let erlang_code = r#"
-module(test).
-export([hello/1]).

hello(Name) ->
    "Hello, " ++ Name ++ "!".
"#;

    let gleam_code = r#"
pub fn hello(name: String) -> String {
  "Hello, " <> name <> "!"
}
"#;

    let path_ex = Path::new("test.ex");
    println!("Testing Elixir function spaces:");
    match get_function_spaces(
        &LANG::Elixir,
        elixir_code.as_bytes().to_vec(),
        path_ex,
        None,
    ) {
        Some(func_space) => {
            println!("Success! Found {} spaces", func_space.spaces.len());
            assert!(!func_space.spaces.is_empty());
        }
        None => println!("No function spaces found for Elixir"),
    }

    let path_erl = Path::new("test.erl");
    println!("\nTesting Erlang function spaces:");
    match get_function_spaces(
        &LANG::Erlang,
        erlang_code.as_bytes().to_vec(),
        path_erl,
        None,
    ) {
        Some(func_space) => {
            println!("Success! Found {} spaces", func_space.spaces.len());
            assert!(!func_space.spaces.is_empty());
        }
        None => println!("No function spaces found for Erlang"),
    }

    let path_gleam = Path::new("test.gleam");
    println!("\nTesting Gleam function spaces:");
    match get_function_spaces(
        &LANG::Gleam,
        gleam_code.as_bytes().to_vec(),
        path_gleam,
        None,
    ) {
        Some(func_space) => {
            println!("Success! Found {} spaces", func_space.spaces.len());
            assert!(!func_space.spaces.is_empty());
        }
        None => println!("No function spaces found for Gleam"),
    }
}
