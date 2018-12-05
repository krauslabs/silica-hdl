
#[derive(Clone, Debug, PartialEq)]
pub struct Mod(pub String, pub Vec<Port>, pub Vec<Stmt>);

#[derive(Clone, Debug, PartialEq)]
pub struct Port(pub Dir, pub String, pub Type);

#[derive(Clone, Debug, PartialEq)]
pub enum Dir {
	Input,
	Output,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
	Bit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
	Assign(String, Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
	Binary(Box<Expr>, BinaryOp, Box<Expr>),
	Unary(UnaryOp, Box<Expr>),
	Paren(Box<Expr>),
	Ident(String),
	Litrl(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
	ShiftLeft,
	ShiftRight,
	BitAnd,
	BitXor,
	BitOr,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
	Negate,
	ReductAnd,
	ReductXor,
	ReductOr,
}
