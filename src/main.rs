use types::{Lexer, Token};

mod ast;
mod types;

fn main() {
    //println!("Hello, world!");
    let input = r#"
        CREATE TABLE users IF NOT EXISTS (
            BIG TEXT name,
            //ORD INTEGER age,
        );
        CREATE TABLE users /*IF NOT EXISTS*/ (
            BIG TEXT name,
            ORD INTEGER age,
        );

        INSERT INTO users (name, age) VALUES
            ("Julius Brummack", 20),
            ("Max Mustermann", 35)
        ;
        SELECT (users.name, users.age) FROM users WHERE (age = 20);
        (("1")("2")("3"))("hello");
        {
            "key":"value",
            "key2":["value1","value2"]
        }

        //Dont go into recursive loop of death on unclosed parenthese
        (
        "#;
    let mut lex = Lexer::new(input);
    loop {
        let nt = lex.next_token();
        print!("{nt:?} ");
        if nt == Token::EOF {
            break;
        }
    }
}