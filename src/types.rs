use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    _ParenOpen,
    ParenClose,
    _SquareOpen,
    SquareClose,
    _CurlyOpen,
    CurlyClose,
    Semicolon,
    Identifier(String),
    Colon,
    Comma,
    Slash,
    Comment,
    Equals,
    Gt,
    Lt,
    Dot,
    Number(i64),
    Double(f64),
    String(String),
    Parentheses(Vec<Token>),
    Bracket(Vec<Token>),
    Curly(Vec<Token>),
    EOF,
}

pub struct Lexer<'a> {
    pub input: Peekable<Chars<'a>>,
}
