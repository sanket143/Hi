pub mod def;

pub fn sanket(){
    let s = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut lex = def::new_lexer(&s);

    lex.read_char();
    println!("Sanket");
}
