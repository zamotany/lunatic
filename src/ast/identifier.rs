use crate::token::Token;

#[derive(Debug)]
pub enum Identifier<'a> {
    Anonymous,
    Named(&'a Token<'a>),
}
