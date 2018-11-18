
#[derive(Debug)]
pub struct Mod {
	pub id: Ident,
	pub ports: Vec<Box<Port>>,
	pub stmts: Vec<Box<Stmt>>,
}

#[derive(Debug)]
pub struct Port {
	pub dir: PortDir,
	pub id: Ident,
	pub ty: Type,
}

#[derive(Debug)]
pub enum PortDir {
	Input,
	Output,
}

#[derive(Debug)]
pub enum Type {
	Bool,
}

#[derive(Debug)]
pub struct Stmt {
	pub kind: StmtKind,
}

#[derive(Debug)]
pub enum StmtKind {
	Assign(AssignStmt),
}

#[derive(Debug)]
pub struct AssignStmt {
	pub id: Ident,
	pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct Expr {
	pub kind: ExprKind,
}

#[derive(Debug)]
pub enum ExprKind {
	Binary(BinaryExpr),
	Ident(Ident),
	Litrl(Litrl),
}

#[derive(Debug)]
pub struct BinaryExpr {
	pub op: BinaryOp,
	pub ex1: Box<Expr>,
	pub ex2: Box<Expr>,
}

#[derive(Debug)]
pub enum BinaryOp {
	BitAnd,
	BitOr,
}

#[derive(Debug)]
pub struct Ident {
	pub val: String,
}

#[derive(Debug)]
pub struct Litrl {
	pub val: String,
}


