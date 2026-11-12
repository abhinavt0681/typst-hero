//! Syntax highlighting with Typst's own parser.
//!
//! Using the real parser means highlighting is always exactly right, and no
//! separate grammar needs maintaining. The editor (CodeMirror) receives the
//! ranges and paints them.

use typst::syntax::{LinkedNode, highlight, parse};

/// A highlighted range. Offsets are in UTF-16 code units, which is what
/// JavaScript strings (and therefore CodeMirror) use.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    pub from: u32,
    pub to: u32,
    /// CSS class such as `typ-strong` or `typ-heading`.
    pub class: &'static str,
}

pub fn highlight_code(code: &str) -> Vec<Span> {
    let root = parse(code);
    let utf16 = Utf16Index::new(code);
    let mut spans = Vec::new();
    walk(&LinkedNode::new(&root), &utf16, &mut spans);
    spans
}

fn walk(node: &LinkedNode, utf16: &Utf16Index, out: &mut Vec<Span>) {
    if let Some(tag) = highlight(node) {
        let start = node.offset();
        let end = start + node.len();
        if end > start {
            out.push(Span {
                from: utf16.at(start),
                to: utf16.at(end),
                class: tag.css_class(),
            });
        }
    }
    for child in node.children() {
        walk(&child, utf16, out);
    }
}

/// Converts byte offsets (Rust) to UTF-16 offsets (JavaScript).
struct Utf16Index(Vec<u32>);

impl Utf16Index {
    fn new(text: &str) -> Self {
        let mut map = Vec::with_capacity(text.len() + 1);
        let mut pos = 0u32;
        for c in text.chars() {
            for _ in 0..c.len_utf8() {
                map.push(pos);
            }
            pos += c.len_utf16() as u32;
        }
        map.push(pos);
        Self(map)
    }

    fn at(&self, byte: usize) -> u32 {
        self.0[byte]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlights_strong_and_headings() {
        let spans = highlight_code("= Title\nsome *bold* text");
        assert!(
            spans.contains(&Span {
                from: 0,
                to: 7,
                class: "typ-heading"
            }),
            "{spans:?}"
        );
        assert!(
            spans.contains(&Span {
                from: 13,
                to: 19,
                class: "typ-strong"
            }),
            "{spans:?}"
        );
    }

    #[test]
    fn offsets_are_utf16() {
        // "é" is 2 bytes in UTF-8 but 1 unit in UTF-16; "😀" is 4 and 2.
        let spans = highlight_code("é😀 *x*");
        assert!(
            spans.contains(&Span {
                from: 4,
                to: 7,
                class: "typ-strong"
            }),
            "{spans:?}"
        );
    }
}
