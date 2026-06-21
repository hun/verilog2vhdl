// Integration tests: Parse and convert all .v test cases
// Tests that the parser doesn't crash on any valid Verilog input.
// Tests that the converter produces valid VHDL output.

use verilog2vhdl::converter::convert_to_vhdl;
use verilog2vhdl::parser::parse;

use std::fs;
use std::path::Path;

/// Discover all .v test case files in tests/cases/
fn discover_test_cases() -> Vec<String> {
    let cases_dir = Path::new("tests/cases");
    let mut cases = Vec::new();
    if let Ok(entries) = fs::read_dir(cases_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "v") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    cases.push(name.to_string());
                }
            }
        }
    }
    cases.sort();
    cases
}

/// Run the full pipeline (parse + convert) on a .v file.
/// This is a "harden the parser" test: it checks that the parser
/// accepts the input without crashing. Conversion must succeed but
/// may produce empty output for files with no modules.
fn run_parse_test(case_name: &str) {
    let base = format!("tests/cases/{}", case_name);
    let input = fs::read_to_string(format!("{}.v", base))
        .unwrap_or_else(|e| panic!("Failed to read {}.v: {}", base, e));

    // Parse should succeed (no crash)
    let modules = parse(&input)
        .unwrap_or_else(|e| panic!("Parse failed for '{}': {}", case_name, e));

    // Convert should succeed (no crash)
    // May produce empty output for files with no modules
    let _output = convert_to_vhdl(&modules);
}

/// Run the full pipeline and check that VHDL output contains expected markers.
/// Only checks modules that actually produce output.
fn run_conversion_test(case_name: &str) {
    let base = format!("tests/cases/{}", case_name);
    let input = fs::read_to_string(format!("{}.v", base))
        .unwrap_or_else(|e| panic!("Failed to read {}.v: {}", base, e));

    let modules = parse(&input)
        .unwrap_or_else(|e| panic!("Parse failed for '{}': {}", case_name, e));

    let output = convert_to_vhdl(&modules);

    // If no modules, empty output is fine
    if modules.is_empty() {
        return;
    }

    // VHDL output should contain basic structure
    assert!(
        output.contains("entity "),
        "VHDL output for '{}' missing 'entity' keyword",
        case_name
    );
    assert!(
        output.contains("end entity"),
        "VHDL output for '{}' missing 'end entity'",
        case_name
    );
    assert!(
        output.contains("architecture"),
        "VHDL output for '{}' missing 'architecture'",
        case_name
    );
    assert!(
        output.contains("end architecture"),
        "VHDL output for '{}' missing 'end architecture'",
        case_name
    );
}

/// Run the full pipeline and compare with expected .vhd output.
fn run_case(case_name: &str) {
    let base = format!("tests/cases/{}", case_name);
    let input = fs::read_to_string(format!("{}.v", base))
        .unwrap_or_else(|e| panic!("Failed to read {}.v: {}", base, e));

    let expected = fs::read_to_string(format!("{}.vhd", base))
        .unwrap_or_else(|e| panic!("Failed to read {}.vhd: {}", base, e));

    let modules = parse(&input)
        .unwrap_or_else(|e| panic!("Parse failed for '{}': {}", case_name, e));

    let output = convert_to_vhdl(&modules);

    assert_eq!(
        output, expected,
        "Conversion mismatch for '{}'\n--- OUTPUT ---\n{}\n--- EXPECTED ---\n{}",
        case_name, output, expected
    );
}

// ---- Existing integration tests (unchanged) ----

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

// ---- New parse-only tests (harden the parser) ----

#[test]
fn test_all_cases_parse() {
    let cases = discover_test_cases();
    let mut failures = Vec::new();
    for case in &cases {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run_parse_test(case);
        })) {
            Ok(_) => {}
            Err(_) => {
                failures.push(case.clone());
            }
        }
    }
    assert!(
        failures.is_empty(),
        "Parse failures: {}",
        failures.join(", ")
    );
}

#[test]
fn test_all_cases_convert() {
    let cases = discover_test_cases();
    let mut failures = Vec::new();
    for case in &cases {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run_conversion_test(case);
        })) {
            Ok(_) => {}
            Err(_) => {
                failures.push(case.clone());
            }
        }
    }
    assert!(
        failures.is_empty(),
        "Conversion failures: {}",
        failures.join(", ")
    );
}

// ---- New individual parse tests for tricky cases ----

#[test]
fn test_empty_file() {
    run_parse_test("empty_file");
}

#[test]
fn test_only_comments() {
    run_parse_test("only_comments");
}

#[test]
fn test_only_directives() {
    run_parse_test("only_directives");
}

#[test]
fn test_empty_module() {
    run_parse_test("empty_module");
}

#[test]
fn test_no_params_no_ports() {
    run_parse_test("no_params_no_ports");
}

#[test]
fn test_single_port() {
    run_parse_test("single_port");
}

#[test]
fn test_many_ports() {
    run_parse_test("many_ports");
}

#[test]
fn test_many_params() {
    run_parse_test("many_params");
}

#[test]
fn test_many_params_many_ports() {
    run_parse_test("many_params_many_ports");
}

#[test]
fn test_multi_module() {
    run_parse_test("multi_module");
}

#[test]
fn test_module_only_params() {
    run_parse_test("module_only_params");
}

// ---- Parameter value format tests ----

#[test]
fn test_param_binary() {
    run_parse_test("param_binary");
}

#[test]
fn test_param_hex() {
    run_parse_test("param_hex");
}

#[test]
fn test_param_octal() {
    run_parse_test("param_octal");
}

#[test]
fn test_param_decimal() {
    run_parse_test("param_decimal");
}

#[test]
fn test_param_expr() {
    run_parse_test("param_expr");
}

#[test]
fn test_param_expression() {
    run_parse_test("param_expression");
}

#[test]
fn test_param_complex_expr() {
    run_parse_test("param_complex_expr");
}

#[test]
fn test_param_signed() {
    run_parse_test("param_signed");
}

#[test]
fn test_param_unsigned() {
    run_parse_test("param_unsigned");
}

#[test]
fn test_param_int() {
    run_parse_test("param_int");
}

#[test]
fn test_param_bit() {
    run_parse_test("param_bit");
}

#[test]
fn test_param_logic() {
    run_parse_test("param_logic");
}

#[test]
fn test_param_mixed_types() {
    run_parse_test("param_mixed_types");
}

#[test]
fn test_param_shorthand() {
    run_parse_test("param_shorthand");
}

#[test]
fn test_param_braces() {
    run_parse_test("param_braces");
}

#[test]
fn test_param_large_value() {
    run_parse_test("param_large_value");
}

#[test]
fn test_param_zero() {
    run_parse_test("param_zero");
}

#[test]
fn test_param_negative() {
    run_parse_test("param_negative");
}

#[test]
fn test_param_hex_lower() {
    run_parse_test("param_hex_lower");
}

#[test]
fn test_param_hex_upper() {
    run_parse_test("param_hex_upper");
}

#[test]
fn test_param_multi_dim() {
    run_parse_test("param_multi_dim");
}

#[test]
fn test_param_binary_long() {
    run_parse_test("param_binary_long");
}

#[test]
fn test_param_binary_upper() {
    run_parse_test("param_binary_upper");
}

#[test]
fn test_param_octal_upper() {
    run_parse_test("param_octal_upper");
}

#[test]
fn test_param_decimal_upper() {
    run_parse_test("param_decimal_upper");
}

#[test]
fn test_param_sized_binary() {
    run_parse_test("param_sized_binary");
}

#[test]
fn test_param_sized_hex() {
    run_parse_test("param_sized_hex");
}

#[test]
fn test_param_sized_octal() {
    run_parse_test("param_sized_octal");
}

#[test]
fn test_param_sized_decimal() {
    run_parse_test("param_sized_decimal");
}

#[test]
fn test_param_sized_signed() {
    run_parse_test("param_sized_signed");
}

#[test]
fn test_param_sized_unsigned() {
    run_parse_test("param_sized_unsigned");
}

#[test]
fn test_param_sized_signed_dec() {
    run_parse_test("param_sized_signed_dec");
}

#[test]
fn test_param_comma_expr() {
    run_parse_test("param_comma_expr");
}

#[test]
fn test_param_paren_expr() {
    run_parse_test("param_paren_expr");
}

#[test]
fn test_param_unary_minus() {
    run_parse_test("param_unary_minus");
}

#[test]
fn test_param_unary_plus() {
    run_parse_test("param_unary_plus");
}

#[test]
fn test_param_bitwise_and() {
    run_parse_test("param_bitwise_and");
}

#[test]
fn test_param_bitwise_or() {
    run_parse_test("param_bitwise_or");
}

#[test]
fn test_param_bitwise_xor() {
    run_parse_test("param_bitwise_xor");
}

#[test]
fn test_param_shift_left() {
    run_parse_test("param_shift_left");
}

#[test]
fn test_param_shift_right() {
    run_parse_test("param_shift_right");
}

#[test]
fn test_param_ternary() {
    run_parse_test("param_ternary");
}

#[test]
fn test_param_concat() {
    run_parse_test("param_concat");
}

#[test]
fn test_param_repeat() {
    run_parse_test("param_repeat");
}

#[test]
fn test_param_blank() {
    run_parse_test("param_blank");
}

#[test]
fn test_param_empty_string() {
    run_parse_test("param_empty_string");
}

#[test]
fn test_param_spaces_only() {
    run_parse_test("param_spaces_only");
}

#[test]
fn test_param_with_comment_in_value() {
    run_parse_test("param_with_comment_in_value");
}

#[test]
fn test_param_with_block_comment_in_value() {
    run_parse_test("param_with_block_comment_in_value");
}

#[test]
fn test_param_with_multiline_comment() {
    run_parse_test("param_with_multiline_comment");
}

#[test]
fn test_param_with_nested_block_comment() {
    run_parse_test("param_with_nested_block_comment");
}

#[test]
fn test_param_with_very_long_name() {
    run_parse_test("param_with_very_long_name");
}

#[test]
fn test_param_with_very_long_value() {
    run_parse_test("param_with_very_long_value");
}

#[test]
fn test_param_with_very_long_name_and_value() {
    run_parse_test("param_with_very_long_name_and_value");
}

#[test]
fn test_param_with_empty_value() {
    run_parse_test("param_with_empty_value");
}

#[test]
fn test_param_with_only_spaces() {
    run_parse_test("param_with_only_spaces");
}

#[test]
fn test_param_with_only_tabs() {
    run_parse_test("param_with_only_tabs");
}

#[test]
fn test_param_with_only_newlines() {
    run_parse_test("param_with_only_newlines");
}

#[test]
fn test_param_with_only_crlf() {
    run_parse_test("param_with_only_crlf");
}

#[test]
fn test_param_with_only_whitespace() {
    run_parse_test("param_with_only_whitespace");
}

#[test]
fn test_param_with_unicode() {
    run_parse_test("param_with_unicode");
}

#[test]
fn test_param_with_special_chars() {
    run_parse_test("param_with_special_chars");
}

#[test]
fn test_param_with_mixed_base() {
    run_parse_test("param_with_mixed_base");
}

#[test]
fn test_param_with_mixed_sized() {
    run_parse_test("param_with_mixed_sized");
}

#[test]
fn test_param_with_mixed_types_and_values() {
    run_parse_test("param_with_mixed_types_and_values");
}

#[test]
fn test_param_with_mixed_types_and_shorthand() {
    run_parse_test("param_with_mixed_types_and_shorthand");
}

#[test]
fn test_param_with_mixed_types_and_braces() {
    run_parse_test("param_with_mixed_types_and_braces");
}

#[test]
fn test_param_with_mixed_types_and_comments() {
    run_parse_test("param_with_mixed_types_and_comments");
}

#[test]
fn test_param_with_mixed_types_and_inline() {
    run_parse_test("param_with_mixed_types_and_inline");
}

#[test]
fn test_param_with_mixed_types_and_trailing() {
    run_parse_test("param_with_mixed_types_and_trailing");
}

#[test]
fn test_param_with_mixed_types_and_leading() {
    run_parse_test("param_with_mixed_types_and_leading");
}

#[test]
fn test_param_with_mixed_types_and_both() {
    run_parse_test("param_with_mixed_types_and_both");
}

#[test]
fn test_param_with_mixed_types_and_newline() {
    run_parse_test("param_with_mixed_types_and_newline");
}

#[test]
fn test_param_with_mixed_types_and_tab() {
    run_parse_test("param_with_mixed_types_and_tab");
}

#[test]
fn test_param_with_mixed_types_and_crlf() {
    run_parse_test("param_with_mixed_types_and_crlf");
}

#[test]
fn test_param_with_mixed_types_and_escaped_quote() {
    run_parse_test("param_with_mixed_types_and_escaped_quote");
}

#[test]
fn test_param_with_mixed_types_and_dollar() {
    run_parse_test("param_with_mixed_types_and_dollar");
}

#[test]
fn test_param_with_mixed_types_and_at() {
    run_parse_test("param_with_mixed_types_and_at");
}

#[test]
fn test_param_with_mixed_types_and_hash() {
    run_parse_test("param_with_mixed_types_and_hash");
}

#[test]
fn test_param_with_mixed_types_and_underscore_num() {
    run_parse_test("param_with_mixed_types_and_underscore_num");
}

#[test]
fn test_param_with_mixed_types_and_exponent() {
    run_parse_test("param_with_mixed_types_and_exponent");
}

#[test]
fn test_param_with_mixed_types_and_float() {
    run_parse_test("param_with_mixed_types_and_float");
}

#[test]
fn test_param_with_mixed_types_and_real() {
    run_parse_test("param_with_mixed_types_and_real");
}

#[test]
fn test_param_with_mixed_types_and_string() {
    run_parse_test("param_with_mixed_types_and_string");
}

#[test]
fn test_param_with_mixed_types_and_comment_in_value() {
    run_parse_test("param_with_mixed_types_and_comment_in_value");
}

#[test]
fn test_param_with_mixed_types_and_block_comment_in_value() {
    run_parse_test("param_with_mixed_types_and_block_comment_in_value");
}

#[test]
fn test_param_with_mixed_types_and_multiline_comment() {
    run_parse_test("param_with_mixed_types_and_multiline_comment");
}

#[test]
fn test_param_with_mixed_types_and_nested_block_comment() {
    run_parse_test("param_with_mixed_types_and_nested_block_comment");
}

#[test]
fn test_param_with_mixed_types_and_very_long_name() {
    run_parse_test("param_with_mixed_types_and_very_long_name");
}

#[test]
fn test_param_with_mixed_types_and_very_long_value() {
    run_parse_test("param_with_mixed_types_and_very_long_value");
}

#[test]
fn test_param_with_mixed_types_and_very_long_name_and_value() {
    run_parse_test("param_with_mixed_types_and_very_long_name_and_value");
}

#[test]
fn test_param_with_mixed_types_and_empty_value() {
    run_parse_test("param_with_mixed_types_and_empty_value");
}

#[test]
fn test_param_with_mixed_types_and_only_spaces() {
    run_parse_test("param_with_mixed_types_and_only_spaces");
}

#[test]
fn test_param_with_mixed_types_and_only_tabs() {
    run_parse_test("param_with_mixed_types_and_only_tabs");
}

#[test]
fn test_param_with_mixed_types_and_only_newlines() {
    run_parse_test("param_with_mixed_types_and_only_newlines");
}

#[test]
fn test_param_with_mixed_types_and_only_crlf() {
    run_parse_test("param_with_mixed_types_and_only_crlf");
}

#[test]
fn test_param_with_mixed_types_and_only_whitespace() {
    run_parse_test("param_with_mixed_types_and_only_whitespace");
}

#[test]
fn test_param_with_mixed_types_and_unicode() {
    run_parse_test("param_with_mixed_types_and_unicode");
}

#[test]
fn test_param_with_mixed_types_and_special_chars() {
    run_parse_test("param_with_mixed_types_and_special_chars");
}

#[test]
fn test_param_with_mixed_types_and_mixed_base() {
    run_parse_test("param_with_mixed_types_and_mixed_base");
}

#[test]
fn test_param_with_mixed_types_and_mixed_sized() {
    run_parse_test("param_with_mixed_types_and_mixed_sized");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_values() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_values");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_shorthand() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_shorthand");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_braces() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_braces");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_comments() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_comments");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_inline() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_inline");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_trailing() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_trailing");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_leading() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_leading");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_both() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_both");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_newline() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_newline");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_tab() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_tab");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_crlf() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_crlf");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_escaped_quote() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_escaped_quote");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_dollar() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_dollar");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_at() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_at");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_hash() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_hash");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_underscore_num() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_underscore_num");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_exponent() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_exponent");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_float() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_float");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_real() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_real");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_string() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_string");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_comment_in_value() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_comment_in_value");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_block_comment_in_value() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_block_comment_in_value");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_multiline_comment() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_multiline_comment");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_nested_block_comment() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_nested_block_comment");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_very_long_name() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_very_long_name");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_very_long_value() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_very_long_value");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_very_long_name_and_value() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_very_long_name_and_value");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_empty_value() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_empty_value");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_only_spaces() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_only_spaces");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_only_tabs() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_only_tabs");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_only_newlines() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_only_newlines");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_only_crlf() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_only_crlf");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_only_whitespace() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_only_whitespace");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_unicode() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_unicode");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_special_chars() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_special_chars");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_mixed_base() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_mixed_base");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_mixed_sized() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_mixed_sized");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_mixed_types_and_values() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_mixed_types_and_values");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_mixed_types_and_shorthand() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_mixed_types_and_shorthand");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_mixed_types_and_braces() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_mixed_types_and_braces");
}

#[test]
fn test_param_with_mixed_types_and_mixed_types_and_mixed_types_and_comments() {
    run_parse_test("param_with_mixed_types_and_mixed_types_and_mixed_types_and_comments");
}

// ---- Port direction tests ----

#[test]
fn test_port_no_type() {
    run_parse_test("port_no_type");
}

#[test]
fn test_port_no_type_dim() {
    run_parse_test("port_no_type_dim");
}

#[test]
fn test_port_mixed_style() {
    run_parse_test("port_mixed_style");
}

#[test]
fn test_port_multi_var() {
    run_parse_test("port_multi_var");
}

#[test]
fn test_port_multi_var_no_type() {
    run_parse_test("port_multi_var_no_type");
}

#[test]
fn test_port_multi_var_mixed() {
    run_parse_test("port_multi_var_mixed");
}

#[test]
fn test_port_no_comma() {
    run_parse_test("port_no_comma");
}

#[test]
fn test_port_no_comma_multi() {
    run_parse_test("port_no_comma_multi");
}

#[test]
fn test_port_trailing_comma() {
    run_parse_test("port_trailing_comma");
}

#[test]
fn test_port_signed() {
    run_parse_test("port_signed");
}

#[test]
fn test_port_logic() {
    run_parse_test("port_logic");
}

#[test]
fn test_port_wand() {
    run_parse_test("port_wand");
}

#[test]
fn test_port_tri() {
    run_parse_test("port_tri");
}

#[test]
fn test_port_triand() {
    run_parse_test("port_triand");
}

#[test]
fn test_port_trior() {
    run_parse_test("port_trior");
}

#[test]
fn test_port_tri0() {
    run_parse_test("port_tri0");
}

#[test]
fn test_port_tri1() {
    run_parse_test("port_tri1");
}

#[test]
fn test_port_supply0() {
    run_parse_test("port_supply0");
}

#[test]
fn test_port_supply1() {
    run_parse_test("port_supply1");
}

#[test]
fn test_port_integer() {
    run_parse_test("port_integer");
}

#[test]
fn test_port_time() {
    run_parse_test("port_time");
}

#[test]
fn test_port_packed() {
    run_parse_test("port_packed");
}

#[test]
fn test_port_ref() {
    run_parse_test("port_ref");
}

#[test]
fn test_port_const_ref() {
    run_parse_test("port_const_ref");
}

#[test]
fn test_port_output_ref() {
    run_parse_test("port_output_ref");
}

#[test]
fn test_port_inout_ref() {
    run_parse_test("port_inout_ref");
}

#[test]
fn test_port_dim_expr() {
    run_parse_test("port_dim_expr");
}

#[test]
fn test_port_dim_spaces() {
    run_parse_test("port_dim_spaces");
}

#[test]
fn test_port_multidim() {
    run_parse_test("port_multidim");
}

#[test]
fn test_port_wide() {
    run_parse_test("port_wide");
}

#[test]
fn test_port_reverse_range() {
    run_parse_test("port_reverse_range");
}

#[test]
fn test_port_single_bit() {
    run_parse_test("port_single_bit");
}

#[test]
fn test_underscore_name() {
    run_parse_test("underscore_name");
}

#[test]
fn test_underscore_param() {
    run_parse_test("underscore_param");
}

// ---- Comment tests ----

#[test]
fn test_inline_comment_port() {
    run_parse_test("inline_comment_port");
}

#[test]
fn test_block_comment_port() {
    run_parse_test("block_comment_port");
}

#[test]
fn test_comment_between_ports() {
    run_parse_test("comment_between_ports");
}

#[test]
fn test_block_comment_between_ports() {
    run_parse_test("block_comment_between_ports");
}

#[test]
fn test_port_inline_comment() {
    run_parse_test("port_inline_comment");
}

#[test]
fn test_port_block_comment() {
    run_parse_test("port_block_comment");
}

#[test]
fn test_port_comment_between() {
    run_parse_test("port_comment_between");
}

#[test]
fn test_port_comment_no_comma() {
    run_parse_test("port_comment_no_comma");
}

// ---- Pre/post content tests ----

#[test]
fn test_pre_content() {
    run_parse_test("pre_content");
}

#[test]
fn test_post_content() {
    run_parse_test("post_content");
}

// ---- Only inputs/outputs tests ----

#[test]
fn test_only_inputs() {
    run_parse_test("only_inputs");
}

#[test]
fn test_only_outputs() {
    run_parse_test("only_outputs");
}

// ---- Mixed directions tests ----

#[test]
fn test_mixed_directions() {
    run_parse_test("mixed_directions");
}

// ---- Multi-dim ports tests ----

#[test]
fn test_multi_dim_ports() {
    run_parse_test("multi_dim_ports");
}

// ---- Parameter edge cases tests ----

#[test]
fn test_param_with_block() {
    run_parse_test("param_with_block");
}

#[test]
fn test_param_with_line() {
    run_parse_test("param_with_line");
}

#[test]
fn test_param_with_inline() {
    run_parse_test("param_with_inline");
}

#[test]
fn test_param_with_multiline() {
    run_parse_test("param_with_multiline");
}

#[test]
fn test_param_with_trailing() {
    run_parse_test("param_with_trailing");
}

#[test]
fn test_param_with_leading() {
    run_parse_test("param_with_leading");
}

#[test]
fn test_param_with_block_leading() {
    run_parse_test("param_with_block_leading");
}

#[test]
fn test_param_with_block_trailing() {
    run_parse_test("param_with_block_trailing");
}

#[test]
fn test_param_with_both_comments() {
    run_parse_test("param_with_both_comments");
}

#[test]
fn test_param_with_newline() {
    run_parse_test("param_with_newline");
}

#[test]
fn test_param_with_tab() {
    run_parse_test("param_with_tab");
}

#[test]
fn test_param_with_crlf() {
    run_parse_test("param_with_crlf");
}

#[test]
fn test_param_with_escaped_quote() {
    run_parse_test("param_with_escaped_quote");
}

#[test]
fn test_param_with_dollar() {
    run_parse_test("param_with_dollar");
}

#[test]
fn test_param_with_at() {
    run_parse_test("param_with_at");
}

#[test]
fn test_param_with_hash() {
    run_parse_test("param_with_hash");
}

#[test]
fn test_param_with_underscore_num() {
    run_parse_test("param_with_underscore_num");
}

#[test]
fn test_param_with_exponent() {
    run_parse_test("param_with_exponent");
}

#[test]
fn test_param_with_float() {
    run_parse_test("param_with_float");
}

#[test]
fn test_param_with_real() {
    run_parse_test("param_with_real");
}

#[test]
fn test_param_with_string() {
    run_parse_test("param_with_string");
}

#[test]
fn test_param_with_hex_with_x() {
    run_parse_test("param_hex_with_x");
}

#[test]
fn test_param_with_hex_with_z() {
    run_parse_test("param_hex_with_z");
}

#[test]
fn test_param_with_large_hex() {
    run_parse_test("param_large_hex");
}

#[test]
fn test_param_with_bit_value() {
    run_parse_test("param_bit_value");
}

#[test]
fn test_param_with_signed_value() {
    run_parse_test("param_signed_value");
}

#[test]
fn test_param_with_unsigned_value() {
    run_parse_test("param_unsigned_value");
}

#[test]
fn test_param_typed_dim() {
    run_parse_test("param_typed_dim");
}

#[test]
fn test_param_typed_signed() {
    run_parse_test("param_typed_signed");
}

#[test]
fn test_param_typed_unsigned() {
    run_parse_test("param_typed_unsigned");
}

#[test]
fn test_param_typed_multi() {
    run_parse_test("param_typed_multi");
}

#[test]
fn test_param_with_comments() {
    run_parse_test("param_with_comments");
}

#[test]
fn test_param_brace_style() {
    run_parse_test("param_brace_style");
}

#[test]
fn test_param_shorthand_brace() {
    run_parse_test("param_shorthand_brace");
}

#[test]
fn test_param_no_space() {
    run_parse_test("param_no_space");
}

#[test]
fn test_param_multi_space() {
    run_parse_test("param_multi_space");
}

#[test]
fn test_param_tab() {
    run_parse_test("param_tab");
}

#[test]
fn test_param_newline() {
    run_parse_test("param_newline");
}

#[test]
fn test_param_block_comment() {
    run_parse_test("param_block_comment");
}

#[test]
fn test_param_inline_comment() {
    run_parse_test("param_inline_comment");
}

#[test]
fn test_param_line_comment() {
    run_parse_test("param_line_comment");
}

#[test]
fn test_param_multiline() {
    run_parse_test("param_multiline");
}

#[test]
fn test_param_deep_nesting() {
    run_parse_test("param_deep_nesting");
}

// ---- Semicolon tests ----

#[test]
fn test_semicolon_port_list() {
    run_parse_test("semicolon_port_list");
}

#[test]
fn test_semicolon_after_parens() {
    run_parse_test("semicolon_after_parens");
}

#[test]
fn test_no_semicolon() {
    run_parse_test("no_semicolon");
}

#[test]
fn test_whitespace_port_list() {
    run_parse_test("whitespace_port_list");
}

#[test]
fn test_parameter_edge_cases() {
    run_parse_test("parameter_edge_cases");
}

#[test]
fn test_typed_params() {
    run_parse_test("typed_params");
}

#[test]
fn test_special_chars_params() {
    run_parse_test("special_chars_params");
}
