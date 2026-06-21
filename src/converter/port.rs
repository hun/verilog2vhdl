// Port conversion: Verilog Port -> VHDL port declaration string

use crate::parser::{Port, PortDirection};
use super::verilog_type::VhdlType;

/// Map Verilog port direction to VHDL direction
fn direction_to_vhdl(direction: &PortDirection) -> &'static str {
    match direction {
        PortDirection::Input => "in",
        PortDirection::Output => "out",
        PortDirection::Inout => "inout",
        PortDirection::Ref | PortDirection::ConstRef => "in",
        PortDirection::OutputRef => "out",
        PortDirection::InoutRef => "inout",
    }
}

/// Convert a single Verilog port to a VHDL port declaration string
pub fn port_to_vhdl(port: &Port) -> String {
    let vhdl_dir = direction_to_vhdl(&port.direction);
    let vhdl_type = VhdlType::from_port(port.port_type.as_ref(), &port.dimensions);
    let type_str = vhdl_type.to_vhdl_string();

    format!("{}: {} {}", port.name, vhdl_dir, type_str)
}
