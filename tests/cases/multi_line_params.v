module multi_line_params(
    parameter WIDTH = 8,
    parameter DEPTH = 16,
    parameter ENABLE = 1
)(
    input wire clk,
    input wire [WIDTH-1:0] addr,
    output wire [WIDTH-1:0] data
);
endmodule
