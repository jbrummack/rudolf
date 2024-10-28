use rudolf::idt;
use rudolf::types::Lexer;
use rudolf::types::Token;

fn main() {
    let input = r#"
        CREATE TABLE users IF NOT EXISTS (
            BIG TEXT name,
            //ORD INTEGER age,
        );
        CREATE TABLE users /*IF NOT EXISTS*/ (
            BIG TEXT name,
            ORD INTEGER age,
        );
        300 / 30.06;
        INSERT INTO users (name, age) VALUES
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
    let lex = Lexer::new(input);
    for token in lex {
        print!("{token:?} ");
    }
    println!("{:?}", idt!("CREATE"));
    /*loop {
        let nt = lex.next_token();
        print!("{nt:?} ");
        if nt == Token::EOF {
            break;
        }
    }*/
}
