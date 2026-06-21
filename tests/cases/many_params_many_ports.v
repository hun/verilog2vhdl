module many_params_many_ports #(
    parameter A = 1,
    parameter B = 2,
    parameter C = 3,
    parameter D = 4,
    parameter E = 5,
    parameter F = 6,
    parameter G = 7,
    parameter H = 8,
    parameter I = 9,
    parameter J = 10,
    parameter K = 11,
    parameter L = 12
)(
    input wire clk,
    input wire rst,
    input wire [7:0] addr,
    input wire [15:0] data_in,
    output wire [7:0] data_out,
    output wire write_en,
    output wire read_en,
    output wire ready,
    output wire [3:0] status,
    output wire [31:0] error_code
);
endmodule
