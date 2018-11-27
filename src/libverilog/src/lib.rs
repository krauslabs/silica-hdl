//! The Verilog Code Generator
//!
//! The verilog code generator creates the verilog source that is the output
//! of the Silica compiler. It uses data created from previous parts of the
//! compilation process (such as the AST and name resolution tables) to 
//! generate suitable output code.

extern crate syntax;

use syntax::ast::*;
use syntax::visit::Visitor;

pub struct Verilog {
	verilog: String,
}

impl Verilog {
	pub fn new() -> Verilog {
		Verilog { verilog: String::new(), }
	}

	pub fn build(&mut self, ast: &syntax::Ast) -> String {
		self.visit_mod(&ast.top);
		self.verilog.clone()
	}
}

impl Visitor for Verilog {

	fn visit_mod(&mut self, m: &Mod) {
		let Mod(id, ports, stmts) = m;

		self.verilog.push_str("module ");
		self.visit_ident(id);
		self.verilog.push_str(" ( \n");

		let port_len = ports.len();
		for (idx, port) in ports.iter().enumerate() {
			self.verilog.push_str("\t");
			self.visit_port(port);
			if idx == (port_len - 1) {
				self.verilog.push_str(" \n");
			} else {
				self.verilog.push_str(", \n");
			}
		}

		self.verilog.push_str("); \n");

		for stmt in stmts.iter() {
			self.verilog.push_str("\t");
			self.visit_stmt(stmt);
			self.verilog.push_str(" \n");
		}

		self.verilog.push_str("endmodule \n");
	}

	fn visit_port(&mut self, p: &Port) {
		let Port(dir, id, ty) = p;
		self.visit_dir(dir);
		self.verilog.push_str(" ");
		self.visit_type(ty);
		self.verilog.push_str(" ");
		self.visit_ident(id);
	}

	fn visit_dir(&mut self, d: &Dir) {
		match d {
			Dir::Input => {
				self.verilog.push_str("input");
			}
			Dir::Output => {
				self.verilog.push_str("output");
			}
		}
	}

	fn visit_type(&mut self, t: &Type) {
		match t {
			Type::Bit => {
				self.verilog.push_str("wire");
			}
		}
	}

	fn visit_stmt(&mut self, s: &Stmt) {
		match s {
			Stmt::Assign(ref id, ref ex) => {
				self.verilog.push_str("assign ");
				self.visit_ident(id);
				self.verilog.push_str(" = ");
				self.visit_expr(ex);
				self.verilog.push_str(";");
			}
		}
	}

	fn visit_expr(&mut self, e: &Expr) {
		match e {
			Expr::Binary(ref ex1, ref op, ref ex2) => {
				self.visit_expr(ex1);
				self.verilog.push_str(" ");
				self.visit_binary_op(op);
				self.verilog.push_str(" ");
				self.visit_expr(ex2);
			}
			Expr::Paren(ref ex) => {
				self.verilog.push_str("( ");
				self.visit_expr(ex);
				self.verilog.push_str(" )");
			}
			Expr::Ident(ref id) => {
				self.visit_ident(id);
			}
			Expr::Litrl(ref lit) => {
				self.visit_litrl(lit);
			}
		}
	}

	fn visit_binary_op(&mut self, b: &BinaryOp) {
		match b {
			BinaryOp::ShiftLeft => {
				self.verilog.push_str("<<");
			}
			BinaryOp::ShiftRight => {
				self.verilog.push_str(">>");
			}
			BinaryOp::BitAnd => {
				self.verilog.push_str("&");
			}
			BinaryOp::BitXor => {
				self.verilog.push_str("^");
			}
			BinaryOp::BitOr => {
				self.verilog.push_str("|");
			}
		}
	}

	fn visit_ident(&mut self, i: &Ident) {
		let Ident(s) = i;
		self.verilog.push_str(s);
	}

	fn visit_litrl(&mut self, l: &Litrl) {
		let Litrl(s) = l;
		self.verilog.push_str(s);
	}
}



