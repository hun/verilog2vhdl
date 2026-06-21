// Module conversion: Verilog Module AST -> VHDL entity + architecture stub

use crate::parser::{Module, Parameter};
use super::port::port_to_vhdl;

/// Convert a Verilog parameter to a VHDL generic declaration
fn param_to_vhdl(param: &Parameter) -> String {
    let comment = if !param.leading_comments.is_empty() {
        let comments: Vec<String> = param.leading_comments.iter()
            .map(|c| format!("        -- {}", c))
            .collect();
        comments.join("\n") + "\n"
    } else {
        String::new()
    };

    let vhdl_type = match (&param.param_type, param.dimensions.is_empty()) {
        (Some(ptype), true) => match ptype.as_str() {
            "int" => "integer".to_string(),
            "bit" | "logic" => "std_logic".to_string(),
            "signed" | "unsigned" => "integer".to_string(),
            _ => "integer".to_string(),
        },
        (Some(ptype), false) => {
            // Typed parameter with dimensions -> vector type
            let range = if param.dimensions.len() == 1 {
                let d = &param.dimensions[0];
                format!("{} downto {}", d.msb, d.lsb)
            } else {
                // Multi-dimensional: use first dimension
                let d = &param.dimensions[0];
                format!("{} downto {}", d.msb, d.lsb)
            };
            match ptype.as_str() {
                "signed" => format!("signed({})", range),
                "unsigned" => format!("unsigned({})", range),
                "logic" => format!("std_logic_vector({})", range),
                "bit" => format!("std_logic_vector({})", range),
                _ => format!("std_logic_vector({})", range),
            }
        }
        (None, _) => "integer".to_string(),
    };

    format!("{}{} : {} := {}", comment, param.name, vhdl_type, param.value)
}

/// Generate the VHDL library and use clauses
fn library_clauses() -> String {
    "library ieee;\nuse ieee.std_logic_1164.all;\nuse ieee.numeric_std.all;".to_string()
}

/// Generate just the entity declaration (no library/use, no architecture stub)
pub fn entity_only(module: &Module) -> String {
    let mut output = String::new();

    output.push_str(&format!("entity {} is\n", module.name));

    // Generics (from parameters)
    if !module.parameters.is_empty() {
        output.push_str("    generic (\n");
        for (i, param) in module.parameters.iter().enumerate() {
            let decl = param_to_vhdl(param);
            if i < module.parameters.len() - 1 {
                output.push_str(&format!("        {};\n", decl));
            } else {
                output.push_str(&format!("        {}\n", decl));
            }
        }
        output.push_str("    );\n");
    }

    // Port declarations
    output.push_str("    port (\n");
    for (i, port) in module.ports.iter().enumerate() {
        let comment = if !port.leading_comments.is_empty() {
            let comments: Vec<String> = port.leading_comments.iter()
                .map(|c| format!("        -- {}", c))
                .collect();
            comments.join("\n") + "\n"
        } else {
            String::new()
        };
        let decl = port_to_vhdl(port);
        if i < module.ports.len() - 1 {
            output.push_str(&format!("{}        {};\n", comment, decl));
        } else {
            output.push_str(&format!("{}        {}\n", comment, decl));
        }
    }
    output.push_str("    );\n");
    output.push_str(&format!("end entity {};\n", module.name));

    output
}

/// Convert a complete Verilog Module AST to a VHDL entity + architecture stub
pub fn module_to_vhdl(module: &Module) -> String {
    let mut output = String::new();

    // Library clauses
    output.push_str(&library_clauses());
    output.push('\n');

    // Blank line before entity
    output.push('\n');

    output.push_str(&entity_only(module));

    // Architecture stub
    output.push('\n');
    output.push_str(&format!(
        "architecture rtl of {} is\nbegin\n    -- Internal logic stub\nend architecture rtl;\n",
        module.name
    ));

    output
}
