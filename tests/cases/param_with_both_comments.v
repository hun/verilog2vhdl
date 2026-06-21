module param_with_both_comments(
    // leading
    parameter WIDTH = 8  // inline
    ,
    /* block between */
    parameter DEPTH = 16
)(
    input wire clk
);
endmodule
