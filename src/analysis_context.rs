use std::{cell::RefCell, slice};

#[derive(Clone, Copy)]
struct CodeRef {
    ptr: *const u8,
    len: usize,
}

thread_local! {
    static CURRENT_CODE: RefCell<Option<CodeRef>> = RefCell::new(None);
}

/// Guard that clears the current code slice when dropped.
pub(crate) struct CodeGuard;

impl Drop for CodeGuard {
    fn drop(&mut self) {
        clear_current_code();
    }
}

/// Enter a new code analysis context and return a guard that will clear it on drop.
pub(crate) fn enter_code_context(code: &[u8]) -> CodeGuard {
    set_current_code(code);
    CodeGuard
}

fn set_current_code(code: &[u8]) {
    CURRENT_CODE.with(|slot| {
        *slot.borrow_mut() = Some(CodeRef {
            ptr: code.as_ptr(),
            len: code.len(),
        });
    });
}

fn clear_current_code() {
    CURRENT_CODE.with(|slot| {
        slot.borrow_mut().take();
    });
}

/// Execute a closure with access to the current source code slice, if available.
pub(crate) fn with_current_code<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&[u8]) -> R,
{
    CURRENT_CODE.with(|slot| {
        slot.borrow().map(|code_ref| {
            // SAFETY:
            // The pointer stored in `CodeRef` was created from a slice that
            // outlives the analysis context. The guard returned by
            // `enter_code_context` clears the stored pointer before the
            // underlying slice is dropped, so this conversion is safe.
            let slice = unsafe { slice::from_raw_parts(code_ref.ptr, code_ref.len) };
            f(slice)
        })
    })
}

/// Compare the textual content of a node with any of the provided keywords.
pub(crate) fn node_text_equals_any(node: &crate::Node, keywords: &[&str]) -> bool {
    with_current_code(|code| {
        let start = node.start_byte();
        let end = node.end_byte();
        if end <= code.len() {
            let text = &code[start..end];
            keywords.iter().any(|kw| text == kw.as_bytes())
        } else {
            false
        }
    })
    .unwrap_or(false)
}

/// Helper to fetch the textual content of a node as UTF-8 when a code context is active.
pub(crate) fn node_text<'a>(node: &crate::Node, code: &'a [u8]) -> Option<&'a str> {
    std::str::from_utf8(&code[node.start_byte()..node.end_byte()]).ok()
}
