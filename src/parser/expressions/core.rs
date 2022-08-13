use crate::{
    ast::{Expression, Identifier},
    parser::{parsing_error::ParsingError, Parser, ParsingResult},
    token::TokenType,
};

impl<'p> Parser<'p> {
    pub(super) fn parse_maybe_spread(&self) -> ParsingResult<Expression> {
        self.parse_maybe_function_definition()
    }

    fn parse_maybe_function_definition(&self) -> ParsingResult<Expression> {
        self.parse_maybe_binary_or()
    }

    pub(super) fn try_parse_identifier(&self) -> ParsingResult<Option<Identifier>> {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::Identifier => {
                    self.advance_cursor();
                    Ok(Some(Identifier(token)))
                }
                _ => Ok(None),
            };
        }

        ParsingError::end_of_tokens(self.get_last_token())
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
                    Ok(Expression::Literal(token))
                }
                _ => ParsingError::unexpected_token(token),
            };
        }

        ParsingError::end_of_tokens(self.get_last_token())
    }
}
