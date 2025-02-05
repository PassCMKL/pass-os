/// This module reads characters in arithmetic expression and converts them to tokens.
/// The allowed tokens are defined in ast module.
// Standard lib
use std::iter::Peekable;
use std::str::Chars;

//Other internal modules
use super::token::Token;

// Other structs

// Tokenizer struct contains a Peekable iterator on the arithmetic expression
pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

// Constructs a new instance of Tokenizer
impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }
}

// Implement Iterator trait for Tokenizer struct.
// With this, we can use next() method on tokenizer to retrieve the next token from arithmetic expression

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();

        match next_char {
            Some('0'..='9') => {
                // TODO: Iterate & peeking through the next characters to create Num token
                // Make sure to return None if the value is not parsable
                None
            },

            // TODO: return the appropriate tokens for available symbols

            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer() {
        let mut tokenizer = Tokenizer::new("34");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.0))
    }
    #[test]
    fn test_decimal_number() {
        let mut tokenizer = Tokenizer::new("34.5");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.5))
    }
    #[test]
    fn test_invalid_char() {
        let mut tokenizer = Tokenizer::new("#$%");
        assert_eq!(tokenizer.next(), None);
    }
}
