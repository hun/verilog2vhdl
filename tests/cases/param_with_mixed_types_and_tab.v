module param_with_mixed_types_and_tab(
	parameter int WIDTH = 8
	,
	parameter DEPTH = 16
	,
	parameter logic [7:0] DATA = 8'hAA
)(
	input wire clk
);
endmodule
