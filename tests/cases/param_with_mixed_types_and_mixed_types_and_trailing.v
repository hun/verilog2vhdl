module param_with_mixed_types_and_mixed_types_and_trailing(
    parameter int WIDTH = 8  // trailing
    ,
    parameter DEPTH = 16  /* trailing */
    ,
    parameter logic [7:0] DATA = 8'hAA
)(
    input wire clk
);
endmodule
