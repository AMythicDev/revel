#![allow(dead_code)]
use peekmore::{PeekMore, PeekMoreIterator};
use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    LeftParem,
    RightParen,
    Identifer,
    Equal,
    Number,
    EOF,
    Colon,
    String,
    Comma,
    Dot,
    Plus,
    Minus,
    Semicolon,
    Slash,
    Star,
}

#[derive(Debug, PartialEq, Eq)]
struct Token<'a> {
    ttype: TokenType,
    start: usize,
    end: usize,
    lexeme: &'a str,
    line: i32,
}

type TokenResult<'a> = Result<Token<'a>, String>;

struct Lexer<'a> {
    source: &'a String,
    chars: PeekMoreIterator<Chars<'a>>,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a String) -> Self {
        Self {
            source,
            chars: source.chars().peekmore(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn token(&mut self, ttype: TokenType) -> TokenResult<'a> {
        Ok(Token {
            ttype,
            line: self.line,
            start: self.start,
            end: self.current,
            lexeme: &self.source[self.start..self.current],
        })
    }

    fn scan_ident(&mut self) -> TokenResult<'a> {
        while let Some(c) = self.chars.peek() {
            if c.is_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }
        self.token(TokenType::Identifer)
    }

    fn scan_number(&mut self) -> TokenResult<'a> {
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
        self.token(TokenType::Number)
    }

    fn scan_token(&mut self) -> TokenResult<'a> {
        self.skip_whitespaces();
        self.start = self.current;
        match self.advance() {
            Some('=') => self.token(TokenType::Equal),
            Some(':') => self.token(TokenType::Colon),
            Some(c) if c.is_digit(10) || c == '.' => self.scan_number(),
            Some(c) if c.is_alphabetic() => self.scan_ident(),
            Some('"') => self.scan_string(),
            None => panic!("No token found"),
            _ => panic!("Invalid token"),
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

    fn scan_string(&mut self) -> TokenResult<'a> {
        self.start += 1;
        while let Some(c) = self.chars.peek() {
            if *c == '"' {
                break;
            };
            self.advance();
        }
        if self.chars.peek() == None {
            panic!("Unterminated string found");
        } else {
            let ret = self.token(TokenType::String);
            self.advance();
            ret
        }
    }
}

fn main() {}

#[test]
fn single_line_stmt() {
    let source = String::from("foo: double = .12");
    let mut scanner = Lexer::new(&source);

    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Identifer,
            start: 0,
            end: 3,
            lexeme: "foo",
            line: 1
        }
    );
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Colon,
            start: 3,
            end: 4,
            lexeme: ":",
            line: 1
        }
    );
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Identifer,
            start: 5,
            end: 11,
            lexeme: "double",
            line: 1
        }
    );
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Equal,
            start: 12,
            end: 13,
            lexeme: "=",
            line: 1
        }
    );
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Number,
            start: 14,
            end: 17,
            lexeme: ".12",
            line: 1
        }
    );
}

#[test]
fn ignore_initial_line_breaks() {
    let source = String::from("\n\nbar: int = 100");
    let mut scanner = Lexer::new(&source);
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Identifer,
            start: 2,
            end: 5,
            lexeme: "bar",
            line: 3
        }
    );
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Colon,
            start: 5,
            end: 6,
            lexeme: ":",
            line: 3
        }
    );
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Identifer,
            start: 7,
            end: 10,
            lexeme: "int",
            line: 3
        }
    );
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Equal,
            start: 11,
            end: 12,
            lexeme: "=",
            line: 3
        }
    );
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Number,
            start: 13,
            end: 16,
            lexeme: "100",
            line: 3
        }
    );
}

#[test]
fn scan_string() {
    let source = String::from("bar: string = \"hello world\"");
    let mut scanner = Lexer::new(&source);
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Identifer,
            start: 0,
            end: 3,
            lexeme: "bar",
            line: 1
        }
    );
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Colon,
            start: 3,
            end: 4,
            lexeme: ":",
            line: 1
        }
    );
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Identifer,
            start: 5,
            end: 11,
            lexeme: "string",
            line: 1
        }
    );
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::Equal,
            start: 12,
            end: 13,
            lexeme: "=",
            line: 1
        }
    );
    assert_eq!(
        scanner.scan_token().unwrap(),
        Token {
            ttype: TokenType::String,
            start: 15,
            end: 26,
            lexeme: "hello world",
            line: 1
        }
    );
}
