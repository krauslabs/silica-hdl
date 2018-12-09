//! The Parser
//!
//! The parser seeks to convert an input string (from a Silica source file) and
//! turn it into an AST (abstract syntax tree).
//!
//! The entry to the parser is the parse() function of the Parser struct, which
//! will start at the 'top module' of the input string and parse from there.
//!
//! TODO: Once multiple source files are supported the top level of parsing will
//! need to be changed to be something at the 'project' level, whether thats a
//! specific design or a 'library' or something else is to be determined.

use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod visit;

mod lexer;
lalrpop_mod!(grammar);

#[derive(Clone, Debug, PartialEq)]
pub struct Ast {
	pub top: ast::Mod,
}

impl Ast {
	pub fn new(input: &str) -> Ast {
		
		let lexer = lexer::Lexer::new(input);

		grammar::SourceFileParser::new().parse(lexer).unwrap()
	}
}

#[cfg(test)]
mod test {

	use crate::{Ast, ast::*};

	fn assert_expr(source: &str, expected: &Expr) {
		let module = format!("top mod a ( out y: bit ) {{ y = {}; }}", source);

		let ast = Ast::new(&module);
		let expr = match ast.top.stmts[0] {
			Stmt::Assign{id: _, ref ex} => ex,
			_ => panic!(),
		};

		assert_eq!(expr, expected);
	}

	fn assert_stmt(source: &str, expected: &Stmt) {
		let module = format!("top mod a ( out y: bit ) {{ {} }}", source);

		let ast = Ast::new(&module);
		let stmt = &ast.top.stmts[0];

		assert_eq!(stmt, expected);
	}

	#[test]
	fn assign_stmt() {
		assert_stmt(
			"y = 5;",
			&Stmt::Assign{
				id: "y".to_string(),
				ex: Expr::Litrl{val: "5".to_string()},
			}
		);
	}

	#[test]
	fn declare_stmt() {
		assert_stmt(
			"let y: bit;",
			&Stmt::Declare{
				id: "y".to_string(),
				ty: Type::Bit,
			}
		);
	}

	#[test]
	fn declare_assign_stmt() {
		assert_stmt(
			"let y: bit = 1;",
			&Stmt::DeclareAssign{
				id: "y".to_string(),
				ty: Type::Bit,
				ex: Expr::Litrl{val: "1".to_string()},
			}
		);
	}

 	#[test]
 	fn precedence() {
 		assert_expr(
 			"1 | 2 ^ 3 & & 4 << 5",
 			&Expr::Binary{
 				lex: Box::new(Expr::Litrl{val: "1".to_string()}),
 				op: BinaryOp::BitOr,
 				rex: Box::new(Expr::Binary{
 					lex: Box::new(Expr::Litrl{val: "2".to_string()}),
 					op: BinaryOp::BitXor,
 					rex: Box::new(Expr::Binary{
 						lex: Box::new(Expr::Litrl{val: "3".to_string()}),
 						op: BinaryOp::BitAnd,
 						rex: Box::new(Expr::Binary{
 							lex: Box::new(Expr::Unary{
 								op: UnaryOp::ReductAnd,
 								ex: Box::new(Expr::Litrl{val: "4".to_string()})
 							}),
 							op: BinaryOp::ShiftLeft,
 							rex: Box::new(Expr::Litrl{val: "5".to_string()}),
 						}),
 					}),
 				}),
 			}
 		);
 	}

 	#[test]
 	fn paren_grouping() {
 		assert_expr(
 			"1 << ( 2 | 3 ) >> 1",
 			&Expr::Binary{
 				lex: Box::new(Expr::Binary{
 					lex: Box::new(Expr::Litrl{val: "1".to_string()}),
 					op: BinaryOp::ShiftLeft,
 					rex: Box::new(Expr::Paren{
 						ex: Box::new(Expr::Binary{
 							lex: Box::new(Expr::Litrl{val: "2".to_string()}),
 							op: BinaryOp::BitOr,
 							rex: Box::new(Expr::Litrl{val: "3".to_string()}),
 						})
 					}),
 				}),
 				op: BinaryOp::ShiftRight,
 				rex: Box::new(Expr::Litrl{val: "1".to_string()}),
 			},
 		);
 	}

	#[test]
	fn trailing_comma() {

	    let no_comma = 
	        "top mod a (
	            in x: bit,
	            out y: bit
	        ) {
	            y = x;
	        }";

	    let comma = 
	        "top mod a (
	            in x: bit,
	            out y: bit,
	        ) {
	            y = x;
	        }";

	    let ast_no_comma = Ast::new(no_comma);
	    let ast_comma = Ast::new(comma);

	    assert_eq!(ast_comma, ast_no_comma);
	}

	#[test]
	fn module() {
		let source = 
			"top mod a (
	            out y: bit
	        ) {
	            y = 1;
	        }";

	    let ast = Ast::new(source);

	    let expected = Ast{
	    	top: Mod{
	    		name: "a".to_string(),
	    		ports: vec![Port{
	    			dir: Dir::Output,
	    			name: "y".to_string(),
	    			ty: Type::Bit,
	    		},],
	    		stmts: vec![Stmt::Assign{
	    			id: "y".to_string(),
	    			ex: Expr::Litrl{val: "1".to_string()},
	    		},],
	    	}
	    };

	    assert_eq!(ast, expected);
	}
}