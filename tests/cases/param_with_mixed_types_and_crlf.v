module param_with_mixed_types_and_crlf(
    parameter int WIDTH = 8\r\n    ,\r\n    parameter DEPTH = 16\r\n    ,\r\n    parameter logic [7:0] DATA = 8'hAA\r\n)(\r\n    input wire clk\r\n);\r\nendmodule
