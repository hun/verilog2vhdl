module param_with_mixed_types_and_mixed_types_and_mixed_types_and_values(
    parameter int WIDTH = 8
    ,
    parameter bit FLAG = 1
    ,
    parameter logic [7:0] DATA = 8'hAA
    ,
    parameter signed [31:0] OFFSET = 32'shDEAD
    ,
    parameter unsigned [15:0] VALUE = 16'uhCAFE
)(
    input wire clk
);
endmodule
