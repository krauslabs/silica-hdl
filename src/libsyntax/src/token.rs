
#[derive(Clone, Debug, PartialEq)]
pub enum Token {

	Ident(String),
	Litrl(String),

	// Punctuation
	Comma,
	Semicolon,
	Colon,
	LeftParen,
	RightParen,
	LeftCurlyBrace,
	RightCurlyBrace,

	// Operators
	Assign,
	Negate,
	BitAnd,
	BitOr,
	BitXor,
	ShiftLeft,
	ShiftRight,

	// Keywords
	Mod,
	Top,
	In,
	Out,
	Bit,
}