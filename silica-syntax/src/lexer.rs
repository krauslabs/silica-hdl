use std::mem;
use std::str::CharIndices;

use crate::BytePos;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // Identifiers
    Ident(String),

    // Literals
    Litrl(String),

    // Punctuation
    Comma,
    Semicolon,
    Colon,
    LeftParen,
    RightParen,
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftSquareBrace,
    RightSquareBrace,

    // Operators
    Assign,
    Negate,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,

    // Keywords
    Mod,
    Top,
    In,
    Out,
    Bit,
    Let,
}

#[derive(Debug, PartialEq)]
pub struct InvalidCharacterError {
    pub pos: BytePos,
    pub ch: char,
}

pub type LexerItem = Result<(BytePos, Token, BytePos), InvalidCharacterError>;

pub struct Lexer<'a> {
    chars: CharIndices<'a>,
    lookahead: Option<(usize, char)>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut chars = input.char_indices();
        let lookahead = chars.next();

        Lexer {
            chars: chars,
            lookahead: lookahead,
        }
    }

    fn read_char(&mut self) -> Option<(usize, char)> {
        mem::replace(&mut self.lookahead, self.chars.next())
    }

    fn peek_char(&mut self) -> Option<(usize, char)> {
        self.lookahead
    }

    fn peek_char_eq(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some((_, peek_ch)) => peek_ch == ch,
            None => false,
        }
    }

    fn is_letter(&mut self, ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn is_number(&mut self, ch: char) -> bool {
        ch.is_numeric()
    }

    fn read_identifier(&mut self, start: usize, first: char) -> (usize, Token, usize) {
        let mut ident = String::new();
        let mut end = start;
        ident.push(first);

        while let Some((_, ch)) = self.peek_char() {
            if self.is_letter(ch) || self.is_number(ch) {
                if let Some((i, ch)) = self.read_char() {
                    ident.push(ch);
                    end = i;
                }
            } else {
                break;
            }
        }

        let token = match ident.as_str() {
            "mod" => Token::Mod,
            "top" => Token::Top,
            "in" => Token::In,
            "out" => Token::Out,
            "bit" => Token::Bit,
            "let" => Token::Let,
            _ => Token::Ident(ident.to_string()),
        };

        (start, token, end + 1)
    }

    fn read_number(&mut self, start: usize, first: char) -> (usize, Token, usize) {
        let mut number = String::new();
        let mut end = start;
        number.push(first);

        while let Some((_, ch)) = self.peek_char() {
            if self.is_number(ch) {
                if let Some((i, ch)) = self.read_char() {
                    number.push(ch);
                    end = i;
                }
            } else {
                break;
            }
        }

        (start, Token::Litrl(number), end + 1)
    }

    fn skip_whitespace(&mut self) {
        while let Some((_, ch)) = self.peek_char() {
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

        if let Some((i, ch)) = self.read_char() {
            match ch {
                ',' => Some(Ok((i, Token::Comma, i + 1))),
                ';' => Some(Ok((i, Token::Semicolon, i + 1))),
                ':' => Some(Ok((i, Token::Colon, i + 1))),
                '(' => Some(Ok((i, Token::LeftParen, i + 1))),
                ')' => Some(Ok((i, Token::RightParen, i + 1))),
                '{' => Some(Ok((i, Token::LeftCurlyBrace, i + 1))),
                '}' => Some(Ok((i, Token::RightCurlyBrace, i + 1))),
                '[' => Some(Ok((i, Token::LeftSquareBrace, i + 1))),
                ']' => Some(Ok((i, Token::RightSquareBrace, i + 1))),
                '=' => Some(Ok((i, Token::Assign, i + 1))),
                '~' => Some(Ok((i, Token::Negate, i + 1))),
                '&' => Some(Ok((i, Token::BitAnd, i + 1))),
                '|' => Some(Ok((i, Token::BitOr, i + 1))),
                '^' => Some(Ok((i, Token::BitXor, i + 1))),
                '<' => {
                    if self.peek_char_eq('<') {
                        self.read_char();
                        Some(Ok((i, Token::ShiftLeft, i + 2)))
                    } else {
                        Some(Err(InvalidCharacterError { pos: i, ch: '<' }))
                    }
                }
                '>' => {
                    if self.peek_char_eq('>') {
                        self.read_char();
                        Some(Ok((i, Token::ShiftRight, i + 2)))
                    } else {
                        Some(Err(InvalidCharacterError { pos: i, ch: '>' }))
                    }
                }
                '/' => {
                    if self.peek_char_eq('/') {
                        self.skip_line();
                        self.next()
                    } else {
                        Some(Err(InvalidCharacterError { pos: i, ch: '/' }))
                    }
                }
                ch @ _ => {
                    if self.is_letter(ch) {
                        Some(Ok(self.read_identifier(i, ch)))
                    } else if self.is_number(ch) {
                        Some(Ok(self.read_number(i, ch)))
                    } else {
                        Some(Err(InvalidCharacterError { pos: i, ch: ch }))
                    }
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use crate::lexer::*;

    fn assert_lex(source: &str, expected: Vec<LexerItem>) {
        let lexer = Lexer::new(source);

        for (idx, item) in lexer.enumerate() {
            assert_eq!(item, expected[idx]);
        }
    }

    #[test]
    fn identifiers() {
        assert_lex(
            "id _id id123 modu mo",
            vec![
                Ok((0, Token::Ident("id".to_string()), 2)),
                Ok((3, Token::Ident("_id".to_string()), 6)),
                Ok((7, Token::Ident("id123".to_string()), 12)),
                Ok((13, Token::Ident("modu".to_string()), 17)),
                Ok((18, Token::Ident("mo".to_string()), 20)),
            ],
        );
    }

    #[test]
    fn literal() {
        assert_lex(
            "1 0123",
            vec![
                Ok((0, Token::Litrl("1".to_string()), 1)),
                Ok((2, Token::Litrl("0123".to_string()), 6)),
            ],
        );
    }

    #[test]
    fn comments() {
        assert_lex(
            "top // this shouldn't be seen \n mod",
            vec![Ok((0, Token::Top, 3)), Ok((32, Token::Mod, 35))],
        );
    }

    #[test]
    fn keywords() {
        assert_lex(
            "mod top in out bit let",
            vec![
                Ok((0, Token::Mod, 3)),
                Ok((4, Token::Top, 7)),
                Ok((8, Token::In, 10)),
                Ok((11, Token::Out, 14)),
                Ok((15, Token::Bit, 18)),
                Ok((19, Token::Let, 22)),
            ],
        );
    }

    #[test]
    fn operators() {
        assert_lex(
            "= ~ & | ^ << >>",
            vec![
                Ok((0, Token::Assign, 1)),
                Ok((2, Token::Negate, 3)),
                Ok((4, Token::BitAnd, 5)),
                Ok((6, Token::BitOr, 7)),
                Ok((8, Token::BitXor, 9)),
                Ok((10, Token::ShiftLeft, 12)),
                Ok((13, Token::ShiftRight, 15)),
            ],
        );
    }

    #[test]
    fn punctuation() {
        assert_lex(
            ", ; : ( ) { } [ ]",
            vec![
                Ok((0, Token::Comma, 1)),
                Ok((2, Token::Semicolon, 3)),
                Ok((4, Token::Colon, 5)),
                Ok((6, Token::LeftParen, 7)),
                Ok((8, Token::RightParen, 9)),
                Ok((10, Token::LeftCurlyBrace, 11)),
                Ok((12, Token::RightCurlyBrace, 13)),
                Ok((14, Token::LeftSquareBrace, 15)),
                Ok((16, Token::RightSquareBrace, 17)),
            ],
        );
    }

    #[test]
    fn error() {
        assert_lex(
            "= ∞ abc",
            vec![
                Ok((0, Token::Assign, 1)),
                Err(InvalidCharacterError { pos: 2, ch: '∞' }),
                Ok((6, Token::Ident("abc".to_string()), 9)),
            ],
        );
    }
}
