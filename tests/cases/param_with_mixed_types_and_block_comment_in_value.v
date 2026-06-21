module param_with_mixed_types_and_block_comment_in_value(
    parameter NAME = "hello"  /* block */
    ,
    parameter WIDTH = 8
)(
    input wire clk
);
endmodule
