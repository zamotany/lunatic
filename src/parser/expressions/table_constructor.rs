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
                        if let Some(field) = self.parse_field()? {
                            fields.push(field);
                        } else {
                            break;
                        }

                        if self.is_token_of_type(&[TokenType::Comma, TokenType::Semicolon]) {
                            self.advance_cursor();
                        }
                    }

                    self.assert_token(TokenType::RightBrace, "Expected '}' after field list")?;
                    self.advance_cursor();

                    Ok(Some(Expression::TableConstructor { fields }))
                }
                _ => self.parse_maybe_literal(),
            };
        }

        Ok(None)
    }

    fn parse_field(&self) -> ParsingResult<Field> {
        if let Some(token) = self.get_token() {
            if token.token_type == TokenType::LeftBracket {
                self.advance_cursor();

                return match self.parse_maybe_expression()? {
                    Some(key) => {
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

                        match self.parse_maybe_expression()? {
                            Some(value) => Ok(Some(Field::Expression { key, value })),
                            None => Err(String::from(
                                "Failed to parse value expression in field of table constructor",
                            )),
                        }
                    }
                    None => Err(String::from(
                        "Failed to parse key expression in field of table constructor",
                    )),
                };
            } else {
                return match self.parse_identifier()? {
                    Some(key) => {
                        self.assert_token(
                            TokenType::Equal,
                            "Expected '=' in field initialization",
                        )?;
                        self.advance_cursor();

                        match self.parse_maybe_expression()? {
                            Some(value) => Ok(Some(Field::Normal { key, value })),
                            None => Err(String::from(
                                "Failed to parse value expression in field of table constructor",
                            )),
                        }
                    }
                    None if !self.is_token_of_type(&[TokenType::Equal]) => {
                        match self.parse_maybe_expression()? {
                            Some(value) => Ok(Some(Field::Anonymous { value })),
                            None => Ok(None),
                        }
                    }
                    _ => Err(String::from("Failed to parse field of table constructor")),
                };
            }
        }

        Ok(None)
    }
}
