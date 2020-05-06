enum TypeToken {
    ILLEGAL,
    EOF,

    // Identifiers + Literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET
}

pub struct Lexer <'a> {
    pub input: &'a [u8],
    pub position: u128,
    pub read_position: u128,
    pub ch: u8
}

impl Lexer <'_> {
    fn next(&self) {
        println!("{}", self.input.len());
    }

    pub fn read_char(&mut self) {
        println!("bleh");
    }
}

