use prolog::lexer::{tokenize, Token, TokenEntry};

fn verify_helper(input: String, expected: Vec<TokenEntry>) {
    let result = tokenize(input);
    assert_eq!(result, expected);
}

#[test]
fn parse_simple_fact() {
    let input = "Foo.".to_string();
    let tokens = tokenize(input);

    let expected = vec![
        TokenEntry {
            token: Token::Constant,
            value: "Foo".to_string(),
        },
        TokenEntry {
            token: Token::Period,
            value: ".".to_string(),
        },
    ];
}
