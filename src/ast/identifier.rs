use crate::token::Token;

#[derive(Debug)]
pub struct Identifier<'a>(pub &'a Token<'a>);
