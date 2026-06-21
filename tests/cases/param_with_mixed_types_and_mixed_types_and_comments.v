module param_with_mixed_types_and_mixed_types_and_comments(
    // width parameter
    parameter int WIDTH = 8
    ,
    /* depth parameter */
    parameter DEPTH = 16
    ,
    // data parameter
    parameter logic [7:0] DATA = 8'hAA
)(
    input wire clk
);
endmodule
