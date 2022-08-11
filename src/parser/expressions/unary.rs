use crate::{
    ast::Expression,
    parser::{Parser, ParsingResult},
    token::TokenType,
};

impl<'p> Parser<'p> {
    pub(super) fn parse_maybe_unary(&self) -> ParsingResult<Expression> {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::Minus | TokenType::Not | TokenType::Hash | TokenType::Tilde => {
                    self.advance_cursor();
                    let right = self.parse_maybe_unary()?;
                    match right {
                        Some(right) => Ok(Some(Expression::Unary {
                            operator: token,
                            right: Box::new(right),
                        })),
                        None => Err(String::from("Failed to parse operand of unary expression")),
                    }
                }
                _ => self.parse_maybe_binary_exponent(),
            };
        }

        Ok(None)
    }
}