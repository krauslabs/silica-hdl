module expressions (
	input wire a,
	input wire b,
	input wire c,
	input wire d,
	output wire y
);
	assign y = ( a << 1 ) >> 2 & ( b ^ c ) | d;
endmodule