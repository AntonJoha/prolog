use utils::get_next_lexeme;
pub mod utils;
#[derive(Debug, PartialEq)]
pub enum Token {
    Statement,
    Fact,
    Rule,
    Query,
    Atoms,
    Atom,
    Terms,
    Term,
    Variable,
    Constant,
    Predicate,
    Period,
    Comma,
    LeftParen,
    RightParen,
    QueryMark,
    RuleMark,
    EndOfFile,
}

#[derive(Debug, PartialEq)]
pub struct TokenEntry {
    pub token: Token,
    pub value: String,
}

pub fn tokenize(mut input: String) -> Vec<TokenEntry> {
    let mut to_return = Vec::new();

    let lexeme: String = get_next_lexeme(&mut input);

    to_return
}
