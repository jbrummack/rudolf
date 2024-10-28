use std::{iter::Peekable, str::Chars};

#[macro_export]
macro_rules! idt {
    ($str:expr) => {
        Token::Identifier($str.to_string())
    };
}
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
    Percent,
    Backslash,
    Dollar,
    Euro,
    Circonflexe,
    Questionmark,
    Exclamationmark,
    And,
    Pipe,
    Minus,
    Plus,
    Hashtag,
    AccentAigu,
    AccentGrave,
    Degree,
    Parentheses(Vec<Token>),
    Bracket(Vec<Token>),
    Curly(Vec<Token>),
    EOF,
}

pub struct Lexer<'a> {
    pub input: Peekable<Chars<'a>>,
}
