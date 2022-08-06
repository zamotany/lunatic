use super::{table_constructor::TableConstructor, prefix::Prefix};
use crate::token::Token;

#[derive(Debug)]
pub enum Expression<'e> {
    Literal(&'e Token<'e>),
    Unary(&'e Token<'e>, Box<Expression<'e>>),
    Binary(Box<Expression<'e>>, &'e Token<'e>, Box<Expression<'e>>),
    TableConstructor(TableConstructor<'e>),
    Prefix(Prefix<'e>)
}

pub trait ExpressionVisitor {
    type Output;

    fn visit_literal(&self, token: &Token) -> Self::Output;
    fn visit_unary(&self, operator: &Token, right: &Box<Expression>) -> Self::Output;
    fn visit_binary(
        &self,
        left: &Box<Expression>,
        operator: &Token,
        right: &Box<Expression>,
    ) -> Self::Output;
    fn visit_table_constructor(&self, table_constructor: &TableConstructor) -> Self::Output;
    fn visit_prefix(&self, prefix: &Prefix) -> Self::Output;
}

impl<'a> Expression<'a> {
    pub fn visit<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        match self {
            Expression::Literal(token) => visitor.visit_literal(token),
            Expression::Unary(operator, right) => visitor.visit_unary(operator, right),
            Expression::Binary(left, operator, right) => {
                visitor.visit_binary(left, operator, right)
            }
            Expression::TableConstructor(table_constructor) => {
                visitor.visit_table_constructor(table_constructor)
            }
            Expression::Prefix(prefix) => visitor.visit_prefix(prefix),
        }
    }
}
