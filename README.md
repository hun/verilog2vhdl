# verilog2vhdl

Convert Verilog / SystemVerilog module definitions to VHDL entity declarations.

## Overview

`verilog2vhdl` parses Verilog and SystemVerilog source files using a proper Pest-based grammar (not regex or line-based tricks) and emits corresponding VHDL entity + architecture stub files. It is designed for rapid IP reuse where the RTL logic is written in Verilog but the top-level testbench or system integration uses VHDL.

### What it does

- **Parses** Verilog/SystemVerilog `module` definitions
- **Extracts** port declarations (direction, type, dimensions, comments)
- **Extracts** parameters and maps them to VHDL generics
- **Maps** Verilog types to VHDL types (`wire`/`reg`/`logic` -> `std_logic_vector`, `signed`/`unsigned` preserved)
- **Preserves** leading comments on ports and parameters as VHDL `--` comments
- **Outputs** a complete VHDL file with library clauses, entity declaration, and architecture stub

### What it does NOT do

- Internal logic translation (architecture body is a stub)
- Task blocks, initial blocks, or generate blocks
- Packed structs, enums, or macros (future work)

## Installation

### From source (Rust toolchain required)

```bash
git clone https://github.com/your-org/verilog2vhdl.git
cd verilog2vhdl
cargo build --release
cargo install --path .
```

### Requirements

- Rust 1.70+ (edition 2021)

## CLI Usage

```
verilog2vhdl [-o <OUTPUT>] [INPUT]

Arguments:
  [INPUT]     Input Verilog/SystemVerilog file (.v, .sv) or '-' for stdin

Options:
  -o, --output <OUTPUT>  Output VHDL file (default: stdout)
  -e, --entity-only      Print only the VHDL entity (no library/use clauses, no architecture stub)
  -h, --help             Print help
  -V, --version          Print version
```

### Examples

```bash
# Convert a file to stdout
verilog2vhdl adder.v

# Convert to a file
verilog2vhdl module.sv -o module.vhd

# Read from stdin (pipe or heredoc)
echo 'module foo(input wire clk); endmodule' | verilog2vhdl
verilog2vhdl - < my_design.v
cat my_design.v | verilog2vhdl -o my_design.vhd

# Entity-only (no library clauses or architecture stub)
echo 'module bar(input wire [7:0] a, output wire [7:0] b); endmodule' | verilog2vhdl -e
```

## Library Usage

`verilog2vhdl` is also a Rust library. Import it to use programmatically:

```toml
# Cargo.toml
[dependencies]
verilog2vhdl = { git = "https://github.com/hun/verilog2vhdl.git" }
```

```rust
use verilog2vhdl::parser::parse;
use verilog2vhdl::converter::convert_to_vhdl;

let input = std::fs::read_to_string("design.v")?;
let modules = parse(&input)?;
let vhdl = convert_to_vhdl(&modules);
```

## Emacs Integration

The included `verilog2vhdl.el` lets you convert Verilog code directly from Emacs without leaving the editor.

### Setup

Add the project directory to your `load-path`:

```elisp
(add-to-list 'load-path "/path/to/verilog2vhdl/")
(require 'verilog2vhdl)
```

Optionally set the executable path explicitly to avoid PATH lookups:

```elisp
(setq verilog2vhdl-program "/path/to/verilog2vhdl/target/release/verilog2vhdl")
```

### Commands

| Command | Description |
|---|---|
| `M-x verilog2vhdl-region` | Convert the selected region to VHDL |
| `M-x verilog2vhdl-buffer` | Convert the entire current buffer |

### Workflow

1. Select a region of Verilog code (or call the buffer command).
2. Run `M-x verilog2vhdl-region`.
3. The VHDL output is copied to the kill ring — paste with `C-y` into any VHDL source buffer.
4. The temporary buffer and temp file are cleaned up automatically.

### `vhdl-port-copy`

When `vhdl-mode` is loaded, the output is passed to `vhdl-port-copy` (which needs the cursor inside an entity declaration). To make this work, `--entity-only` is passed to `verilog2vhdl` so the temp buffer contains only the entity — no library clauses or architecture stub. The result is formatted as a port declaration ready to paste into an entity.

If `vhdl-mode` is not loaded, the full VHDL output is placed on the kill ring via `kill-new`.

With `C-u` prefix (`C-u M-x verilog2vhdl-region`), you are prompted for the path to the `verilog2vhdl` executable.

## Input Examples

### Simple module with ports

```verilog
module adder(
    input wire clk,
    input wire rst_n,
    input wire [7:0] a,
    input wire [7:0] b,
    output reg [7:0] sum
);
    // internal logic ...
endmodule
```

### Module with parameters

```verilog
module fifo #(
    parameter DATA_WIDTH = 8,
    parameter ADDR_SIZE = 4
)(
    input wire clk,
    input wire [DATA_WIDTH-1:0] data_in,
    output wire [ADDR_SIZE-1:0] count
);
endmodule
```

### Signed / Unsigned types

```verilog
module signed_mul #(
    parameter WIDTH = 16
)(
    input signed [WIDTH-1:0] a,
    input unsigned [WIDTH-1:0] b,
    output signed [WIDTH-1:0] result
);
endmodule
```

## Output Examples

### Simple module -> VHDL entity

```vhdl
library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity adder is
    port (
        clk: in std_logic;
        rst_n: in std_logic;
        a: in std_logic_vector(7 downto 0);
        b: in std_logic_vector(7 downto 0);
        sum: out std_logic_vector(7 downto 0)
    );
end entity adder;

architecture rtl of adder is
begin
    -- Internal logic stub
end architecture rtl;
```

### Parameterized module -> VHDL generic

```vhdl
entity fifo is
    generic (
        DATA_WIDTH : integer := 8;
        ADDR_SIZE : integer := 4
    );
    port (
        clk: in std_logic;
        data_in: in std_logic_vector(DATA_WIDTH-1 downto 0);
        count: out std_logic_vector(ADDR_SIZE-1 downto 0)
    );
end entity fifo;
```

## Supported Constructs

| Feature | Status |
|---|---|
| Old-style ports (`input wire [7:0] a`) | Yes |
| New-style ports (`input [7:0] a`) | Yes |
| Port directions (`in`, `out`, `inout`, `ref`) | Yes |
| Types (`wire`, `reg`, `logic`, `signed`, `unsigned`) | Yes |
| Multi-dimensional arrays | Yes |
| Module parameters -> VHDL generics | Yes |
| Comments (leading port/param comments) | Yes |
| Multiple modules per file | Yes |
| `endmodule` terminator | Yes |
| Expression-based dimensions (`DATA_WIDTH-1`) | Yes |
| Packed structs | Planned |
| Enum types | Planned |
| Macro expansion | Not planned |
| Internal logic translation | Not planned |

## Test Suite

```bash
# All tests (unit + integration)
cargo test

# Unit tests only (parser)
cargo test --lib

# Integration tests only (end-to-end conversion)
cargo test --test conversion_tests
```

28 tests total: 23 parser unit tests + 5 integration tests.

## Architecture

```
verilog2vhdl/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Library re-exports
│   ├── main.rs          # CLI entry point (clap)
│   ├── parser/
│   │   ├── mod.rs       # Re-exports
│   │   ├── verilog.pest # Pest grammar
│   │   ├── verilog.rs   # Parser implementation + tests
│   │   └── ast.rs       # AST node definitions
│   └── converter/
│       ├── mod.rs       # Convert entry point
│       ├── verilog_type.rs # Type mapping
│       ├── port.rs          # Port conversion
│       └── module_conv.rs   # Module -> entity conversion
├── tests/
│   ├── conversion_tests.rs  # Integration tests
│   └── cases/           # .v input + .vhd expected output pairs
├── examples/
│   └── simple_adder.v
├── verilog2vhdl.el      # Emacs integration (verilog2vhdl-region, verilog2vhdl-buffer)
└── PLAN.md              # Implementation roadmap
```

## License

MIT or Apache-2.0, same as Rust.
