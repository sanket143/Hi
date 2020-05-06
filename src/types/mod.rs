pub mod helpers;

use std::str;
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
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

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
        
        let tok = str::from_utf8(&[self.ch]).unwrap().to_string();

        let token = match self.ch {
            b'=' => new_token(
              TokenType::ASSIGN,
              tok),
            b';' => new_token(
              TokenType::SEMICOLON,
              tok),
            b'(' => new_token(
              TokenType::LPAREN,
              tok),
            b')' => new_token(
              TokenType::RPAREN,
              tok),
            b',' => new_token(
              TokenType::COMMA,
              tok),
            b'+' => new_token(
              TokenType::PLUS,
              tok),
            b'{' => new_token(
              TokenType::LBRACE,
              tok),
            b'}' => new_token(
              TokenType::RBRACE,
              tok),
            0    => new_token(
              TokenType::EOF,
              String::from("")),
            _    => {
                if helpers::is_letter(self.ch) {
                    let ident = self.read_identifier();
                    return new_token(helpers::lookup_ident(&ident), ident);

                } else if helpers::is_digit(self.ch) {
                    let number = self.read_number();
                    return new_token(TokenType::INT, number);

                }

                return new_token(TokenType::ILLEGAL, self.ch.to_string());
            }
        };

        self.read_char();

        return token;
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

