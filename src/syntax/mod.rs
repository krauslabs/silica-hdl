pub mod ast;
lalrpop_mod!(grammar);

use self::ast::Mod;

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

	pub fn parse(&self) -> Box<Mod> {
		grammar::TopModParser::new().parse(&self.input).unwrap()
	}
}
