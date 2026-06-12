// Pest-based Verilog parser implementation

use pest::Parser as PestParser;
use pest::iterators::Pair;
use pest_derive::Parser;
use thiserror::Error;

use super::ast::*;

/// Errors that can occur during parsing
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parse error: {0}")]
    Parse(#[from] pest::error::Error<Rule>),
}

/// The Pest grammar
#[derive(Parser)]
#[grammar = "src/parser/verilog.pest"]
pub struct VerilogParser;

/// Extract text content from a pest::Pair, stripping whitespace
fn pair_text(pair: &Pair<'_, Rule>) -> String {
    pair.as_str().trim().to_string()
}

/// Parse a comment token into a Comment AST node
fn parse_comment_text(text: &str) -> Comment {
    if text.starts_with("//") {
        // Strip the // prefix and trim
        let content = text[2..].trim().to_string();
        Comment::Line(content)
    } else if text.starts_with("/*") && text.ends_with("*/") {
        // Strip /* and */ prefixes
        let content = &text[2..text.len() - 2];
        Comment::Block(content.trim().to_string())
    } else {
        Comment::Line(text.trim().to_string())
    }
}

/// Parse a dimension [MSB:LSB] - dimension is atomic, so strip brackets and split
fn parse_dimension(pair: Pair<'_, Rule>) -> Dimension {
    let text = pair.as_str().trim();
    // text is like "[7:0]" or "[WIDTH-1:0]" — strip brackets
    let inner = &text[1..text.len() - 1]; // remove '[' and ']'
    let mut parts = inner.splitn(2, ':');
    let msb = parts.next().unwrap_or("").trim().to_string();
    let lsb = parts.next().unwrap_or("").trim().to_string();
    Dimension { msb, lsb }
}

/// Parse port direction
fn parse_direction(text: &str) -> PortDirection {
    match text {
        "input" => PortDirection::Input,
        "output" => PortDirection::Output,
        "inout" => PortDirection::Inout,
        "ref" => PortDirection::Ref,
        "const ref" => PortDirection::ConstRef,
        "output ref" => PortDirection::OutputRef,
        "inout ref" => PortDirection::InoutRef,
        "input ref" => PortDirection::Ref, // SV input ref maps to Ref
        _ => panic!("Unknown direction: {}", text),
    }
}

/// Parse port type
fn parse_port_type(text: &str) -> PortType {
    match text {
        "wire" => PortType::Wire,
        "reg" => PortType::Reg,
        "logic" => PortType::Logic,
        "wand" => PortType::Wand,
        "tri" => PortType::Tri,
        "triand" => PortType::Triand,
        "trior" => PortType::Trior,
        "tri0" => PortType::Tri0,
        "tri1" => PortType::Tri1,
        "supply0" => PortType::Supply0,
        "supply1" => PortType::Supply1,
        "integer" => PortType::Integer,
        "time" => PortType::Time,
        "signed" => PortType::Signed,
        "unsigned" => PortType::Unsigned,
        "packed" => PortType::Packed,
        _ => panic!("Unknown port type: {}", text),
    }
}

/// Extract comments from source text with their byte positions
fn extract_comments(source: &str) -> Vec<(usize, usize, String)> {
    // Returns (start_byte, end_byte, comment_text)
    let mut comments = Vec::new();
    let mut pos = 0;

    while pos < source.len() {
        if source[pos..].starts_with("//") {
            // Line comment: find end of line
            let start = pos + 2;
            let end = source[start..]
                .find('\n')
                .map(|i| start + i)
                .unwrap_or(source.len());
            let content = source[start..end].trim().to_string();
            comments.push((pos, end, format!("//{}", content)));
            pos = end;
            continue;
        } else if source[pos..].starts_with("/*") {
            // Block comment: find */
            if let Some(end_tag) = source[pos + 2..].find("*/") {
                let end = pos + 2 + end_tag + 2;
                let content = source[pos + 2..end - 2].trim().to_string();
                comments.push((pos, end, format!("/*{}*/", content)));
                pos = end;
                continue;
            }
        }
        pos += 1;
    }
    comments
}

/// Find leading comments that appear immediately before a position in the source
fn find_leading_comments(
    comments: &[(usize, usize, String)],
    port_start: usize,
    source: &str,
) -> Vec<Comment> {
    // Comments that end before the port starts, and are within reasonable proximity
    let mut result = Vec::new();
    let mut found = false;

    // Iterate backwards from the port start to find the closest comments
    for &(_start, end, ref text) in comments.iter().rev() {
        if end >= port_start {
            // This comment starts at or after the port - skip
            continue;
        }
        // Check if there's only whitespace between this comment and the port
        let gap = &source[end..port_start];
        if !gap.trim().is_empty() {
            // There's non-whitespace between them - stop looking
            if found {
                break;
            }
            continue;
        }
        result.push(parse_comment_text(text));
        found = true;
    }

    result.reverse();
    result
}

/// Parse a single port from its tokens, extracting leading comments from source
fn parse_port_from_tokens(tokens: Vec<Pair<'_, Rule>>, comments: &[(usize, usize, String)], port_start: usize, source: &str) -> Port {
    let mut iter = tokens.into_iter();

    // First token is the direction keyword
    let direction_text = pair_text(&iter.next().unwrap());
    let direction = parse_direction(&direction_text);

    let mut port_type: Option<PortType> = None;
    let mut dimensions = Vec::new();
    let mut name = String::new();

    for token in iter {
        match token.as_rule() {
            Rule::port_type => {
                port_type = Some(parse_port_type(&pair_text(&token)));
            }
            Rule::dimension => {
                dimensions.push(parse_dimension(token));
            }
            Rule::identifier => {
                name = pair_text(&token);
            }
            Rule::port_direction => {
                // Multi-word directions may have extra direction tokens
            }
            _ => {}
        }
    }

    // Extract leading comments from source text based on position
    let leading_comments = find_leading_comments(comments, port_start, source);

    Port {
        name,
        direction,
        port_type,
        dimensions,
        leading_comments,
        inline_comment: None,
        trailing_comments: Vec::new(),
    }
}

/// Parse the port list, extracting comments from source text for each port
fn parse_port_list(pair: &Pair<'_, Rule>, source: &str, comments: &[(usize, usize, String)]) -> Vec<Port> {
    let inner_pairs: Vec<Pair<'_, Rule>> = pair.clone().into_inner().collect();

    let mut ports = Vec::new();

    for pp in &inner_pairs {
        if pp.as_rule() == Rule::port_decl {
            let tokens: Vec<Pair<'_, Rule>> = pp.clone().into_inner().collect();
            if !tokens.is_empty() {
                // Use the actual byte position of the port_direction pair from the parse tree.
                // This is more reliable than searching for the keyword text, because the
                // port_decl span includes COMMENT tokens that would throw off offset math.
                let port_direction_pair = tokens
                    .iter()
                    .find(|t| t.as_rule() == Rule::port_direction)
                    .expect("port_decl must have a port_direction token");
                let port_start = port_direction_pair.as_span().start();

                let port = parse_port_from_tokens(tokens, comments, port_start, source);
                ports.push(port);
            }
        }
    }

    ports
}

/// Parse a complete module definition
fn parse_module_def(pair: Pair<'_, Rule>, source: &str, comments: &[(usize, usize, String)]) -> Module {
    let inner: Vec<Pair<'_, Rule>> = pair.into_inner().collect();

    // Find module_header
    let header = inner
        .iter()
        .find(|p: &&Pair<'_, Rule>| p.as_rule() == Rule::module_header)
        .cloned()
        .expect("No module_header found");

    // Parse module name from header
    let name = pair_text(
        &header
            .clone()
            .into_inner()
            .find(|p: &Pair<'_, Rule>| p.as_rule() == Rule::identifier)
            .unwrap(),
    );

    // Parse module parameters (Verilog-2001 style: #( ... ))
    // Structure: module_parameters -> parameter_list -> parameter_decl (xN)
    let params: Vec<Parameter> = header
        .clone()
        .into_inner()
        .find(|p: &Pair<'_, Rule>| p.as_rule() == Rule::module_parameters)
        .and_then(|mp: Pair<'_, Rule>| {
            mp.into_inner()
                .find(|p: &Pair<'_, Rule>| p.as_rule() == Rule::parameter_list)
        })
        .map(|pl: Pair<'_, Rule>| -> Vec<Parameter> {
            let mut params = Vec::new();
            for param_pair in pl.into_inner() {
                if param_pair.as_rule() == Rule::parameter_decl {
                    let inner_tokens: Vec<Pair<'_, Rule>> = param_pair.into_inner().collect();
                    // First tokens may be COMMENT tokens (leading comments)
                    let mut leading_comments = Vec::new();
                    let mut value_start = 0;
                    for (i, token) in inner_tokens.iter().enumerate() {
                        if token.as_rule() == Rule::COMMENT {
                            leading_comments.push(parse_comment_text(token.as_str()));
                        } else {
                            value_start = i;
                            break;
                        }
                    }
                    let mut non_comment = inner_tokens.into_iter().skip(value_start);
                    let pname = pair_text(&non_comment.next().unwrap());
                    let pvalue = pair_text(&non_comment.next().unwrap());
                    params.push(Parameter {
                        name: pname,
                        value: pvalue,
                        leading_comments,
                    });
                }
            }
            params
        })
        .unwrap_or_default();

    // Parse port list from header, associating comments from source text
    let ports: Vec<Port> = header
        .clone()
        .into_inner()
        .find(|p: &Pair<'_, Rule>| p.as_rule() == Rule::port_list)
        .map(|p: Pair<'_, Rule>| parse_port_list(&p, source, comments))
        .unwrap_or_default();

    Module {
        name,
        parameters: params,
        ports,
        header_comments: Vec::new(),
        body_comments: Vec::new(),
    }
}

/// Parse the entire file - extracts all modules
pub fn parse(input: &str) -> Result<Vec<Module>, ParseError> {
    // Pre-extract comments from source text with byte positions
    let comments = extract_comments(input);

    let modules: Vec<Module> = VerilogParser::parse(Rule::file, input)?
        .flat_map(|file_pair| {
            file_pair
                .into_inner()
                .filter(|p: &Pair<'_, Rule>| p.as_rule() == Rule::module_def)
                .map(|pair| parse_module_def(pair, input, &comments))
                .collect::<Vec<Module>>()
        })
        .collect();

    Ok(modules)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_tree(pair: pest::iterators::Pair<'_, Rule>, indent: usize) {
        let spaces = " ".repeat(indent * 2);
        let s = pair.as_span();
        let line_no = s.start().to_string();
        println!(
            "{}{} [{}, end={}] str={:?}",
            spaces,
            format!("{:?}", pair.as_rule()),
            line_no,
            s.end(),
            pair.as_str(),
        );
        for child in pair.into_inner() {
            print_tree(child, indent + 1);
        }
    }

    #[test]
    fn debug_simple_adder_tree() {
        let input = std::fs::read_to_string("examples/simple_adder.v").unwrap();
        println!("INPUT:\n{}", input);
        println!("\nPARSE TREE:\n");
        let result = VerilogParser::parse(Rule::file, &input);
        match result {
            Ok(pairs) => {
                for p in pairs {
                    print_tree(p, 0);
                }
            }
            Err(e) => {
                println!("ERROR: {}", e);
            }
        }
        // We don't assert — just print for debugging
    }

    #[test]
    fn test_simple_module() {
        let input = "module adder(input a, input b, output sum); assign sum = a + b; endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].name, "adder");
        assert_eq!(modules[0].ports.len(), 3);
        assert_eq!(modules[0].ports[0].name, "a");
        assert_eq!(modules[0].ports[0].direction, PortDirection::Input);
        assert_eq!(modules[0].ports[1].name, "b");
        assert_eq!(modules[0].ports[1].direction, PortDirection::Input);
        assert_eq!(modules[0].ports[2].name, "sum");
        assert_eq!(modules[0].ports[2].direction, PortDirection::Output);
    }

    #[test]
    fn test_old_style_ports() {
        // Simplified test - single type and dimension
        let input = "module old_style(input wire clk, input wire [7:0] data_in, output reg [15:0] data_out); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].ports.len(), 3);
        assert_eq!(modules[0].ports[0].name, "clk");
        assert_eq!(modules[0].ports[0].direction, PortDirection::Input);
        assert_eq!(modules[0].ports[0].port_type, Some(PortType::Wire));
        assert!(modules[0].ports[0].dimensions.is_empty());
        assert_eq!(modules[0].ports[1].name, "data_in");
        assert_eq!(modules[0].ports[1].direction, PortDirection::Input);
        assert_eq!(modules[0].ports[1].port_type, Some(PortType::Wire));
        assert_eq!(modules[0].ports[1].dimensions.len(), 1);
        assert_eq!(modules[0].ports[1].dimensions[0].msb, "7");
        assert_eq!(modules[0].ports[1].dimensions[0].lsb, "0");
        assert_eq!(modules[0].ports[2].name, "data_out");
        assert_eq!(modules[0].ports[2].direction, PortDirection::Output);
        assert_eq!(modules[0].ports[2].port_type, Some(PortType::Reg));
        assert_eq!(modules[0].ports[2].dimensions.len(), 1);
        assert_eq!(modules[0].ports[2].dimensions[0].msb, "15");
        assert_eq!(modules[0].ports[2].dimensions[0].lsb, "0");
    }

    #[test]
    fn test_new_style_ports() {
        let input = "module new_style(input clk, input [7:0] data_in, output [15:0] data_out, input wire enable); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].ports.len(), 4);
        assert_eq!(modules[0].ports[0].name, "clk");
        assert_eq!(modules[0].ports[0].direction, PortDirection::Input);
        assert_eq!(modules[0].ports[0].port_type, None);
        assert_eq!(modules[0].ports[1].name, "data_in");
        assert_eq!(modules[0].ports[1].direction, PortDirection::Input);
        assert_eq!(modules[0].ports[1].port_type, None);
        assert_eq!(modules[0].ports[1].dimensions.len(), 1);
        assert_eq!(modules[0].ports[2].name, "data_out");
        assert_eq!(modules[0].ports[2].direction, PortDirection::Output);
        assert_eq!(modules[0].ports[2].port_type, None);
        assert_eq!(modules[0].ports[2].dimensions.len(), 1);
        assert_eq!(modules[0].ports[3].name, "enable");
        assert_eq!(modules[0].ports[3].direction, PortDirection::Input);
        assert_eq!(modules[0].ports[3].port_type, Some(PortType::Wire));
    }

    #[test]
    fn test_inout_ports() {
        let input = "module tri_bus(input [7:0] addr, inout [7:0] data_bus, output enable); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].ports.len(), 3);
        assert_eq!(modules[0].ports[1].name, "data_bus");
        assert_eq!(modules[0].ports[1].direction, PortDirection::Inout);
        assert_eq!(modules[0].ports[1].dimensions.len(), 1);
    }

    #[test]
    fn test_parameters() {
        let input = "module param_module #(parameter WIDTH = 8, parameter DEPTH = 256)(input clk, input [WIDTH-1:0] addr, output [WIDTH-1:0] data); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].name, "param_module");
        assert_eq!(modules[0].parameters.len(), 2);
        assert_eq!(modules[0].parameters[0].name, "WIDTH");
        assert_eq!(modules[0].parameters[0].value, "8");
        assert_eq!(modules[0].parameters[1].name, "DEPTH");
        assert_eq!(modules[0].parameters[1].value, "256");
    }

    #[test]
    fn test_systemverilog_logic() {
        let input = "module sv_module(input logic clk, input logic [31:0] addr, output logic [31:0] data); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].ports[0].port_type, Some(PortType::Logic));
        assert_eq!(modules[0].ports[1].port_type, Some(PortType::Logic));
    }

    #[test]
    fn test_no_ports() {
        let input = "module empty_module(); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].ports.len(), 0);
    }

    #[test]
    fn test_multiple_modules() {
        let input = "module module_a(input a, output b); endmodule module module_b(input c, input d, output e); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules.len(), 2);
        assert_eq!(modules[0].name, "module_a");
        assert_eq!(modules[1].name, "module_b");
    }

    #[test]
    fn test_reference_ports() {
        let input = "module sv_ref #(parameter WIDTH = 32)(input ref [WIDTH-1:0] data_in, output ref [WIDTH-1:0] data_out); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].ports[0].direction, PortDirection::Ref);
        assert_eq!(modules[0].ports[1].direction, PortDirection::OutputRef);
    }

    #[test]
    fn test_fifo_module() {
        let input = "module fifo #(parameter DATA_WIDTH = 32, parameter ADDR_WIDTH = 8)(input clk, input rst_n, input wr_en, input rd_en, input [DATA_WIDTH-1:0] wr_data, output [DATA_WIDTH-1:0] rd_data, output [ADDR_WIDTH-1:0] used_count, output full, output empty); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].name, "fifo");
        assert_eq!(modules[0].parameters.len(), 2);
        assert_eq!(modules[0].ports.len(), 9);
        assert_eq!(modules[0].ports[4].name, "wr_data");
        assert_eq!(modules[0].ports[4].dimensions.len(), 1);
        assert_eq!(modules[0].ports[4].dimensions[0].msb, "DATA_WIDTH-1");
    }

    #[test]
    fn test_mux_module() {
        let input = "module mux #(parameter SELECT_WIDTH = 2, parameter DATA_WIDTH = 16)(input [SELECT_WIDTH-1:0] sel, input [DATA_WIDTH-1:0] [3:0] data_in, output [DATA_WIDTH-1:0] data_out); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].name, "mux");
        assert_eq!(modules[0].ports[1].dimensions.len(), 2);
    }

    #[test]
    fn test_register_file() {
        let input = "module regfile #(parameter ADDR_SIZE = 5, parameter DATA_SIZE = 32, parameter DEPTH = 1 << ADDR_SIZE)(input clk, input rst, input [ADDR_SIZE-1:0] rd_addr, input [ADDR_SIZE-1:0] wr_addr, input [DATA_SIZE-1:0] wr_data, input wr_en, output [DATA_SIZE-1:0] rd_data); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].name, "regfile");
        assert_eq!(modules[0].parameters.len(), 3);
        assert_eq!(modules[0].ports.len(), 7);
    }

    #[test]
    fn test_multiline_module() {
        let input = "module multi_line\n    #(\n        parameter WIDTH = 8,\n        parameter DEPTH = 16\n    ) (\n        input wire clk,\n        input wire rst_n,\n        output reg [WIDTH-1:0] q\n    );\n    endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].name, "multi_line");
        assert_eq!(modules[0].parameters.len(), 2);
        assert_eq!(modules[0].ports.len(), 3);
    }

    #[test]
    fn test_end_keyword() {
        // Verilog-2001 uses 'endmodule' as module terminator
        let input = "module short_end(input a, output b); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].name, "short_end");
        assert_eq!(modules[0].ports.len(), 2);
    }

    #[test]
    fn test_signed_unsigned() {
        let input = "module signed_mod(input signed [31:0] a, output unsigned [31:0] b); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].ports[0].port_type, Some(PortType::Signed));
        assert_eq!(modules[0].ports[1].port_type, Some(PortType::Unsigned));
    }

    #[test]
    fn test_comments_in_module() {
        // Comments should be consumed silently
        let input = "// Top-level comment\nmodule commented_mod(\n    input wire clk,  // clock\n    /* data port */\n    input wire [7:0] data\n); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].name, "commented_mod");
        assert_eq!(modules[0].ports.len(), 2);
    }

    #[test]
    fn test_comment_capture() {
        // Verify comments are captured and attached to ports
        let input = "module with_comments(\n    // Clock signal\n    input wire clk,\n    // Reset signal (active low)\n    input wire rst_n,\n    /* Data output port */\n    output reg [7:0] data_out\n); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].name, "with_comments");
        assert_eq!(modules[0].ports.len(), 3);

        // Port 0 (clk) should have a leading comment
        assert!(!modules[0].ports[0].leading_comments.is_empty());
        assert!(matches!(
            modules[0].ports[0].leading_comments[0],
            Comment::Line(ref s) if s.contains("Clock")
        ));

        // Port 1 (rst_n) should have a leading comment
        eprintln!("Port 1 leading_comments: {:?}", modules[0].ports[1].leading_comments);
        assert!(!modules[0].ports[1].leading_comments.is_empty());
        assert!(matches!(
            modules[0].ports[1].leading_comments[0],
            Comment::Line(ref s) if s.contains("Reset")
        ));

        // Port 2 (data_out) should have a block comment
        assert!(!modules[0].ports[2].leading_comments.is_empty());
        assert!(matches!(
            modules[0].ports[2].leading_comments[0],
            Comment::Block(ref s) if s.contains("Data")
        ));
    }

    #[test]
    fn test_block_comment() {
        let input = "/* Multi-line\n   comment */\nmodule block_comment(input a); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].name, "block_comment");
    }

    #[test]
    fn test_parameter_with_braces() {
        // Verilog-2001 style: #(begin parameter X=1; parameter Y=2; end)
        let input = "module brace_params #(begin parameter A = 1; parameter B = 2; end)(input clk); endmodule";
        // This should parse or fail gracefully
        let result = parse(input);
        // For now, brace-style parameters may not be fully supported
        // Just verify it doesn't crash
        drop(result);
    }

    #[test]
    fn test_wide_dimensions() {
        let input = "module wide_mod(input [1023:0] wide_in, output [2047:0] wide_out); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].ports[0].dimensions[0].msb, "1023");
        assert_eq!(modules[0].ports[0].dimensions[0].lsb, "0");
        assert_eq!(modules[0].ports[1].dimensions[0].msb, "2047");
    }

    #[test]
    fn test_empty_port_list() {
        let input = "module no_ports(); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].ports.len(), 0);
    }

    #[test]
    fn test_single_port() {
        let input = "module single(input clk); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].ports.len(), 1);
        assert_eq!(modules[0].ports[0].name, "clk");
    }
}
