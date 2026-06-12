module mux3d #(
    parameter N = 8,
    parameter M = 4
)(
    input wire [N*M-1:0] sel,
    input wire [N-1:0] [M-1:0] in_a,
    input wire [N-1:0] [M-1:0] in_b,
    output reg [N*M-1:0] out
);

    integer i;
    always @(*) begin
        for (i = 0; i < N; i = i + 1) begin
            if (sel[i])
                out[i*M +: M] <= in_a[i];
            else
                out[i*M +: M] <= in_b[i];
        end
    end

endmodule
