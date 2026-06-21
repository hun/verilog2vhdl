module param_with_mixed_types_and_inline(
    parameter int WIDTH = 8  // width
    ,
    parameter DEPTH = 16  /* depth */
    ,
    parameter logic [7:0] DATA = 8'hAA  // data
)(
    input wire clk
);
endmodule
