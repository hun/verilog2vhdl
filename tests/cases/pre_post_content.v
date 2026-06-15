// Test file with pre- and post-module content
// This tests that default_nettype, includes, and trailing text are ignored

`timescale 1ns / 1ps
`define BUS_WIDTH 8
`include "common_defines.v"

default_nettype wire
default_nettype none

/*
 * Multi-line block comment at top
 */

module pre_post_content(
    // Clock signal
    input wire clk,
    // Reset signal (active low)
    input wire rst_n,
    // Input data
    input wire [7:0] data_in,
    // Output data
    output reg [7:0] data_out
);

    always @(posedge clk) begin
        if (rst_n)
            data_out <= 8'd0;
        else
            data_out <= data_in;
    end

endmodule

// Trailing content after endmodule
// This should be completely ignored
`ifdef SIMULATION
`define DEBUG_MODE
`endif

default_nettype wire
`include "trailing_include.v"
