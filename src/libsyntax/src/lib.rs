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

#[macro_use] extern crate lalrpop_util;
extern crate regex;

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

	use super::*;

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
}