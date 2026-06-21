module param_with_mixed_types_and_multiline_comment(
    parameter NAME = "hello"  /*
        multiline
        comment
    */
    ,
    parameter WIDTH = 8
)(
    input wire clk
);
endmodule
