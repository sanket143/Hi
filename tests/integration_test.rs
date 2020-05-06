use hi::types::TokenType;
use hi::lexer::def;

#[test]
fn test_next_token() {
    let s = String::from("=+(){};");
    let mut lex = def::new_lexer(&s);

    let tests = [
      (TokenType::ASSIGN,    b'='),
      (TokenType::PLUS,      b'+'),
      (TokenType::LPAREN,    b'('),
      (TokenType::RPAREN,    b')'),
      (TokenType::LBRACE,    b'{'),
      (TokenType::RBRACE,    b'}'),
      (TokenType::SEMICOLON, b';'),
    ];

    for test in tests.iter() {
        let tok = lex.next();
        let ttype = format!("{:?}", tok.ttype);
        let literal = format!("{:?}", tok.literal);
        let test0 = format!("{:?}", test.0);
        let test1 = format!("{:?}", test.1);

        if ttype != test0 {
            panic!("tokentype wrong. expected={:?}, got={:?}",
              test0, ttype);
        }

        if literal != test1 {
            panic!("literal wrong. expected={:?}, got={:?}",
              test1, literal);
        }
    }
}
