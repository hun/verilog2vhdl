module param_with_mixed_sized(
    parameter SIGNED_8 = 8'shFF,
    parameter UNSIGNED_8 = 8'ushFF,
    parameter SIGNED_16 = 16'shFFFF,
    parameter UNSIGNED_16 = 16'uhFFFF
)(
    input wire clk
);
endmodule
