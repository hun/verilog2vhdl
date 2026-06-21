module param_with_mixed_types_and_mixed_types_and_mixed_types_and_shorthand(
    parameter int WIDTH = 8
    ,
    DEPTH = 16
    ,
    parameter logic [7:0] DATA = 8'hAA
    ,
    ENABLE = 1
)(
    input wire clk
);
endmodule
