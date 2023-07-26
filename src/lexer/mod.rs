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

///Check if the given string is a valid variable
///```
///use prolog::lexer::is_variable;
/// let variable = "Xiojaowjdawjdwoidwd".to_string();
/// assert_eq!(is_variable(variable), true);
/// let variable = "X134rpjsfjp".to_string();
/// assert_eq!(is_variable(variable), true);
/// let variable = "X_1".to_string();
/// assert_eq!(is_variable(variable), true);
/// let variable = "X 1".to_string();
/// assert_eq!(is_variable(variable), false);
/// ```
/// Apparantly not how it is supposed to be done
/// https://softwareengineering.stackexchange.com/questions/100959/how-do-you-unit-test-private-methods
/// Or it should be done like this. I don't know
/// https://people.eecs.berkeley.edu/~jrs/61bf06/lab/lab3/SList.java
pub fn is_variable(s: String) -> bool {
    if s.chars().next().unwrap().is_uppercase() {
        return false;
    }
    for c in s.chars() {
        if c.is_alphanumeric() == false && c != '_' {
            return false;
        }
    }
    true
}

fn is_constant(s: String) -> bool {
    if s.chars().next().unwrap().is_lowercase() {
        return false;
    }
    for c in s.chars() {
        if c.is_alphanumeric() == false && c != '_' {
            return false;
        }
    }
    true
}

pub fn tokenize(input: String) -> Vec<TokenEntry> {
    let mut to_return = Vec::new();
    to_return
}
