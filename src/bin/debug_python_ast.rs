// Debug tool to inspect Python AST structure for boolean operators
use singularity_code_analysis::*;

fn main() {
    let code = "if a and b:\n    pass";
    println!("Code:\n{}\n", code);

    // This would need to be implemented as a test or internal function
    // since node is private
    println!("Run: cargo test debug_python_ast -- --nocapture");
}
