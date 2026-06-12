module signed_unsigned #(
    parameter WIDTH = 16
)(
    // Signed input
    input signed [WIDTH-1:0] a,
    // Unsigned input
    input unsigned [WIDTH-1:0] b,
    // Signed result
    output signed [WIDTH-1:0] sum,
    // Unsigned result
    output unsigned [WIDTH-1:0] product
);

    assign sum = a + WIDTH'(b);
    assign product = b * WIDTH'(a);

endmodule
