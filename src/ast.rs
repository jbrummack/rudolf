use crate::types::{Lexer, Token};

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.input.next() {
            Some(c) => {
                //print!("{c}");
                match c {
                    ',' => Token::Comma,
                    ':' => Token::Colon,
                    ';' => Token::Semicolon,
                    '=' => Token::Equals,
                    '(' => self.parse_list_expr(),
                    ')' => Token::ParenClose,
                    '[' => self.parse_bracket_expr(),
                    ']' => Token::SquareClose,
                    '{' => self.parse_curly_expr(),
                    '}' => Token::CurlyClose,
                    '<' => Token::Lt,
                    '>' => Token::Gt,
                    '.' => Token::Dot,
                    '"' => self.read_string(),
                    '/' => self.skip_comment(),
                    c if c.is_alphabetic() => self.read_identifier(c),
                    c if c.is_numeric() => self.read_number(c),
                    _ => panic!("Unexpected character: {}", c),
                }
            }
            None => Token::EOF,
        }
    }

    fn parse_list_expr(&mut self) -> Token {
        let mut paren = Token::Parentheses(Vec::new());
        loop {
            match self.next_token() {
                Token::ParenClose => {
                    return paren;
                }
                Token::EOF => {
                    return Token::EOF;
                }
                add => {
                    if let Token::Parentheses(inner) = &mut paren {
                        inner.push(add);
                    }
                }
            }
        }
    }
    fn parse_bracket_expr(&mut self) -> Token {
        let mut paren = Token::Bracket(Vec::new());
        loop {
            match self.next_token() {
                Token::SquareClose => {
                    return paren;
                }
                Token::EOF => {
                    return Token::EOF;
                }
                add => {
                    if let Token::Bracket(inner) = &mut paren {
                        inner.push(add);
                    }
                }
            }
        }
    }
    fn parse_curly_expr(&mut self) -> Token {
        let mut paren = Token::Curly(Vec::new());
        loop {
            match self.next_token() {
                Token::CurlyClose => {
                    return paren;
                }
                Token::EOF => {
                    return Token::EOF;
                }
                add => {
                    if let Token::Curly(inner) = &mut paren {
                        inner.push(add);
                    }
                }
            }
        }
    }

    fn skip_comment(&mut self) -> Token {
        let mut prev = '/';
        let mut singleline = false;
        let mut multiline = 0;
        let mut comment_buffer = "/".to_owned();
        while let Some(&c) = self.input.peek() {
            //println!("{},{},{},{}", prev, c, multiline, singleline);
            match (prev, c, multiline, singleline) {
                ('/', '/', 0, _) => singleline = true,
                (_, '\n', 0, true) => break,
                ('/', '*', _, false) => multiline += 1,
                ('*', '/', _, _) => multiline -= 1,
                ('/', _, 0, false) => break,
                (_, '*', 1, _) => (),
                _ => (),
            }
            comment_buffer.push(c);
            prev = c;
            self.input.next();
        }
        //
        if comment_buffer == "/".to_owned() {
            Token::Slash
        } else {
            //println!("COMMENT: {}", comment_buffer);
            Token::Comment
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.input.next();
        }
    }

    fn read_identifier(&mut self, first: char) -> Token {
        let mut ident = String::new();
        ident.push(first);

        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.input.next();
            } else {
                break;
            }
        }

        Token::Identifier(ident)
    }

    fn read_number(&mut self, first: char) -> Token {
        let mut number = String::new();
        number.push(first);
        while let Some(&c) = self.input.peek() {
            //println!("{number}");
            if c.is_numeric() || c == '.' {
                number.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        let mut pts = 0;
        for char in number.chars() {
            if char == '.' {
                pts += 1;
            }
        }
        match pts {
            0 => Token::Number(number.parse().unwrap()),
            1 => Token::Double(number.parse().unwrap()),
            _ => panic!("Cant parse numeral: {}", number),
        }
    }

    fn read_string(&mut self) -> Token {
        let mut string = String::new();

        while let Some(c) = self.input.next() {
            if c == '"' {
                break;
            }
            string.push(c);
        }

        Token::String(string)
    }
}
