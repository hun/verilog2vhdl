module param_with_mixed_types_and_mixed_types_and_leading(
    // leading
    parameter int WIDTH = 8
    ,
    /* leading */
    parameter DEPTH = 16
    ,
    // leading
    parameter logic [7:0] DATA = 8'hAA
)(
    input wire clk
);
endmodule
