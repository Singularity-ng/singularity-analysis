use std::path::Path;
use singularity_code_analysis::{RustParser, metrics};

fn main() {
    let source = r#"fn f(a: bool, b: usize) {
        if a {
            return a;
        }
    }"#;
    
    let path = Path::new("test.rs");
    let parser = RustParser::new(source.as_bytes().to_vec(), &path, None);
    
    println!("Source code:");
    println!("{}", source);
    println!();
    
    println!("Root node kind: {}", parser.get_root().kind());
    println!("Root node has {} children", parser.get_root().child_count());
    
    for i in 0..parser.get_root().child_count() {
        if let Some(child) = parser.get_root().child(i) {
            println!("Child {}: {} ({})", i, child.kind(), child.kind_id());
        }
    }
    
    println!();
    println!("Metrics result:");
    match metrics(&parser, &path) {
        Some(func_space) => {
            println!("Found {} spaces", func_space.spaces.len());
            for space in &func_space.spaces {
                println!("  Space: {} ({:?})", space.name.as_ref().unwrap_or(&"unnamed".to_string()), space.kind);
            }
        }
        None => println!("No function spaces found"),
    }
}
