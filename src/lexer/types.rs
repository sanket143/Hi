pub struct Lexer {
    pub input: String,
    position: u128,
    read_position: u128,
    ch: char
}

pub fn newLexer(input: String) -> Lexer {
    Lexer {
        input,
        position: 0,
        read_position: 0,
        ch: 'a'
    }
}
