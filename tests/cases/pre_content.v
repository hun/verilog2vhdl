`timescale 1ns / 1ps
`define MY_PARAM 42

module pre_content(
    input wire clk
);
endmodule

`ifdef SIMULATION
`define DEBUG
`endif
