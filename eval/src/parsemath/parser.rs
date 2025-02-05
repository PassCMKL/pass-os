/// This program reads tokens returned by Tokenizer and converts them into AST.
// Standard lib
use std::fmt;

// Internal modules
use super::ast::Node;
use super::token::{OperPrec, Token};
use super::tokenizer::Tokenizer;

//Structs and constants

// Parser struct
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

// Public methods of Parser

impl<'a> Parser<'a> {
    // Create a new instance of Parser
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }

    // Take an arithmetic expression as input and return an AST

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            // TODO: Replace this with proper handling of return value from generate_ast result
            _ => panic!()
        }
    }
}

// Private methods of Parser

impl<'a> Parser<'a> {
    // Retrieve the next token from arithmetic expression and set it to current_token field in Parser struct
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(())
    }

    // Main workhorse method that is called recursively

    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            }
            let right_expr = self.convert_token_to_node(left_expr.clone())?;
            left_expr = right_expr;
        }
        Ok(left_expr)
    }

    // Construct AST node for numbers, taking into account negative prefixes while handling parenthesis

    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            }
            Token::LeftParen => {
                self.get_next_token()?;

                // TODO: Replace the following code to check for matching parenthesis;
                // also convert (x)(y) to x times y
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                Ok(expr)
            }
            _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
        }
    }

    // Check for balancing parenthesis
    fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        if expected == self.current_token {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidOperator(format!(
                "Expected {:?}, got {:?}",
                expected, self.current_token
            )))
        }
    }

    // Construct Operator AST nodes
    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;
                //Get right-side expression
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            // TODO: Complete the node construction for other tokens

            _ => Err(ParseError::InvalidOperator(format!(
                "Please enter valid operator {:?}",
                self.current_token
            ))),
        }
    }
}

// Custom error handler for Parser
#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ParseError::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
            ParseError::InvalidOperator(e) => write!(f, "Error in evaluating {}", e),
        }
    }
}

// Handle error thrown from AST module

impl From<Box<dyn std::error::Error>> for ParseError {
    fn from(_evalerr: Box<dyn std::error::Error>) -> Self {
        ParseError::UnableToParse("Unable to parse".into())
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsemath::ast::Node::{Add, Or, Number};
    #[test]
    fn test_addition() {
        let mut parser = Parser::new("1+2").unwrap();
        let expected = Add(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn test_bitwise_or() {
        let mut parser = Parser::new("6|2").unwrap();
        let expected = Or(Box::new(Number(6.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
}
