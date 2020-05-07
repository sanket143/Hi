pub mod def;
pub mod types;
pub mod helpers;

pub fn sanket(){
    let s = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut lex = def::new_lexer(&s);

    lex.read_char();
    println!("Sanket");
}
