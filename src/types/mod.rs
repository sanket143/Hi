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
    pub position: usize,
    pub read_position: usize,
    pub ch: u8
}

impl Lexer <'_> {
    fn next(&self) {
        println!("{}", self.input.len());
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}

