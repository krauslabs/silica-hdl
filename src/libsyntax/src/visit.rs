#![allow(unused_variables)]

use crate::ast::*;

pub trait Visitor: Sized {
	fn visit_mod(&mut self, m: &Mod) { walk_mod(self, m); }
	fn visit_port(&mut self, p: &Port) { walk_port(self, p); }
	fn visit_dir(&mut self, d: &Dir) { walk_dir(self, d); }
	fn visit_type(&mut self, t: &Type) { walk_type(self, t); }
	fn visit_stmt(&mut self, s: &Stmt) { walk_stmt(self, s); }
	fn visit_expr(&mut self, e: &Expr) { walk_expr(self, e); }
	fn visit_unary_op(&mut self, u: &UnaryOp) { walk_unary_op(self, u); }
	fn visit_binary_op(&mut self, b: &BinaryOp) { walk_binary_op(self, b); }
}

pub fn walk_mod<V: Visitor>(visitor: &mut V, m: &Mod) {
	let Mod{name, ports, stmts} = m;

	for port in ports {
		visitor.visit_port(port);
	}
	for stmt in stmts {
		visitor.visit_stmt(stmt);
	}
}

pub fn walk_port<V: Visitor>(visitor: &mut V, p: &Port) {
	let Port{dir, name, ty} = p;

	visitor.visit_dir(dir);
	visitor.visit_type(ty);
}

pub fn walk_dir<V: Visitor>(visitor: &mut V, d: &Dir) {
	
}

pub fn walk_type<V: Visitor>(visitor: &mut V, t: &Type) {
	
}

pub fn walk_stmt<V: Visitor>(visitor: &mut V, s: &Stmt) {
	match s {
		Stmt::Assign{id, ex} => {
			visitor.visit_expr(ex);
		}
	}
}

pub fn walk_expr<V: Visitor>(visitor: &mut V, e: &Expr) {
	match e {

		Expr::Binary{lex, op, rex} => {
			visitor.visit_expr(lex);
			visitor.visit_binary_op(op);
			visitor.visit_expr(rex);
		}
		Expr::Unary{op, ex} => {
			visitor.visit_unary_op(op);
		}
		Expr::Paren{ex} => {
			visitor.visit_expr(ex);
		}
		Expr::Ident{id} => {
		}
		Expr::Litrl{val} => {
		}
	}
}

pub fn walk_unary_op<V: Visitor>(visitor: &mut V, u: &UnaryOp) {
	
}

pub fn walk_binary_op<V: Visitor>(visitor: &mut V, b: &BinaryOp) {
	
}
