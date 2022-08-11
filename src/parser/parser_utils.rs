use crate::{
    parser::Parser,
    token::{Token, TokenType},
};

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

    pub(super) fn assert_token(&self, token_type: TokenType, message: &str) -> Result<(), String> {
        if !self.is_token_of_type(&[token_type]) {
            return Err(String::from(message));
        }

        Ok(())
    }
}
