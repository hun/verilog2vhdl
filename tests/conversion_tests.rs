// Integration tests: Verilog input -> VHDL output comparison

use verilog2vhdl::converter::convert_to_vhdl;
use verilog2vhdl::parser::parse;

use std::fs;

/// Run the full pipeline (parse + convert) on a .v file and compare with expected .vhd
fn run_case(case_name: &str) {
    let base = format!("tests/cases/{}", case_name);
    let input = fs::read_to_string(format!("{}.v", base))
        .unwrap_or_else(|e| panic!("Failed to read {}.v: {}", base, e));

    let expected = fs::read_to_string(format!("{}.vhd", base))
        .unwrap_or_else(|e| panic!("Failed to read {}.vhd: {}", base, e));

    let modules = parse(&input)
        .unwrap_or_else(|e| panic!("Parse failed for {}: {}", case_name, e));

    let output = convert_to_vhdl(&modules);

    assert_eq!(
        output, expected,
        "Conversion mismatch for '{}'\n--- OUTPUT ---\n{}\n--- EXPECTED ---\n{}",
        case_name, output, expected
    );
}

#[test]
fn test_simple_adder() {
    run_case("simple_adder");
}

#[test]
fn test_parameterized_fifo() {
    run_case("parameterized_fifo");
}

#[test]
fn test_multidim_mux() {
    run_case("multidim_mux");
}

#[test]
fn test_signed_unsigned() {
    run_case("signed_unsigned");
}

#[test]
fn test_no_ports() {
    run_case("no_ports");
}

#[test]
fn test_pre_post_content() {
    run_case("pre_post_content");
}
