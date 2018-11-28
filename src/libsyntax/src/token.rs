
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

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "mod" => Token::Mod,
        "top" => Token::Top,
        "in" => Token::In,
        "out" => Token::Out,
        "bit" => Token::Bit,
        _ => Token::Ident(ident.to_string()),
    }
}