use super::{prefix::Prefix, PrefixVisitor, TableConstructor, TableConstructorVisitor};
use crate::token::Token;

#[derive(Debug)]
pub enum Expression<'e> {
    Literal(&'e Token<'e>),
    Unary {
        operator: &'e Token<'e>,
        right: Box<Expression<'e>>,
    },
    Binary {
        left: Box<Expression<'e>>,
        operator: &'e Token<'e>,
        right: Box<Expression<'e>>,
    },
    TableConstructor(TableConstructor<'e>),
    Prefix(Prefix<'e>),
}

pub trait ExpressionVisitor<T>: PrefixVisitor<T> + TableConstructorVisitor<T> {
    fn visit_literal(&self, token: &Token) -> T;
    fn visit_unary(&self, operator: &Token, right: &Box<Expression>) -> T;
    fn visit_binary(&self, left: &Box<Expression>, operator: &Token, right: &Box<Expression>) -> T;
    fn visit_table_constructor(&self, table_constructor: &TableConstructor) -> T {
        table_constructor.visit(self)
    }
    fn visit_prefix(&self, prefix: &Prefix) -> T {
        prefix.visit(self)
    }
}

impl<'a> Expression<'a> {
    pub fn visit<T, V>(&self, visitor: &V) -> T
    where
        V: ExpressionVisitor<T>,
    {
        match self {
            Expression::Literal(token) => visitor.visit_literal(token),
            Expression::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expression::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expression::TableConstructor(table_constructor) => {
                visitor.visit_table_constructor(table_constructor)
            }
            Expression::Prefix(prefix) => visitor.visit_prefix(prefix),
        }
    }
}
