module param_with_mixed_types_and_comment_in_value(
    parameter NAME = "hello"  // comment
    ,
    parameter WIDTH = 8
)(
    input wire clk
);
endmodule
