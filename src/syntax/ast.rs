
#[derive(Clone, Debug, PartialEq)]
pub struct Mod {
	pub name: String,
	pub ports: Vec<Port>,
	pub stmts: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Port {
	pub dir: Dir, 
	pub name: String, 
	pub ty: Type,
}

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
	Assign{id: String, ex: Expr},
	Declare{id: String, ty: Type},
	DeclareAssign{id: String, ty: Type, ex: Expr},
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
	Binary{lex: Box<Expr>, op: BinaryOp, rex: Box<Expr>},
	Unary{op: UnaryOp, ex: Box<Expr>},
	Paren{ex: Box<Expr>},
	Ident{id: String},
	Litrl{val: String},
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
