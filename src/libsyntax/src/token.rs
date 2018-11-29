
#[derive(Clone, Debug, PartialEq)]
pub enum Token {

	// Identifiers
	Ident(String),

	// Literals
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