module simple_adder(
    // Clock signal
    input wire clk,
    // Reset signal (active low)
    input wire rst_n,
    // Input A
    input wire [7:0] a,
    // Input B
    input wire [7:0] b,
    // Sum output
    output reg [7:0] sum
);

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n)
            sum <= 8'd0;
        else
            sum <= a + b;
    end

endmodule
