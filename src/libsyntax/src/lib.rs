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

lalrpop_mod!(grammar);

#[derive(Clone, Debug, PartialEq)]
pub struct Ast {
	pub top: ast::Mod,
}

impl Ast {
	pub fn new(input: &str) -> Ast {

		// strip the input of all comments before returning the struct
		//
		// TODO: implement this in lalrpop grammar when possible, comments
		// stripped here cause byte indices of 'input' to not match the
		// actual file that they are taken from.
		let re = regex::Regex::new(r"//.*").unwrap();
		let input_stripped = re.replace_all(input, "");

		grammar::SourceFileParser::new().parse(&input_stripped).unwrap()
	}
}
