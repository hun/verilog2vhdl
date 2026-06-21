// Lexer for extracting comments from Verilog source.
//
// Uses pest's grammar to find all comments in the source text via the
// `file` rule. Since the grammar's WHITESPACE is silent, pest
// automatically matches COMMENT tokens at every level of the parse tree.
// We collect them all and deduplicate by span position.

use pest::Parser as PestParser;

use crate::parser::ast::Comment;
use crate::parser::verilog::{Rule, VerilogParser};

/// Parse a comment token into a Comment AST node
pub(super) fn parse_comment_text(text: &str) -> Comment {
    if text.starts_with("//") {
        let content = text[2..].trim().to_string();
        Comment::Line(content)
    } else if text.starts_with("/*") && text.ends_with("*/") {
        let content = &text[2..text.len() - 2];
        Comment::Block(content.trim().to_string())
    } else {
        Comment::Line(text.trim().to_string())
    }
}

/// Extract all unique comments from source text using pest's grammar rules.
///
/// Returns comments sorted by their start position in the source, deduplicated
/// by span (start, end) position. Each entry is (start_byte, end_byte, Comment).
pub fn extract_comments(source: &str) -> Vec<(usize, usize, Comment)> {
    let mut comments = Vec::new();
    let mut seen = std::collections::HashSet::new();

    if let Ok(file_pairs) = VerilogParser::parse(Rule::file, source) {
        for pair in file_pairs {
            collect_comments(&pair, source, &mut comments, &mut seen);
        }
    }

    comments.sort_by_key(|&(pos, _, _)| pos);
    comments
}

/// Recursively collect COMMENT pairs from a pest::Pair and its children.
fn collect_comments<'a>(
    pair: &pest::iterators::Pair<'a, Rule>,
    source: &str,
    comments: &mut Vec<(usize, usize, Comment)>,
    seen: &mut std::collections::HashSet<(usize, usize)>,
) {
    if pair.as_rule() == Rule::COMMENT {
        let span = pair.as_span();
        let start = span.start();
        let end = span.end();

        // Deduplicate by span position — pest may match the same comment
        // at multiple grammar levels.
        if seen.insert((start, end)) {
            let text = span.as_str();
            comments.push((start, end, parse_comment_text(text)));
        }
    }

    for child in pair.clone().into_inner() {
        collect_comments(&child, source, comments, seen);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_line_comments() {
        let source = "// top comment\nmodule foo(input // inline comment\na); endmodule";
        let comments = extract_comments(source);
        assert!(comments.len() >= 2);
        assert!(matches!(&comments[0].2, Comment::Line(text) if text == "top comment"));
    }

    #[test]
    fn test_extract_block_comments() {
        let source = "/* block */\nmodule foo(input a); /* inline */ endmodule";
        let comments = extract_comments(source);
        assert!(comments.len() >= 2);
        assert!(matches!(&comments[0].2, Comment::Block(text) if text == "block"));
    }

    #[test]
    fn test_extract_multiline_block_comment() {
        let source = "/* multi\nline\ncomment */\nmodule foo(input a); endmodule";
        let comments = extract_comments(source);
        assert!(!comments.is_empty());
        assert!(matches!(&comments[0].2, Comment::Block(text) if text.contains("multi")));
    }

    #[test]
    fn test_comments_sorted_by_position() {
        let source = "// first\nmodule foo(\n    // second\n    input a,  // third\n    output b  // fourth\n); endmodule";
        let comments = extract_comments(source);
        assert!(comments.len() >= 4);
        assert!(comments[0].0 < comments[1].0);
        assert!(comments[1].0 < comments[2].0);
        assert!(comments[2].0 < comments[3].0);
    }

    #[test]
    fn test_no_comments() {
        let source = "module foo(input a, output b); endmodule";
        let comments = extract_comments(source);
        assert!(comments.is_empty());
    }

    #[test]
    fn test_deduplication() {
        // A comment before a port appears at multiple grammar levels.
        // Should only appear once.
        let source = "module foo(\n    // port comment\n    input a\n); endmodule";
        let comments = extract_comments(source);
        let port_comment_count = comments.iter()
            .filter(|(_, _, c)| matches!(c, Comment::Line(text) if text.contains("port comment")))
            .count();
        assert_eq!(port_comment_count, 1);
    }

    #[test]
    fn test_empty_source() {
        let comments = extract_comments("");
        assert!(comments.is_empty());
    }
}
