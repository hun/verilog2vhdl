module param_typed_multi(
    parameter int WIDTH = 8,
    parameter bit FLAG = 1,
    parameter logic [7:0] DATA = 8'hAA,
    parameter signed [31:0] OFFSET = 32'sh0
)(
    input wire clk
);
endmodule
