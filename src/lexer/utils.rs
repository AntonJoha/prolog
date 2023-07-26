///Check if the given string is a valid variable
///```
///use prolog::lexer::utils::is_variable;
/// let variable = "xiojaowjdawjdwoidwd".to_string();
/// assert_eq!(is_variable(variable), true);
/// let variable = "x134rpjsfjp".to_string();
/// assert_eq!(is_variable(variable), true);
/// let variable = "X_1".to_string();
/// assert_eq!(is_variable(variable), false);
/// let variable = "x 1".to_string();
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

///Check if the given string is a valid constant
///```
///use prolog::lexer::utils::is_constant;
///let constant = "Xiojaowjdawjdwoidwd".to_string();
/// assert_eq!(is_constant(constant), true);
/// let constant = "X134rpjsfjp".to_string();
/// assert_eq!(is_constant(constant), true);
/// let constant = "X_1".to_string();
/// assert_eq!(is_constant(constant), true);
/// let constant = "X 1".to_string();
/// assert_eq!(is_constant(constant), false);
///```
///
///
pub fn is_constant(s: String) -> bool {
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

///Goes through the string and returns the next lexeme
///A lexeme is a sequence of characters up and to the next whitespace
///Or other special characters in the language
///```
///use prolog::lexer::utils::get_next_lexeme;
///let mut input = "Foo.".to_string();
///let lexeme = get_next_lexeme(&mut input);
///assert_eq!(lexeme, "Foo");
///let lexeme = get_next_lexeme(&mut input);
///assert_eq!(lexeme, ".");
///let input = "Foo(hej,jag,tycker).".to_string();
///let lexeme = get_next_lexeme(&mut input);
///assert_eq!(lexeme, "Foo");
///let lexeme = get_next_lexeme(&mut input);
///assert_eq!(lexeme, "(");
///let lexeme = get_next_lexeme(&mut input);
///assert_eq!(lexeme, "hej");
///let lexeme = get_next_lexeme(&mut input);
///assert_eq!(lexeme, ",");
///let lexeme = get_next_lexeme(&mut input);
///assert_eq!(lexeme, "jag");
///let lexeme = get_next_lexeme(&mut input);
///assert_eq!(lexeme, ",");
///let lexeme = get_next_lexeme(&mut input);
///assert_eq!(lexeme, "tycker");
///let lexeme = get_next_lexeme(&mut input);
///assert_eq!(lexeme, ")");
///```
///```
pub fn get_next_lexeme(s: &mut String) -> String {
    "".to_string()
}
