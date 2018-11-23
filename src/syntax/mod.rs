//! The Parser
//!
//! The parser seeks to convert an input string (from a Silica source file) and
//! turn it into an AST (abstract syntax tree).
//!
//! The entry to the parser is the parse() function of the Parser struct, which
//! will start at the 'top module' of the input string and parse from there.
//!
//! TODO: The top level of parsing should start atleast at the source file
//! level, as each source file can define multiple modules, constants, types,
//! etc... at its top level before defining the top module.
//!
//! TODO: Once multiple source files are supported the top level of parsing will
//! need to be changed to be something at the 'project' level, whether thats a
//! specific design or a 'library' or something else is to be determined.

pub mod ast;
lalrpop_mod!(grammar);

pub struct Parser {
	input: String,
}

impl Parser {
	pub fn new(input: &str) -> Parser {

		// strip the input of all comments before returning the struct
		//
		// TODO: implement this in lalrpop grammar when possible, comments
		// stripped here cause byte indices of 'input' to not match the
		// actual file that they are taken from.
		let re = regex::Regex::new(r"//.*").unwrap();
		let input_stripped = re.replace_all(input, "");

		Parser{ input: input_stripped.to_string() }
	}

	pub fn parse(&self) -> ast::Ast {
		grammar::SourceFileParser::new().parse(&self.input).unwrap()
	}
}
