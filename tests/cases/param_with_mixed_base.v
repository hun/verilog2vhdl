module param_with_mixed_base(
    parameter BINARY = 8'b10101010,
    parameter HEX = 8'hFF,
    parameter OCTAL = 8'o77,
    parameter DECIMAL = 8'd255
)(
    input wire clk
);
endmodule
