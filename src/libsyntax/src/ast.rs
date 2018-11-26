
#[derive(Clone, Debug)]
pub struct Mod(pub Ident, pub Vec<Port>, pub Vec<Stmt>);

#[derive(Clone, Debug)]
pub struct Port(pub Dir, pub Ident, pub Type);

#[derive(Clone, Debug)]
pub enum Dir {
	Input,
	Output,
}

#[derive(Clone, Debug)]
pub enum Type {
	Bool,
}

#[derive(Clone, Debug)]
pub enum Stmt {
	Assign(Ident, Expr),
}

#[derive(Clone, Debug)]
pub enum Expr {
	Binary(Box<Expr>, BinaryOp, Box<Expr>),
	Ident(Ident),
	Litrl(Litrl),
}

#[derive(Clone, Debug)]
pub enum BinaryOp {
	BitAnd,
	BitOr,
}

#[derive(Clone, Debug)]
pub struct Ident(pub String);

#[derive(Clone, Debug)]
pub struct Litrl(pub String);
