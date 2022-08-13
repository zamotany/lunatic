use crate::{
    ast::{Expression, Identifier, Prefix, Variable},
    parser::{Parser, ParsingResult},
    token::TokenType,
};

impl<'p> Parser<'p> {
    pub(super) fn parse_maybe_prefix(&self) -> ParsingResult<Expression> {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::Identifier => {
                    self.advance_cursor();
                    return Ok(Expression::Prefix(Prefix::Variable(Variable::Identifier(
                        Identifier(token),
                    ))));
                }
                // TODO: functioncall
                TokenType::LeftParen => {
                    self.advance_cursor();
                    let expression = self.parse_maybe_expression()?;
                    self.assert_token(TokenType::RightParen, "Expected `)` after expression")?;
                    self.advance_cursor();
                    Ok(Expression::Prefix(Prefix::Group(Box::new(
                        expression,
                    ))))
                }
                _ => self.parse_maybe_table_constructor(),
            };
        }

        Err(String::from("Unexpected end of tokens"))
    }
}
