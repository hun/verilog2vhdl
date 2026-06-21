module param_with_mixed_types_and_mixed_sized(
    parameter SIGNED_8 = 8'shFF
    ,
    parameter UNSIGNED_8 = 8'ushFF
    ,
    parameter SIGNED_16 = 16'shFFFF
    ,
    parameter UNSIGNED_16 = 16'uhFFFF
    ,
    parameter WIDTH = 8
)(
    input wire clk
);
endmodule
