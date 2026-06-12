module fifo #(
    // Data width
    parameter DATA_WIDTH = 8,
    // Depth exponent
    parameter ADDR_SIZE = 4
)(
    input wire clk,
    input wire rst,
    input wire wr_en,
    input wire rd_en,
    input wire [DATA_WIDTH-1:0] data_in,
    output reg [DATA_WIDTH-1:0] data_out,
    output reg [ADDR_SIZE-1:0] count
);

    reg [DATA_WIDTH-1:0] mem [0:(1<<ADDR_SIZE)-1];
    reg [ADDR_SIZE:0] wr_ptr, rd_ptr;

    always @(posedge clk) begin
        if (wr_en)
            mem[wr_ptr] <= data_in;
        if (rd_en)
            data_out <= mem[rd_ptr];
    end

endmodule
