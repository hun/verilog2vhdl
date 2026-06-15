#![allow(dead_code)]
// AST definitions for Verilog/SystemVerilog parsing

use std::fmt;

/// Represents a source code span (line numbers, etc.)
#[derive(Debug, Clone)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

/// A comment found in the source
#[derive(Debug, Clone)]
pub enum Comment {
    /// Single-line comment: // text
    Line(String),
    /// Multi-line comment: /* text */
    Block(String),
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Comment::Line(text) => write!(f, "{}", text),
            Comment::Block(text) => write!(f, "{}", text),
        }
    }
}

/// Port direction in Verilog
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortDirection {
    Input,
    Output,
    Inout,
    Ref,
    ConstRef,
    OutputRef,
    InoutRef,
}

impl fmt::Display for PortDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortDirection::Input => write!(f, "input"),
            PortDirection::Output => write!(f, "output"),
            PortDirection::Inout => write!(f, "inout"),
            PortDirection::Ref => write!(f, "ref"),
            PortDirection::ConstRef => write!(f, "const ref"),
            PortDirection::OutputRef => write!(f, "output ref"),
            PortDirection::InoutRef => write!(f, "inout ref"),
        }
    }
}

/// Port type in Verilog
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortType {
    Wire,
    Reg,
    Logic,
    Wand,
    Tri,
    Triand,
    Trior,
    Tri0,
    Tri1,
    Supply0,
    Supply1,
    Integer,
    Time,
    Signed,
    Unsigned,
    Packed,
}

impl fmt::Display for PortType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortType::Wire => write!(f, "wire"),
            PortType::Reg => write!(f, "reg"),
            PortType::Logic => write!(f, "logic"),
            PortType::Wand => write!(f, "wand"),
            PortType::Tri => write!(f, "tri"),
            PortType::Triand => write!(f, "triand"),
            PortType::Trior => write!(f, "trior"),
            PortType::Tri0 => write!(f, "tri0"),
            PortType::Tri1 => write!(f, "tri1"),
            PortType::Supply0 => write!(f, "supply0"),
            PortType::Supply1 => write!(f, "supply1"),
            PortType::Integer => write!(f, "integer"),
            PortType::Time => write!(f, "time"),
            PortType::Signed => write!(f, "signed"),
            PortType::Unsigned => write!(f, "unsigned"),
            PortType::Packed => write!(f, "packed"),
        }
    }
}

/// A dimension range [MSB:LSB]
#[derive(Debug, Clone)]
pub struct Dimension {
    pub msb: String,
    pub lsb: String,
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.msb, self.lsb)
    }
}

/// A port in a Verilog module
#[derive(Debug, Clone)]
pub struct Port {
    /// Port name
    pub name: String,
    /// Port direction (input/output/inout, etc.)
    pub direction: PortDirection,
    /// Port type (wire/reg/logic, etc.) - None for new-style ports with implicit type
    pub port_type: Option<PortType>,
    /// Array dimensions if this is a vector
    pub dimensions: Vec<Dimension>,
    /// Comments that appear before this port declaration
    pub leading_comments: Vec<Comment>,
    /// Comments that appear on the same line after the port name
    pub inline_comment: Option<Comment>,
    /// Comments that appear after this port declaration (before next port)
    pub trailing_comments: Vec<Comment>,
}

/// A parameter in a Verilog module
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter value expression
    pub value: String,
    /// Optional parameter type (int, bit, logic, signed, unsigned)
    pub param_type: Option<String>,
    /// Array dimensions if this is a vector parameter
    pub dimensions: Vec<Dimension>,
    /// Comments that appear before this parameter
    pub leading_comments: Vec<Comment>,
}

/// A complete Verilog module
#[derive(Debug, Clone)]
pub struct Module {
    /// Module name
    pub name: String,
    /// Module parameters (if any)
    pub parameters: Vec<Parameter>,
    /// Module ports
    pub ports: Vec<Port>,
    /// Module declaration comments
    pub header_comments: Vec<Comment>,
    /// Comments between module declaration and end
    pub body_comments: Vec<Comment>,
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.direction, self.name)?;
        if !self.dimensions.is_empty() {
            for dim in &self.dimensions {
                write!(f, "{}", dim)?;
            }
        }
        if let Some(ref port_type) = self.port_type {
            write!(f, " {}", port_type)?;
        }
        Ok(())
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module {}(", self.name)?;
        if let Some((last, ports)) = self.ports.split_last() {
            for port in ports {
                write!(f, "{}, ", port)?;
            }
            write!(f, "{}", last)?;
        }
        writeln!(f, ")")?;
        Ok(())
    }
}
