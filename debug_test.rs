use std::path::PathBuf;
use singularity_code_analysis::*;

fn main() {
    let source_code = "function f(a, b) {
         return a * b;
     }
     function f1(a, b) {
         return a * b;
     }";
    
    let path = PathBuf::from("foo.js");
    let source_as_vec = source_code.as_bytes().to_vec();
    
    let parser = JavascriptParser::new(source_as_vec.clone(), &path, None);
    let root = parser.get_root();
    
    println!("Root node: {}", root.kind());
    
    // Walk the tree and print all nodes
    fn print_tree(node: &Node, depth: usize, code: &[u8]) {
        let indent = "  ".repeat(depth);
        let start = node.start_byte();
        let end = node.end_byte();
        let text = String::from_utf8_lossy(&code[start..end]);
        let text_preview = if text.len() > 50 { format!("{}...", &text[..47]) } else { text.to_string() };
        
        println!("{}{} ({})", indent, node.kind(), text_preview);
        
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                print_tree(&child, depth + 1, code);
            }
        }
    }
    
    print_tree(&root, 0, &source_as_vec);
}
