#![allow(dead_code)]
use peekmore::{PeekMore, PeekMoreIterator};
use std::{ops::Range, str::Chars};

#[derive(Debug)]
enum TokenType {
    LeftParem,
    RightParen,
    Identifer,
    Equal,
    Number,
    EOF,
    Decimal,
    Colon,
    Comma,
    Dot,
    Plus,
    Minus,
    Semicolon,
    Slash,
    Star,
}

#[derive(Debug)]
enum Object {
    Integer(usize),
    Floating(f64),
}

#[derive(Debug)]
struct Token<'a> {
    start: usize,
    end: usize,
    lexeme: &'a str,
}

#[derive(Debug)]
struct TokenResult<'a> {
    pub line: i32,
    pub ttype: TokenType,
    pub inner: Result<Token<'a>, String>,
}

struct Scanner<'a> {
    source: &'a String,
    chars: PeekMoreIterator<Chars<'a>>,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a String) -> Self {
        Self {
            source,
            chars: source.chars().peekmore(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn add_token(&mut self, ttype: TokenType) -> TokenResult<'a> {
        TokenResult {
            line: self.line,
            ttype,
            inner: Ok(Token {
                start: self.start,
                end: self.current,
                lexeme: &self.source[self.start..self.current],
            }),
        }
    }

    fn index_source(&self, range: Range<usize>) -> String {
        String::from_utf8(self.source.as_bytes()[range].to_vec()).unwrap()
    }

    fn get_token_from_source(&self) -> String {
        String::from_utf8(self.source.as_bytes()[self.start..self.current].to_vec()).unwrap()
    }

    fn scan_ident(&mut self) {
        while let Some(c) = self.chars.next() {
            if c.is_alphanumeric() {
                self.current += 1;
            } else {
                self.add_token(TokenType::Identifer);
                return;
            }
        }
        self.add_token(TokenType::Identifer);
    }

    fn scan_decimal(&mut self) -> TokenResult<'a> {
        while let Some(c) = self.chars.peek() {
            if c.is_digit(10) {
                self.advance();
            }
        }
        if let Some(c) = self.chars.peek() {
            if *c == '.' {
                self.advance();
                while let Some(c) = self.chars.peek() {
                    if c.is_digit(10) {
                        self.advance();
                    }
                }
            }
        }
        self.add_token(TokenType::Decimal)
    }

    fn scan_tokens(&mut self) -> TokenResult<'a> {
        self.skip_whitespaces();
        self.start = self.current;
        match self.advance() {
            Some('=') => self.add_token(TokenType::Equal),
            Some(':') => self.add_token(TokenType::Colon),
            Some(c) if c.is_digit(10) || c == '.' => self.scan_decimal(),
            Some(c) if c.is_alphabetic() => self.scan_ident(),
            _ => Err("No Token found"),
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.chars.next()
    }

    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\r' => _ = self.advance(),
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => break,
            }
        }
    }
}

fn main() {}

#[test]
fn scan_identifier() {
    let source = String::from("myvar = .12");
    let mut scanner = Scanner::new(&source);
    scanner.scan_tokens();
    dbg!(scanner.tokens);
    panic!();
}
