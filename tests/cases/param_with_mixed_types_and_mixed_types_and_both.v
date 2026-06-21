module param_with_mixed_types_and_mixed_types_and_both(
    // leading
    parameter int WIDTH = 8  // inline
    ,
    /* leading */
    parameter DEPTH = 16  /* inline */
    ,
    // leading
    parameter logic [7:0] DATA = 8'hAA  // inline
)(
    input wire clk
);
endmodule
