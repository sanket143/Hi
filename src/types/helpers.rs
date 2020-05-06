use crate::types::TokenType;

pub fn is_letter(ch: u8) -> bool {
    b'a' <= ch &&
      ch <= b'z' ||
      b'A' <= ch &&
      ch <= b'Z' ||
      ch == b'_'
}

pub fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}

pub fn lookup_ident(identifier: &str) -> TokenType {
    match identifier {
        "fn"  => TokenType::FUNCTION,
        "let" => TokenType::LET,
        _     => TokenType::IDENT
    }
}
