#![allow(dead_code)]
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
struct Token {
    ttype: TokenType,
    word: String,
    literal: Option<Object>,
    line: usize,
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    next_char: Option<char>,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::with_capacity(2048),
            start: 0,
            current: 0,
            line: 1,
            next_char: None,
        }
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Object>) {
        let word_bytes = self.source.as_bytes();
        let word = String::from_utf8(word_bytes[self.start..self.current].to_vec()).unwrap();
        self.tokens.push(Token {
            ttype,
            word,
            line: self.line,
            literal,
        })
    }

    fn index_source(&self, range: Range<usize>) -> String {
        String::from_utf8(self.source.as_bytes()[range].to_vec()).unwrap()
    }

    fn get_token_from_source(&self) -> String {
        String::from_utf8(self.source.as_bytes()[self.start..self.current].to_vec()).unwrap()
    }

    fn scan_ident(&mut self, chars: &mut Chars<'_>) {
        while let Some(c) = chars.next() {
            if c.is_alphanumeric() {
                self.current += 1;
            } else {
                self.add_token(TokenType::Identifer, None);
                self.next_char = Some(c);
                return;
            }
        }
        self.add_token(TokenType::Identifer, None);
        self.next_char = None;
    }

    fn scan_decimal(&mut self, chars: &mut Chars<'_>) {
        let mut has_period = false;
        while let Some(c) = chars.next() {
            if c.is_digit(10) {
                self.current += 1;
            } else if c == '.' && !has_period {
                has_period = true;
                self.current += 1;
            } else if c == '.' && has_period {
                panic!("Syntax error");
            } else {
                self.add_token(TokenType::Decimal, None);
                self.next_char = Some(c);
                return;
            }
        }
        if has_period {
            self.add_token(
                TokenType::Decimal,
                Some(Object::Floating(
                    self.get_token_from_source().parse::<f64>().unwrap(),
                )),
            );
        } else {
            self.add_token(TokenType::Decimal, None);
        }
        self.next_char = None;
    }

    fn scan_tokens(&mut self) {
        let source = self.source.clone();
        let mut chars = source.chars();
        self.next_char = chars.next();
        while let Some(c) = self.next_char {
            self.current += 1;
            match c {
                '=' => self.add_token(TokenType::Equal, None),
                '\n' => self.line += 1,
                ' ' | '\t' | '\r' => {}
                ':' => self.add_token(TokenType::Colon, None),
                c if c.is_digit(10) || c == '.' => {
                    self.scan_decimal(&mut chars);
                    self.start = self.current;
                    continue;
                }
                c if c.is_alphabetic() => {
                    self.scan_ident(&mut chars);
                    self.start = self.current;
                    continue;
                }
                _ => {}
            }
            self.start += 1;
            self.next_char = chars.next();
        }
        self.add_token(TokenType::EOF, None);
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn scan_identifier() {
    let source = String::from("myvar = .12");
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();
    dbg!(scanner.tokens);
    panic!();
}
