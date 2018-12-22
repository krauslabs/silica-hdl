module simple_mod ( 
	input wire a, 
	input wire b, 
	input wire c, 
	input wire d, 
	output wire x, 
	output wire y, 
	output wire z 
); 
	assign x = a << 1 >> 1 & b ^ c | d; 
	assign y = ~a & &a & ^a & |a; 
	assign z = ( a & b ) | c; 
endmodule 
