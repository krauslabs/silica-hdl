use std::iter::Peekable;
use std::str::Chars;

use token::{self, Token};

pub type LexerItem = Result<Token, LexerError>;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
	pub fn new(input: &str) -> Lexer {
        Lexer { input: input.chars().peekable() }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_char_eq(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&peek_ch) => peek_ch == ch,
            None => false,
        }
    }

    fn is_letter(&mut self, ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn peek_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => self.is_letter(ch),
            None => false,
        }
    }

    fn is_number(&mut self, ch: char) -> bool {
        ch.is_numeric()
    }

    fn peek_is_number(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => self.is_number(ch),
            None => false,
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_letter() || self.peek_is_number() {
            ident.push(self.read_char().unwrap()); // TODO: unwrap()
        }

        ident
    }

    fn read_number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);

        while self.peek_is_number() {
            number.push(self.read_char().unwrap()); // TODO: unwrap()
        }

        number
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek_char() {
            if !ch.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn skip_line(&mut self) {
    	while !self.peek_char_eq('\n') {
    		self.read_char();
    	}
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerItem;

    fn next(&mut self) -> Option<LexerItem> {

        self.skip_whitespace();

        match self.read_char() {
            Some(',') => Some(Ok(Token::Comma)),
            Some(';') => Some(Ok(Token::Semicolon)),
            Some(':') => Some(Ok(Token::Colon)),
            Some('(') => Some(Ok(Token::LeftParen)),
            Some(')') => Some(Ok(Token::RightParen)),
            Some('{') => Some(Ok(Token::LeftCurlyBrace)),
            Some('}') => Some(Ok(Token::RightCurlyBrace)),
            Some('=') => Some(Ok(Token::Assign)),
            Some('~') => Some(Ok(Token::Negate)),
            Some('&') => Some(Ok(Token::BitAnd)),
            Some('|') => Some(Ok(Token::BitOr)),
            Some('^') => Some(Ok(Token::BitXor)),
            Some('<') => { 
                if self.peek_char_eq('<') {
                    self.read_char();
                    Some(Ok(Token::ShiftLeft))
                } else {
                    Some(Err(LexerError::InvalidCharacter('<')))
                }
            }
            Some('>') => { 
                if self.peek_char_eq('>') {
                    self.read_char();
                    Some(Ok(Token::ShiftRight))
                } else {
                    Some(Err(LexerError::InvalidCharacter('>')))
                }
            }
            Some('/') => {
                if self.peek_char_eq('/') {
                    self.skip_line();
                    self.next()
                } else {
                    Some(Err(LexerError::InvalidCharacter('/')))
                }
            }
            Some(ch @ _) => {
                if self.is_letter(ch) {
                    let ident = self.read_identifier(ch);
                    Some(Ok(token::lookup_ident(&ident)))
                } else if self.is_number(ch) {
                    let num = self.read_number(ch);
                    Some(Ok(Token::Litrl(num)))
                } else {
                    Some(Err(LexerError::InvalidCharacter(ch)))
                }
            }
            None => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    InvalidCharacter(char),
}

#[cfg(test)]
mod test {

    use super::*;

    fn assert_lex(source: &str, expected: Vec<LexerItem>) {
        let lexer = Lexer::new(source);

        for (idx, item) in lexer.enumerate() {
            assert_eq!(item, expected[idx])
        }
    }

    #[test]
    fn identifiers() {
        assert_lex(
            "id _id id123 modu mo ",
            vec![
                Ok(Token::Ident("id".to_string())),
                Ok(Token::Ident("_id".to_string())),
                Ok(Token::Ident("id123".to_string())),
                Ok(Token::Ident("modu".to_string())),
                Ok(Token::Ident("mo".to_string())),
            ]
        );
    }

    #[test]
    fn literal() {
        assert_lex(
            " 1 0123 ",
            vec![
                Ok(Token::Litrl("1".to_string())),
                Ok(Token::Litrl("0123".to_string())),
            ]
        );
    }

    #[test]
    fn comments() {
        assert_lex(
            " top // this shouldn't be seen \n mod ",
            vec![
                Ok(Token::Top),
                Ok(Token::Mod),
            ]
        );
    }

    #[test]
    fn keywords() {
        assert_lex(
            " mod top in out bit ",
            vec![
                Ok(Token::Mod),
                Ok(Token::Top),
                Ok(Token::In),
                Ok(Token::Out),
                Ok(Token::Bit),
            ]
        );
    }

    #[test]
    fn operators() {
        assert_lex(
            " = ~ & | ^ << >> ",
            vec![
                Ok(Token::Assign),
                Ok(Token::Negate),
                Ok(Token::BitAnd),
                Ok(Token::BitOr),
                Ok(Token::BitXor),
                Ok(Token::ShiftLeft),
                Ok(Token::ShiftRight),
            ]
        );
    }

    #[test]
    fn punctuation() {
        assert_lex(
            " , ; : ( ) { } ",
            vec![
                Ok(Token::Comma),
                Ok(Token::Semicolon),
                Ok(Token::Colon),
                Ok(Token::LeftParen),
                Ok(Token::RightParen),
                Ok(Token::LeftCurlyBrace),
                Ok(Token::RightCurlyBrace),
            ]
        );
    }
}

