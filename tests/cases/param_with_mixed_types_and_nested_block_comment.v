module param_with_mixed_types_and_nested_block_comment(
    parameter NAME = "hello"  /* outer /* nested */ still outer */
    ,
    parameter WIDTH = 8
)(
    input wire clk
);
endmodule
