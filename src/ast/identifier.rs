use crate::token::Token;

#[derive(Debug)]
pub struct Identifier<'a> {
    pub token: &'a Token<'a>,
}

impl <'a> Identifier<'a> {
    pub fn new(token: &'a Token<'a>) -> Identifier<'a> {
        Identifier { token }
    }
}