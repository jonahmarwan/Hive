use crate::token::Node;

#[derive(Debug)]
pub struct ASTNode {
    pub _type: ASTNodeType,
    pub value: Node,
    pub left_child: Node,
    pub right_child: Node
}

#[derive(Debug)]
pub enum ASTNodeType {
    BinaryExpression,
}