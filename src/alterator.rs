use crate::*;

/// A trait to create a richer `AST` node for a programming language, mainly
/// thought to be sent on the network.
pub trait Alterator
where
    Self: Checker,
{
    /// Creates a new `AST` node containing the code associated to the node,
    /// its span, and its children.
    ///
    /// This function can be overloaded according to the needs of each
    /// programming language.
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        Self::get_default(node, code, span, children)
    }

    /// Gets the code as text and the span associated to a node.
    fn get_text_span(node: &Node, code: &[u8], span: bool, text: bool) -> (String, Span) {
        let text = if text {
            String::from_utf8(code[node.start_byte()..node.end_byte()].to_vec()).unwrap()
        } else {
            "".to_string()
        };
        if span {
            let (spos_row, spos_column) = node.start_position();
            let (epos_row, epos_column) = node.end_position();
            (
                text,
                Some((spos_row + 1, spos_column + 1, epos_row + 1, epos_column + 1)),
            )
        } else {
            (text, None)
        }
    }

    /// Gets a default `AST` node containing the code associated to the node,
    /// its span, and its children.
    fn get_default(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        let (text, span) = Self::get_text_span(node, code, span, node.child_count() == 0);
        AstNode::new(node.kind(), text, span, children)
    }

    /// Gets a new `AST` node if and only if the code is not a comment,
    /// otherwise [`None`] is returned.
    fn get_ast_node(
        node: &Node,
        code: &[u8],
        children: Vec<AstNode>,
        span: bool,
        comment: bool,
    ) -> Option<AstNode> {
        if comment && Self::is_comment(node) {
            None
        } else {
            Some(Self::alterate(node, code, span, children))
        }
    }
}

// Singularity custom parsers - delegate to standard parsers for compatibility
impl Alterator for PreprocCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        CppCode::alterate(node, code, span, children)
    }
}

impl Alterator for CcommentCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        CppCode::alterate(node, code, span, children)
    }
}

impl Alterator for CppCode {
    fn alterate(node: &Node, code: &[u8], span: bool, mut children: Vec<AstNode>) -> AstNode {
        match Cpp::from(node.kind_id()) {
            Cpp::StringLiteral | Cpp::CharLiteral => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            Cpp::PreprocDef | Cpp::PreprocFunctionDef | Cpp::PreprocCall => {
                if let Some(last) = children.last() {
                    if last.r#type == "\n" {
                        children.pop();
                    }
                }
                Self::get_default(node, code, span, children)
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

impl Alterator for PythonCode {}

impl Alterator for JavaCode {}

// Singularity custom MozjsCode parser - delegate to standard JavascriptCode parser
impl Alterator for MozjsCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        JavascriptCode::alterate(node, code, span, children)
    }
}

impl Alterator for JavascriptCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        match Javascript::from(node.kind_id()) {
            Javascript::String => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

impl Alterator for TypescriptCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        match Typescript::from(node.kind_id()) {
            Typescript::String => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

impl Alterator for TsxCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        match Tsx::from(node.kind_id()) {
            Tsx::String => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

impl Alterator for RustCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        match Rust::from(node.kind_id()) {
            Rust::StringLiteral | Rust::CharLiteral => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

// BEAM languages - Elixir, Erlang, Gleam (minimal implementations)
impl Alterator for ElixirCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        match Elixir::from(node.kind_id()) {
            Elixir::String => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

impl Alterator for ErlangCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        match Erlang::from(node.kind_id()) {
            Erlang::Atom | Erlang::Char => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

impl Alterator for GleamCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        match Gleam::from(node.kind_id()) {
            Gleam::String => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

impl Alterator for LuaCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        match Lua::from(node.kind_id()) {
            Lua::String => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

impl Alterator for GoCode {}

impl Alterator for CsharpCode {}
