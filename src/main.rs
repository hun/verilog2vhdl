// Main entry point for verilog2vhdl

use clap::Parser;
use std::fs;
use std::io::{self, Read};
use std::process;

use verilog2vhdl::converter::{convert_to_entity_only, convert_to_vhdl};
use verilog2vhdl::parser::parse;

/// Verilog/SystemVerilog to VHDL Entity Converter
#[derive(Parser, Debug)]
#[command(name = "verilog2vhdl")]
#[command(version, about = "Convert Verilog/SystemVerilog modules to VHDL entities", long_about = None)]
struct Cli {
    /// Input Verilog/SystemVerilog file (.v, .sv) or '-' for stdin
    input: Option<String>,

    /// Output VHDL file (default: stdout)
    #[arg(short, long)]
    output: Option<String>,

    /// Print only the VHDL entity (no library/use clauses, no architecture stub)
    #[arg(short, long)]
    entity_only: bool,
}

fn read_input(cli: &Cli) -> String {
    match &cli.input {
        Some(path) if path == "-" => {
            let mut buffer = String::new();
            if let Err(e) = io::stdin().read_to_string(&mut buffer) {
                eprintln!("Error reading stdin: {}", e);
                process::exit(1);
            }
            buffer
        }
        Some(path) => match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading '{}': {}", path, e);
                process::exit(1);
            }
        },
        None => {
            let mut buffer = String::new();
            if let Err(e) = io::stdin().read_to_string(&mut buffer) {
                eprintln!("Error reading stdin: {}", e);
                process::exit(1);
            }
            buffer
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let input = read_input(&cli);

    let modules = match parse(&input) {
        Ok(mods) => mods,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(1);
        }
    };

    let vhdl = if cli.entity_only {
        convert_to_entity_only(&modules)
    } else {
        convert_to_vhdl(&modules)
    };

    match cli.output {
        Some(ref f) => {
            match fs::write(f, &vhdl) {
                Ok(_) => {
                    eprintln!("Wrote {} module(s) to {}", modules.len(), f);
                }
                Err(e) => {
                    eprintln!("Error writing '{}': {}", f, e);
                    process::exit(1);
                }
            }
        }
        None => {
            print!("{}", vhdl);
        }
    }
}
