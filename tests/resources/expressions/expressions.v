module expressions (
	input wire a,
	input wire b,
	output wire x,
	output wire y,
	output wire z
);
    assign x = a << 1;
	assign y = b >> 1;
	assign z = ( a & b ) | x;
endmodule