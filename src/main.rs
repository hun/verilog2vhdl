// Main entry point for verilog2vhdl

mod parser;

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
    
    let output = match output_file {
        Some(ref f) => {
            let mut out_str = String::new();
            for module in &modules {
                out_str.push_str(&format!("// Converted from: {}\n", input_file));
                out_str.push_str(&format!("// Module: {}\n", module.name));
                out_str.push_str(&format!(
                    "// Ports: {}\n\n",
                    module.ports.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ")
                ));
                out_str.push_str(&format!("entity {} is\n", module.name));
                // ... (VHDL generation would go here)
                out_str.push_str("begin\n");
                out_str.push_str("end;\n\n");
            }
            match fs::write(f, &out_str) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error writing '{}': {}", f, e);
                    process::exit(1);
                }
            }
            format!("Wrote {} module(s) to {}\n", modules.len(), f)
        }
        None => {
            // Print to stdout for now - just show parsed info
            let mut out = String::new();
            for module in &modules {
                out.push_str(&format!("module {}\n", module.name));
                if !module.parameters.is_empty() {
                    out.push_str("  parameters:\n");
                    for param in &module.parameters {
                        out.push_str(&format!("    {} = {}\n", param.name, param.value));
                    }
                }
                if !module.ports.is_empty() {
                    out.push_str("  ports:\n");
                    for port in &module.ports {
                        out.push_str(&format!(
                            "    {} {} {:?} {} (dims: {})\n",
                            port.direction,
                            port.name,
                            port.port_type,
                            if port.dimensions.is_empty() {
                                "".to_string()
                            } else {
                                format!("[{}]", 
                                    port.dimensions.iter()
                                        .map(|d| format!("{}:{}", d.msb, d.lsb))
                                        .collect::<Vec<_>>()
                                        .join("]["))
                            },
                            port.dimensions.len()
                        ));
                    }
                }
                out.push('\n');
            }
            out
        }
    };
    
    print!("{}", output);
}
