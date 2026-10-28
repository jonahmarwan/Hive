use crate::{ast::ASTNodeType::{Assignment, BinaryExpression, IdenDeclarator, Identifier, Literal}, token::{Node, Token::{self, EOF, IDENTIFIER, NUMBER}}};
use crate::ast::{ASTNode};
use crate::runtime::RuntimeValue;

pub struct Parser {
    pub tokens: Vec<Node>,
    token_index: usize,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            tokens: Vec::new(),
            token_index: 0,
        }
    }
}

impl Parser {
    pub fn new(t: Vec<Node>) -> Self {
        Self {
            tokens: t,
            token_index: 0,
        }
    }

    pub fn current_token(&self) -> Node {
        self.tokens
            .get(self.token_index)
            .cloned()
            .unwrap_or_else(|| Node { _type: EOF, value: RuntimeValue::String(String::from("None")) })
    }  

    fn advance(&mut self) -> Node {
        self.token_index += 1;
        self.current_token()
    }

    fn get_number_node(&self) -> Result<ASTNode, String> {
        if matches!(self.current_token()._type, Token::NUMBER) {
            return Result::Ok(ASTNode { _type: Literal, value: self.current_token(), left_child: None, right_child: None});
        }
        Err(String::from("Number expected"))
    }

    fn get_operator_node(&self) -> Result<ASTNode, String> {
        if matches!(self.current_token()._type, Token::PLUS | Token::MINUS) {
            return Result::Ok(ASTNode { _type: Literal, value: self.current_token(), left_child: None, right_child: None});
        }
        Err(String::from("Operator expected"))
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut left_node = self.get_number_node()?;
        self.advance(); // Move past the number

        while matches!(self.current_token()._type, Token::MUL | Token::DIV) {
            let operator = ASTNode {
                _type: Literal,
                value: self.current_token(),
                left_child: None,
                right_child: None
            };

            self.advance(); // Move past the operator
            
            let right_node = self.get_number_node()?;
            self.advance(); // Move past the right number

            left_node = ASTNode { 
                _type: BinaryExpression, 
                value: operator.value, 
                left_child: Some(Box::new(left_node)), 
                right_child: Some(Box::new(right_node))
            }
        }
        Result::Ok(left_node)
    }

    pub fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let mut left_node = self.parse_term()?;
        // REMOVED: self.advance() here was breaking the position tracking

        while let Ok(operator) = self.get_operator_node() {
            self.advance(); // Move past the operator

            let right_node = self.parse_term()?; // parse_term handles its own advancement

            left_node = ASTNode { 
                _type: BinaryExpression, 
                value: operator.value,
                left_child: Some(Box::new(left_node)),
                right_child: Some(Box::new(right_node)) 
            }
        }
        Result::Ok(left_node)
    }

    pub fn parse_statement(&mut self) -> Result<ASTNode, String> {
        if matches!(self.current_token()._type, Token::NUMBER) {
            return self.parse_expression();
        } else if matches!(self.current_token()._type, Token::KEYWORD) {
            let mut left_node: ASTNode = ASTNode{
                _type: IdenDeclarator,
                value: self.current_token(),
                left_child: None,
                right_child: None
            };
            if let RuntimeValue::String(s) = &self.current_token().value {
                match s.as_str() {
                    "def" => {
                        self.advance();
                        if matches!(self.current_token()._type, Token::IDENTIFIER) {
                            let variable = ASTNode {
                                _type: Identifier,
                                value: self.current_token(),
                                left_child: None,
                                right_child: None
                            };

                            self.advance();

                            if matches!(self.current_token()._type, Token::ASSIGNMENT) {
                                let mut operator = ASTNode {
                                    _type: Assignment,
                                    value: self.current_token(),
                                    left_child: Some(Box::new(variable)),
                                    right_child: None
                                };

                                self.advance();

                                if matches!(self.current_token()._type, Token::NUMBER) {
                                    let rhs = self.parse_expression().expect("Invalid Assignment");
                                    operator.right_child = Some(Box::new(rhs));
                                    left_node.left_child = Some(Box::new(operator));
                                }
                            }
                        }      
                    },
                    _ => {}
                }
            }
            return Result::Ok(left_node)
        }
        Result::Err(String::from("Invalid keyword"))
    }
}
