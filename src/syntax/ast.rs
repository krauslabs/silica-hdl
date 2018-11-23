
#[derive(Clone, Debug)]
pub struct Ast {
	pub top: Box<Mod>,
}

#[derive(Clone, Debug)]
pub struct Mod {
	pub id: Ident,
	pub ports: Vec<Box<Port>>,
	pub stmts: Vec<Box<Stmt>>,
}

#[derive(Clone, Debug)]
pub struct Port {
	pub dir: PortDir,
	pub id: Ident,
	pub ty: Type,
}

#[derive(Clone, Debug)]
pub enum PortDir {
	Input,
	Output,
}

#[derive(Clone, Debug)]
pub enum Type {
	Bool,
}

#[derive(Clone, Debug)]
pub struct Stmt {
	pub kind: StmtKind,
}

#[derive(Clone, Debug)]
pub enum StmtKind {
	Assign(AssignStmt),
}

#[derive(Clone, Debug)]
pub struct AssignStmt {
	pub id: Ident,
	pub expr: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Expr {
	pub kind: ExprKind,
}

#[derive(Clone, Debug)]
pub enum ExprKind {
	Binary(BinaryExpr),
	Ident(Ident),
	Litrl(Litrl),
}

#[derive(Clone, Debug)]
pub struct BinaryExpr {
	pub op: BinaryOp,
	pub ex1: Box<Expr>,
	pub ex2: Box<Expr>,
}

#[derive(Clone, Debug)]
pub enum BinaryOp {
	BitAnd,
	BitOr,
}

#[derive(Clone, Debug)]
pub struct Ident {
	pub val: String,
}

#[derive(Clone, Debug)]
pub struct Litrl {
	pub val: String,
}


