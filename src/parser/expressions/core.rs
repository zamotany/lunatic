use crate::{
    ast::{Expression, Identifier},
    parser::{Parser, ParsingResult},
    token::TokenType,
};


impl<'p> Parser<'p> {
    pub(super) fn parse_maybe_spread(&self) -> ParsingResult<Expression> {
        self.parse_maybe_function_definition()
    }

    fn parse_maybe_function_definition(&self) -> ParsingResult<Expression> {
        self.parse_maybe_binary_or()
    }

    pub(super) fn parse_identifier(&self) -> ParsingResult<Identifier> {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::Identifier => {
                    self.advance_cursor();
                    Ok(Some(Identifier(token)))
                }
                _ => Ok(None),
            };
        }

        Ok(None)
    }

    pub(super) fn parse_maybe_literal(&self) -> ParsingResult<Expression> {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::False
                | TokenType::True
                | TokenType::Nil
                | TokenType::Numeral
                | TokenType::LiteralString => {
                    self.advance_cursor();
                    Ok(Some(Expression::Literal(token)))
                }
                _ => Ok(None),
            };
        }

        Ok(None)
    }
}
