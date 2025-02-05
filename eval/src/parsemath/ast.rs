/// This program contains list of valid AST nodes that can be constructed and also evaluates an AST to compute a value
// Standard lib
use std::error;

//structs

// List of allowed AST nodes that can be constructed by Parser
// Tokens can be arithmetic operators or a Number
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    // WARNING: Bitwise And and Or operation only works on integer value
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),

    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(f64),
}

// Given an AST, calculate the numeric value.
pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),

        // TODO: complete the match expression to evaluate the numeric value
        _ => panic!()
    }
}

//Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expr1() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("1+2-3").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 0.0);
    }
    #[test]
    fn test_expr2() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("3+2-1*5/4").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 3.75);
    }
    #[test]
    fn test_expr3() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("3+3 | 4").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 6.0);
    }
}
