use crate::{
    lexer::{InvalidCharacterError, Token},
    BytePos, Error,
};

use lalrpop_util::ParseError;

impl From<ParseError<BytePos, Token, InvalidCharacterError>> for Error {
    fn from(error: ParseError<BytePos, Token, InvalidCharacterError>) -> Self {
        match error {
            ParseError::InvalidToken { location } => Error::InvalidToken { pos: location },
            ParseError::UnrecognizedToken { token, expected } => {
                Error::UnrecognizedToken { token, expected }
            }
            ParseError::ExtraToken { token } => Error::ExtraToken { token },
            ParseError::User {
                error: InvalidCharacterError { ch, pos },
            } => Error::InvalidCharacter { pos, ch },
        }
    }
}
