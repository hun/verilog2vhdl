// Verilog type -> VHDL type mapping

use crate::parser::{Dimension, PortType};

/// Represents a VHDL type that can be used in port declarations
#[derive(Debug, Clone)]
pub enum VhdlType {
    /// std_logic (scalar)
    StdLogic,
    /// std_logic_vector(N-1 downto 0)
    StdLogicVector(Dimension),
    /// std_logic_vector(N downto M) for multi-dim or non-zero-based
    StdLogicVectorRange(String, String),
    /// signed(N-1 downto 0)
    Signed(Dimension),
    /// signed(N downto M)
    SignedRange(String, String),
    /// unsigned(N-1 downto 0)
    Unsigned(Dimension),
    /// unsigned(N downto M)
    UnsignedRange(String, String),
    /// integer (for parameters)
    #[allow(dead_code)]
    Integer,
}

impl VhdlType {
    /// Convert a Verilog port_type + dimensions to a VHDL type
    pub fn from_port(port_type: Option<&PortType>, dimensions: &[Dimension]) -> Self {
        // If no dimensions, it's a scalar -> std_logic
        if dimensions.is_empty() {
            return VhdlType::StdLogic;
        }

        // Check if any dimension is signed/unsigned
        let is_signed = port_type == Some(&PortType::Signed);
        let is_unsigned = port_type == Some(&PortType::Unsigned);

        if dimensions.len() == 1 {
            let dim = &dimensions[0];
            // Check if it's a simple [N:0] range (common case)
            if dim.lsb == "0" {
                let msb = &dim.msb;
                match (is_signed, is_unsigned) {
                    (true, _) => VhdlType::Signed(Dimension {
                        msb: msb.clone(),
                        lsb: "0".to_string(),
                    }),
                    (_, true) => VhdlType::Unsigned(Dimension {
                        msb: msb.clone(),
                        lsb: "0".to_string(),
                    }),
                    _ => VhdlType::StdLogicVector(Dimension {
                        msb: msb.clone(),
                        lsb: "0".to_string(),
                    }),
                }
            } else {
                // General range [msb:lsb]
                match (is_signed, is_unsigned) {
                    (true, _) => VhdlType::SignedRange(dim.msb.clone(), dim.lsb.clone()),
                    (_, true) => VhdlType::UnsignedRange(dim.msb.clone(), dim.lsb.clone()),
                    _ => VhdlType::StdLogicVectorRange(dim.msb.clone(), dim.lsb.clone()),
                }
            }
        } else {
            // Multi-dimensional: use the first dimension for the outermost range
            // VHDL allows multi-dimensional vectors in std_logic_vector
            let mut ranges: Vec<String> = Vec::new();
            for dim in dimensions {
                ranges.push(format!("{} downto {}", dim.msb, dim.lsb));
            }
            // For multi-dim, use the first dimension
            let first = &dimensions[0];
            if first.lsb == "0" {
                match (is_signed, is_unsigned) {
                    (true, _) => VhdlType::Signed(Dimension {
                        msb: first.msb.clone(),
                        lsb: "0".to_string(),
                    }),
                    (_, true) => VhdlType::Unsigned(Dimension {
                        msb: first.msb.clone(),
                        lsb: "0".to_string(),
                    }),
                    _ => VhdlType::StdLogicVector(Dimension {
                        msb: first.msb.clone(),
                        lsb: "0".to_string(),
                    }),
                }
            } else {
                match (is_signed, is_unsigned) {
                    (true, _) => VhdlType::SignedRange(first.msb.clone(), first.lsb.clone()),
                    (_, true) => VhdlType::UnsignedRange(first.msb.clone(), first.lsb.clone()),
                    _ => VhdlType::StdLogicVectorRange(first.msb.clone(), first.lsb.clone()),
                }
            }
        }
    }

    /// Convert to VHDL type string
    pub fn to_vhdl_string(&self) -> String {
        match self {
            VhdlType::StdLogic => "std_logic".to_string(),
            VhdlType::StdLogicVector(d) => format!("std_logic_vector({} downto {})", d.msb, d.lsb),
            VhdlType::StdLogicVectorRange(msb, lsb) => format!("std_logic_vector({} downto {})", msb, lsb),
            VhdlType::Signed(d) => format!("signed({} downto {})", d.msb, d.lsb),
            VhdlType::SignedRange(msb, lsb) => format!("signed({} downto {})", msb, lsb),
            VhdlType::Unsigned(d) => format!("unsigned({} downto {})", d.msb, d.lsb),
            VhdlType::UnsignedRange(msb, lsb) => format!("unsigned({} downto {})", msb, lsb),
            VhdlType::Integer => "integer".to_string(),
        }
    }
}

impl std::fmt::Display for VhdlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_vhdl_string())
    }
}
