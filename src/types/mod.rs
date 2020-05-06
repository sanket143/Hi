pub mod helpers;

use crate::types;

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

pub struct Token {
    pub ttype: TokenType,
    pub literal: String 
}

pub fn new_token(ttype: TokenType, literal: String) -> Token {
    types::Token {
        ttype,
        literal
    }
}

impl Lexer <'_> {
    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        match self.ch {
            b'=' => new_token(
              TokenType::ASSIGN,
              self.ch.to_string()),
            b';' => new_token(
              TokenType::SEMICOLON,
              self.ch.to_string()),
            b'(' => new_token(
              TokenType::LPAREN,
              self.ch.to_string()),
            b')' => new_token(
              TokenType::RPAREN,
              self.ch.to_string()),
            b',' => new_token(
              TokenType::COMMA,
              self.ch.to_string()),
            b'+' => new_token(
              TokenType::PLUS,
              self.ch.to_string()),
            b'{' => new_token(
              TokenType::LBRACE,
              self.ch.to_string()),
            b'}' => new_token(
              TokenType::RBRACE,
              self.ch.to_string()),
            0    => new_token(
              TokenType::EOF,
              String::from("")),
            _    => {
                if helpers::is_letter(self.ch) {
                    let ident = self.read_identifier();
                    return new_token(helpers::lookup_ident(&ident), ident);
                }

                return new_token(TokenType::ILLEGAL, self.ch.to_string());
            }
        }
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

