module param_expression(
    parameter WIDTH = 8 + 16,
    parameter DEPTH = 256 / 4,
    parameter MASK = 16 % 5
)(
    input wire clk
);
endmodule
