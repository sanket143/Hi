use crate::lexer::types;

pub fn new_lexer<'a> (input: &'a String) -> types::Lexer {
    let input = input.as_bytes();
    types::Lexer {
        input,
        position: 0,
        read_position: 1,
        ch: input[0]
    }
}

