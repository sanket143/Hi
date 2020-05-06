use hi::types::TokenType;
use hi::lexer::def;

#[test]
fn test_next_token() {
    let s = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
";
    let s = String::from(s);
    let mut lex = def::new_lexer(&s);

    let tests = [
      (TokenType::LET,       "let"),
      (TokenType::IDENT,     "five"),
      (TokenType::ASSIGN,    "="),
      (TokenType::INT,       "5"),
      (TokenType::SEMICOLON, ";"),
      (TokenType::LET,       "let"),
      (TokenType::IDENT,     "ten"),
      (TokenType::ASSIGN,    "="),
      (TokenType::INT,       "10"),
      (TokenType::SEMICOLON, ";"),
      (TokenType::LET,       "let"),
      (TokenType::IDENT,     "add"),
      (TokenType::ASSIGN,    "="),
      (TokenType::FUNCTION,  "fn"),
      (TokenType::LPAREN,    "("),
      (TokenType::IDENT,     "x"),
      (TokenType::COMMA,     ","),
      (TokenType::IDENT,     "y"),
      (TokenType::RPAREN,    ")"),
      (TokenType::LBRACE,    "{"),
      (TokenType::IDENT,     "x"),
      (TokenType::PLUS,      "+"),
      (TokenType::IDENT,     "y"),
      (TokenType::SEMICOLON, ";"),
      (TokenType::RPAREN,    "}"),
      (TokenType::SEMICOLON, ";"),
      (TokenType::IDENT,     "result"),
      (TokenType::ASSIGN,    "="),
      (TokenType::IDENT,     "add"),
      (TokenType::LPAREN,    "("),
      (TokenType::IDENT,     "five"),
      (TokenType::COMMA,     ","),
      (TokenType::IDENT,     "ten"),
      (TokenType::RPAREN,    ")"),
      (TokenType::SEMICOLON, ";"),
      (TokenType::EOF,       ""),
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
