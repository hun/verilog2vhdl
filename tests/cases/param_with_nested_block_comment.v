module param_with_nested_block_comment(
    parameter WIDTH = 8  /* outer /* nested */ still outer */
)(
    input wire clk
);
endmodule
