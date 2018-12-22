use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod visit;

mod error;
mod lexer;
lalrpop_mod!(parser);

/// Source code byte offsets, used for spans and errors.
pub type BytePos = usize;

/// Syntax error types.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    InvalidToken {
        pos: BytePos,
    },
    UnrecognizedToken {
        token: Option<(BytePos, lexer::Token, BytePos)>,
        expected: Vec<String>,
    },
    ExtraToken {
        token: (BytePos, lexer::Token, BytePos),
    },
    InvalidCharacter {
        pos: BytePos,
        ch: char,
    },
}

/// Parses source code into an AST.
pub fn parse_source(source: &str) -> Result<ast::Ast, Error> {
    let lexer = lexer::Lexer::new(source);
    let ast = parser::SourceFileParser::new().parse(lexer)?;

    Ok(ast)
}
