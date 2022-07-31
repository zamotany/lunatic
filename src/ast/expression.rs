use crate::token::Token;

#[derive(Debug)]
pub enum Expression<'e> {
    Literal(&'e Token<'e>),
    Group(Box<Expression<'e>>),
    Unary(&'e Token<'e>, Box<Expression<'e>>),
    Binary(Box<Expression<'e>>, &'e Token<'e>, Box<Expression<'e>>),
}

pub trait ExpressionVisitor {
    type Output;

    fn visit_literal(&self, token: &Token) -> Self::Output;
    fn visit_group(&self, expression: &Box<Expression>) -> Self::Output;
    fn visit_unary(&self, operator: &Token, right: &Box<Expression>) -> Self::Output;
    fn visit_binary(
        &self,
        left: &Box<Expression>,
        operator: &Token,
        right: &Box<Expression>,
    ) -> Self::Output;
}

impl<'a> Expression<'a> {
    pub fn visit<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        match self {
            Expression::Literal(token) => visitor.visit_literal(token),
            Expression::Group(expression) => visitor.visit_group(expression),
            Expression::Unary(operator, right) => visitor.visit_unary(operator, right),
            Expression::Binary(left, operator, right) => {
                visitor.visit_binary(left, operator, right)
            }
        }
    }
}
