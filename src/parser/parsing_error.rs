use std::convert::From;
use crate::token::Token;

use super::ParsingResult;

#[derive(Debug)]
pub struct ParsingError<'a> {
    pub message: String,
    pub token: &'a Token<'a>,
}

impl <'a> ParsingError<'a> {
    pub fn new<T>(message: &'a str, token: &'a Token<'a>) -> ParsingResult<'a, T> {
        Err(ParsingError { message: String::from(message), token })
    }

    pub fn end_of_tokens<T>(token: &'a Token<'a>) -> ParsingResult<'a, T> {
        ParsingError::new("Unexpected end of tokens", token)
    }

    pub fn unexpected_token<T>(token: &'a Token<'a>) -> ParsingResult<'a, T> {
        ParsingError::new("Unexpected token", token)
    }
}
