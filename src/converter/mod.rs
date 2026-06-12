// Converter module: Verilog AST -> VHDL entity text

mod verilog_type;
mod port;
mod module_conv;

use crate::parser::Module;

/// Convert one or more Verilog Modules to VHDL text
pub fn convert_to_vhdl(modules: &[Module]) -> String {
    let mut output = String::new();
    for module in modules {
        output.push_str(&module_conv::module_to_vhdl(module));
        output.push('\n');
    }
    output
}

/// Convert one or more Verilog Modules to VHDL entity-only text
/// (no library/use clauses, no architecture stub)
pub fn convert_to_entity_only(modules: &[Module]) -> String {
    let mut output = String::new();
    for module in modules {
        output.push_str(&module_conv::entity_only(module));
        output.push('\n');
    }
    output
}
