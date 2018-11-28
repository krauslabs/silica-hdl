use std::iter::Peekable;
use std::str::Chars;

use token::{self, Token};

// pub type Location = usize;
// pub type LexerItem = (Location, Token, Location);

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

	fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_letter() {
            ident.push(self.read_char().unwrap()); // TODO: unwrap()
        }

        ident
    }

    fn read_number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);

        while let Some(&ch) = self.peek_char() {
            if !self.is_number(ch) {
                break;
            }
            number.push(self.read_char().unwrap()); // TODO: unwrap()
        }

        number
    }

    pub fn next_token(&mut self) -> Option<Token> {

    	self.skip_whitespace();

    	match self.read_char() {
    		Some(',') => Some(Token::Comma),
    		Some(';') => Some(Token::Semicolon),
    		Some(':') => Some(Token::Colon),
    		Some('(') => Some(Token::LeftParen),
    		Some(')') => Some(Token::RightParen),
    		Some('{') => Some(Token::LeftCurlyBrace),
    		Some('}') => Some(Token::RightCurlyBrace),
    		Some('=') => Some(Token::Assign),
    		Some('~') => Some(Token::Negate),
    		Some('&') => Some(Token::BitAnd),
    		Some('|') => Some(Token::BitOr),
    		Some('^') => Some(Token::BitXor),
    		Some('<') => { 
    			if self.peek_char_eq('<') {
    				self.read_char();
    				Some(Token::ShiftLeft)
    			} else {
    				panic!();
    			}
    		}
    		Some('>') => { 
    			if self.peek_char_eq('>') {
    				self.read_char();
    				Some(Token::ShiftRight)
    			} else {
    				panic!();
    			}
    		}
    		Some('/') => {
    			if self.peek_char_eq('/') {
    				self.skip_line();
    				self.next_token()
    			} else {
    				None
    			}
    		}
    		Some(ch @ _) => {
    			if self.is_letter(ch) {
    				let ident = self.read_identifier(ch);
    				Some(token::lookup_ident(&ident))
    			} else if self.is_number(ch) {
    				let num = self.read_number(ch);
    				Some(Token::Litrl(num))
    			} else {
    				None
    			}
    		}
    		None => None,
    	}
    }
}

