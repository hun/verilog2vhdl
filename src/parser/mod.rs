// Parser module re-exports

mod ast;
mod lexer;
mod verilog;

#[allow(unused_imports)]
pub use ast::*;
pub use verilog::*;

// Re-export the lexer for use by the parser
pub use lexer::extract_comments;
