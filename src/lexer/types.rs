use std::str;
use crate::lexer::helpers;

#[derive(Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + Literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,
    EQ,
    NOTEQ,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN
}

pub struct Lexer <'a> {
    pub input: &'a [u8],
    pub position: usize,
    pub read_position: usize,
    pub ch: u8
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String 
}

pub fn new_token(ttype: TokenType, literal: String) -> Token {
    Token {
        ttype,
        literal
    }
}

impl Lexer <'_> {
    pub fn next(&mut self) -> Token {
        self.skip_whitespace();
        
        let tok = str::from_utf8(&[self.ch]).unwrap().to_string();

        let token = match self.ch {
            b'=' => {
                match self.peek() {
                  b'=' => {
                    self.read_char();
                    new_token(TokenType::EQ, String::from("=="))
                  },
                  _    => new_token(TokenType::ASSIGN, tok)
                }
            },
            b'+' => new_token(TokenType::PLUS, tok),
            b'-' => new_token(TokenType::MINUS, tok),
            b'!' => {
                match self.peek() {
                  b'=' => {
                    self.read_char();
                    new_token(TokenType::NOTEQ, String::from("!="))
                  },
                  _    => new_token(TokenType::BANG, tok)
                }
            },
            b'/' => new_token(TokenType::SLASH, tok),
            b'*' => new_token(TokenType::ASTERISK, tok),
            b'<' => new_token(TokenType::LT, tok),
            b'>' => new_token(TokenType::GT, tok),
            b';' => new_token(TokenType::SEMICOLON, tok),
            b'(' => new_token(TokenType::LPAREN, tok),
            b')' => new_token(TokenType::RPAREN, tok),
            b',' => new_token(TokenType::COMMA, tok),
            b'{' => new_token(TokenType::LBRACE, tok),
            b'}' => new_token(TokenType::RBRACE, tok),
            0    => new_token(TokenType::EOF, String::from("")),
            _    => {
                if helpers::is_letter(self.ch) {
                    let ident = self.read_identifier();
                    return new_token(helpers::lookup_ident(&ident), ident);

                } else if helpers::is_digit(self.ch) {
                    let number = self.read_number();
                    return new_token(TokenType::INT, number);

                }

                self.read_char();
                return new_token(TokenType::ILLEGAL, tok);
            }
        };

        self.read_char();

        return token;
    }

    pub fn peek(&mut self) -> u8 {
        self.input[self.read_position]
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

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;

        loop {
            if !helpers::is_letter(self.ch) {
                break std::str::from_utf8(&self.input[position..self.position])
                  .unwrap()
                  .to_string();
            }

            self.read_char();
        }
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;

        loop {
            if !helpers::is_digit(self.ch) {
                break std::str::from_utf8(&self.input[position..self.position])
                  .unwrap()
                  .to_string();
            }

            self.read_char();
        }
    }

    pub fn skip_whitespace(&mut self) {
        loop {
          if self.ch != b' '
            && self.ch != b'\t'
            && self.ch != b'\n'
            && self.ch != b'\r' {
              break;
          }

          self.read_char();
        }
    }
}

