use super::{parsing_error::ParsingError, Parser, ParsingResult};
use crate::token::{Token, TokenType};

impl<'p> Parser<'p> {
    pub(super) fn advance_cursor(&self) {
        *self.current.borrow_mut() += 1;
    }

    pub(super) fn get_token(&self) -> Option<&Token> {
        if *self.current.borrow() >= self.tokens.len() {
            return None;
        }

        Some(&self.tokens[*self.current.borrow()])
    }

    pub(super) fn get_last_token(&self) -> &Token {
        &self.tokens.last().unwrap()
    }

    pub(super) fn is_token_of_type(&self, token_types: &[TokenType]) -> bool {
        match self.get_token() {
            Some(token) => {
                for token_type in token_types {
                    if &token.token_type == token_type {
                        return true;
                    }
                }

                return false;
            }
            None => false,
        }
    }

    pub(super) fn assert_token(
        &'p self,
        token_type: TokenType,
        message: &'p str,
    ) -> ParsingResult<'p, ()> {
        if !self.is_token_of_type(&[token_type]) {
            return ParsingError::new(message, self.get_token().unwrap_or(self.get_last_token()));
        }

        Ok(())
    }
}
