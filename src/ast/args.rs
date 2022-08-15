use super::{Expression, TableConstructor};
use crate::token::Token;

#[derive(Debug)]
pub enum Args<'a> {
    ExpressionList(Vec<Expression<'a>>),
    TableConstructor(TableConstructor<'a>),
    LiteralString(&'a Token<'a>),
}

pub trait ArgsVisitor<T> {
    fn visit_args_expression_list(&self, expressions: &Vec<Expression>) -> T;
    fn visit_args_table_constructor(&self, table_constructor: &TableConstructor) -> T;
    fn visit_args_literal_string(&self, token: &Token) -> T;
}

impl<'a> Args<'a> {
    pub fn visit<T, V>(&self, visitor: &V) -> T
    where
        V: ArgsVisitor<T> + ?Sized,
    {
        match self {
            Args::ExpressionList(expressions) => visitor.visit_args_expression_list(expressions),
            Args::TableConstructor(table_constructor) => {
                visitor.visit_args_table_constructor(table_constructor)
            }
            Args::LiteralString(token) => visitor.visit_args_literal_string(token),
        }
    }
}
