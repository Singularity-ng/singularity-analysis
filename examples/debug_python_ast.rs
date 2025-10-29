// Debug tool to inspect Python AST structure for boolean operators
use singularity_code_analysis::*;

fn main() {
    let code = "if a and b:\n    pass";
    println!("Code:\n{}\n", code);

    let tree = node::Tree::new::<PythonParser>(code.as_bytes());
    let root = tree.get_root();

    println!("AST Structure:");
    print_tree(&root, 0);
}

fn print_tree(node: &node::Node, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}[{:3}] {} ({})",
        indent,
        node.kind_id(),
        node.kind(),
        if node.child_count() > 0 { format!("{} children", node.child_count()) } else { "leaf".to_string() }
    );

    for child in node.children() {
        print_tree(&child, depth + 1);
    }
}
