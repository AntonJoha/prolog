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
}

pub struct Lexer {
    pub tokens: Vec<Token>,
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
pub fn is_variable(s: String) -> bool {
    if s.chars().next().unwrap().is_uppercase() == false {
        return false;
    }
    for c in s.chars() {
        if c.is_alphanumeric() == false && c != '_' {
            return false;
        }
    }
    true
}
