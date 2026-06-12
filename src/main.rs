// Main entry point for verilog2vhdl

mod parser;
mod converter;

use std::env;
use std::fs;
use std::process;

fn print_usage() {
    println!("verilog2vhdl - Convert Verilog/SystemVerilog modules to VHDL entities");
    println!();
    println!("Usage: verilog2vhdl <input.v[|.sv]> [-o <output.vhd>]");
    println!();
    println!("Options:");
    println!("  -o <file>   Output file (default: stdout)");
    println!("  --help      Show this help");
    println!();
    println!("Examples:");
    println!("  verilog2vhdl adder.v");
    println!("  verilog2vhdl module.sv -o module.vhd");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.contains(&"--help".to_string()) {
        print_usage();
        process::exit(if args.contains(&"--help".to_string()) { 0 } else { 1 });
    }

    let mut input_file = None;
    let mut output_file = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: -o requires a filename argument");
                    process::exit(1);
                }
            }
            _ => {
                if input_file.is_none() {
                    input_file = Some(args[i].clone());
                } else {
                    eprintln!("Error: unexpected argument '{}'", args[i]);
                    process::exit(1);
                }
            }
        }
        i += 1;
    }

    let input_file = match input_file {
        Some(f) => f,
        None => {
            eprintln!("Error: no input file specified");
            print_usage();
            process::exit(1);
        }
    };

    let input = match fs::read_to_string(&input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading '{}': {}", input_file, e);
            process::exit(1);
        }
    };

    let modules = match parser::parse(&input) {
        Ok(mods) => mods,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(1);
        }
    };

    let vhdl = converter::convert_to_vhdl(&modules);

    match output_file {
        Some(f) => {
            match fs::write(&f, &vhdl) {
                Ok(_) => {
                    println!("Wrote {} module(s) to {}", modules.len(), f);
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
