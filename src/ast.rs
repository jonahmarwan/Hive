use crate::{token::Node};

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub _type: ASTNodeType,
    pub value: Node,
    pub left_child: Option<Box<ASTNode>>,
    pub right_child: Option<Box<ASTNode>>
}


#[derive(Debug, Clone)]
pub enum ASTNodeType {
    BinaryExpression,
    Literal,
    Identifier,
    Assignment,
    IdenDeclarator
}