use crate::{ast::ASTNodeType::ConstantExpression, token::Node};

#[derive(Debug)]
pub struct ASTNode {
    pub _type: ASTNodeType,
    pub value: Node,
    pub left_child: Option<Box<ASTNode>>,
    pub right_child: Option<Box<ASTNode>>
}


#[derive(Debug)]
pub enum ASTNodeType {
    BinaryExpression,
    ConstantExpression
}