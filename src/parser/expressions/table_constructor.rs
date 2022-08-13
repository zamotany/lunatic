use crate::{
    ast::{Expression, Field},
    parser::{Parser, ParsingResult},
    token::TokenType,
};

impl<'p> Parser<'p> {
    pub(super) fn parse_maybe_table_constructor(&self) -> ParsingResult<Expression> {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::LeftBrace => {
                    self.advance_cursor();

                    let mut fields = Vec::new();
                    while !self.is_token_of_type(&[TokenType::RightBrace]) {
                        let field = self.parse_field()?;
                        fields.push(field);

                        if self.is_token_of_type(&[TokenType::Comma, TokenType::Semicolon]) {
                            self.advance_cursor();
                        }
                    }

                    self.assert_token(TokenType::RightBrace, "Expected '}' after field list")?;
                    self.advance_cursor();

                    Ok(Expression::TableConstructor { fields })
                }
                _ => self.parse_maybe_literal(),
            };
        }

        Err(String::from("Unexpected end of tokens"))
    }

    fn parse_field(&self) -> ParsingResult<Field> {
        if let Some(token) = self.get_token() {
            if token.token_type == TokenType::LeftBracket {
                self.advance_cursor();

                let key = self.parse_maybe_expression()?;

                self.assert_token(
                    TokenType::RightBracket,
                    "Expected ']' in field initialization",
                )?;
                self.advance_cursor();

                self.assert_token(
                    TokenType::Equal,
                    "Expected '=' in field initialization",
                )?;
                self.advance_cursor();

                let value = self.parse_maybe_expression()?;
                return Ok(Field::Expression { key, value });
            } else {
                return match self.try_parse_identifier()? {
                    Some(key) => {
                        self.assert_token(
                            TokenType::Equal,
                            "Expected '=' in field initialization",
                        )?;
                        self.advance_cursor();

                        let value = self.parse_maybe_expression()?;
                        Ok(Field::Normal { key, value })
                    }
                    None if !self.is_token_of_type(&[TokenType::Equal]) => {
                        let value = self.parse_maybe_expression()?;
                        Ok(Field::Anonymous { value })
                    }
                    _ => Err(String::from("Failed to parse field of table constructor")),
                };
            }
        }

        Err(String::from("Unexpected end of tokens"))
    }
}
