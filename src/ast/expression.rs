use crate::token::Token;

#[derive(Debug)]
pub enum Expression<'e> {
    Literal(&'e Token<'e>),
    Group(Box<Expression<'e>>),
    Unary(&'e Token<'e>, Box<Expression<'e>>),
}
