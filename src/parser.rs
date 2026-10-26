use crate::{ast::ASTNodeType::BinaryExpression, token::{Node, Token::{self, EOF}}};
use crate::ast::{ASTNode};
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
    pub fn current_token(&mut self) -> Node {
        self.tokens
            .get(self.token_index)
            .cloned()
            .unwrap_or_else(|| Node { _type: EOF, value: String::from("None") })
    }  
    fn advance(&mut self) -> Node {
        self.token_index += 1;
        self.current_token()
    }
    fn get_number_node(&mut self) -> Result<Node, String> {
        if matches!(self.current_token()._type, Token::NUMBER) {
            return Result::Ok(self.current_token().clone());
        }
        Err(String::from("Number expected"))
    }
    fn get_operator_node(&mut self) -> Result<Node, String> {
        if matches!(self.current_token()._type, Token::PLUS) {
           return Result::Ok(self.current_token().clone()) 
        }
        Err(String::from("Operator expected"))
    }
    pub fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let left_node = self.get_number_node()?;
        self.advance();

        let operator = self.get_operator_node()?;
        self.advance();

        let right_node = self.get_number_node()?;
        self.advance();

        let branch = ASTNode{_type: BinaryExpression, value: operator, left_child: left_node, right_child: right_node};
        Result::Ok(branch)
    }

}