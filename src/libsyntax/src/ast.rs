
#[derive(Clone, Debug, PartialEq)]
pub struct Mod(pub Ident, pub Vec<Port>, pub Vec<Stmt>);

#[derive(Clone, Debug, PartialEq)]
pub struct Port(pub Dir, pub Ident, pub Type);

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
	Assign(Ident, Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
	Binary(Box<Expr>, BinaryOp, Box<Expr>),
	Unary(UnaryOp, Box<Expr>),
	Paren(Box<Expr>),
	Ident(Ident),
	Litrl(Litrl),
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

#[derive(Clone, Debug, PartialEq)]
pub struct Ident(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct Litrl(pub String);
