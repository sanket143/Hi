use std::io;
use std::io::Write;

use crate::lexer::types::TokenType;
use crate::lexer::def;

pub fn start() {
    const PROMPT: &'static str = ">> ";

    greet();

    loop {
        print!("{}", PROMPT);

        let _ = io::stdout().flush();

        let mut input = String::new();

        io::stdin()
          .read_line(&mut input)
          .expect("Failed to read line");


        let mut lex = def::new_lexer(&input);
        loop {
            let tok = lex.next();
            match tok.ttype {
                TokenType::EOF => break,
                _ => {
                    println!("{:?}", tok);
                }
            }
        }
    }
}

fn greet() {
    println!("Hello there! This is the Hi Repl");
}
