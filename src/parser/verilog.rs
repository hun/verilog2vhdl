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

/// Parse a dimension [MSB:LSB]
fn parse_dimension(pair: Pair<'_, Rule>) -> Dimension {
    let mut parts = pair.into_inner();
    let msb = pair_text(&parts.next().unwrap());
    let lsb = pair_text(&parts.next().unwrap());
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
        _ => panic!("Unknown port type: {}", text),
    }
}

/// Parse a single port from its tokens
fn parse_port_from_tokens(tokens: Vec<Pair<'_, Rule>>) -> Port {
    let direction_text = pair_text(&tokens[0]);
    let direction = parse_direction(&direction_text);
    
    let mut port_type: Option<PortType> = None;
    let mut dimensions = Vec::new();
    let mut name = String::new();
    
    for token in &tokens[1..] {
        match token.as_rule() {
            Rule::port_type => {
                port_type = Some(parse_port_type(&pair_text(token)));
            }
            Rule::dimension => {
                dimensions.push(parse_dimension(token.clone()));
            }
            Rule::identifier => {
                name = pair_text(token);
            }
            Rule::port_direction => {
                // This can happen with "const ref" - the "ref" part is parsed separately
                // We'll handle it by checking the direction
            }
            _ => {}
        }
    }
    
    Port {
        name,
        direction,
        port_type,
        dimensions,
        leading_comments: Vec::new(),
        inline_comment: None,
        trailing_comments: Vec::new(),
    }
}

/// Parse a complete module definition
fn parse_module_def(pair: Pair<'_, Rule>) -> Module {
    let inner: Vec<Pair<'_, Rule>> = pair.into_inner().collect();
    
    // Find module_header
    let header = inner.iter()
        .find(|p: &&Pair<'_, Rule>| p.as_rule() == Rule::module_header)
        .cloned()
        .expect("No module_header found");
    
    // Parse module name from header
    let name = pair_text(&header.clone().into_inner().find(|p: &Pair<'_, Rule>| p.as_rule() == Rule::identifier).unwrap());
    
    // Parse module parameters (Verilog-2001 style: #( ... ))
    let params: Vec<Parameter> = header.clone().into_inner()
        .find(|p: &Pair<'_, Rule>| p.as_rule() == Rule::module_parameters)
        .map(|p: Pair<'_, Rule>| -> Vec<Parameter> {
            let mut params = Vec::new();
            for param_pair in p.into_inner() {
                if param_pair.as_rule() == Rule::parameter_decl {
                    let mut inner = param_pair.into_inner();
                    let pname = pair_text(&inner.next().unwrap());
                    let pvalue = pair_text(&inner.next().unwrap());
                    params.push(Parameter {
                        name: pname,
                        value: pvalue,
                        leading_comments: Vec::new(),
                    });
                }
            }
            params
        })
        .unwrap_or_default();
    
    // Parse port list from header
    let ports: Vec<Port> = header.clone().into_inner()
        .find(|p: &Pair<'_, Rule>| p.as_rule() == Rule::port_list)
        .map(|p: Pair<'_, Rule>| -> Vec<Port> {
            let mut ports = Vec::new();
            let inner_pairs: Vec<Pair<'_, Rule>> = p.clone().into_inner().collect();
            
            // port_list contains port_decl nodes
            for pp in &inner_pairs {
                if pp.as_rule() == Rule::port_decl {
                    let tokens: Vec<Pair<'_, Rule>> = pp.clone().into_inner().collect();
                    if !tokens.is_empty() {
                        ports.push(parse_port_from_tokens(tokens));
                    }
                }
            }
            ports
        })
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
    let modules: Vec<Module> = VerilogParser::parse(Rule::file, input)?
        .flat_map(|file_pair| {
            file_pair.into_inner()
                .filter(|p: &Pair<'_, Rule>| p.as_rule() == Rule::module_def)
                .map(parse_module_def)
                .collect::<Vec<Module>>()
        })
        .collect();
    
    Ok(modules)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let modules = parse(input);
        match &modules {
            Ok(modules) => {
                println!("Parsed {} modules", modules.len());
                for m in modules {
                    println!("Module: {}", m.name);
                    for p in &m.ports {
                        println!("  Port: {:?} {:?} {:?}", p.name, p.direction, p.port_type);
                    }
                }
            }
            Err(e) => {
                println!("Parse error: {:?}", e);
            }
        }
        let modules = modules.unwrap();
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
        assert_eq!(modules[0].ports[1].direction, PortDirection::Ref);
    }

    #[test]
    fn test_fifo_module() {
        let input = "module fifo #(parameter DATA_WIDTH = 32, parameter ADDR_WIDTH = 8)(input clk, input rst_n, input wr_en, input rd_en, input [DATA_WIDTH-1:0] wr_data, output [DATA_WIDTH-1:0] rd_data, output [ADDR_WIDTH-1:0] used_count, output full, output empty); endmodule";
        let modules = parse(input).unwrap();
        assert_eq!(modules[0].name, "fifo");
        assert_eq!(modules[0].parameters.len(), 2);
        assert_eq!(modules[0].ports.len(), 10);
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
        assert_eq!(modules[0].ports.len(), 8);
    }
}
