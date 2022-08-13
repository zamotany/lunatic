use crate::{
    ast::{Expression, Identifier, Prefix, Variable},
    parser::{Parser, ParsingResult},
    token::TokenType,
};

impl<'p> Parser<'p> {
    pub(super) fn parse_maybe_var_access(&self) -> ParsingResult<Expression> {
        match self.parse_maybe_var_identifier()? {
            Expression::Prefix(prefix) => {
                let mut current_prefix = prefix;
                let mut error: Result<(), String> = Result::Ok(());

                while let Some(token) = self.get_token() {
                    match token.token_type {
                        TokenType::LeftBracket => {
                            self.advance_cursor();

                            let expression = self.parse_maybe_expression()?;
                            current_prefix = Prefix::Variable(Variable::ExpressionMemberAccess {
                                reference: Box::new(current_prefix),
                                member: Box::new(expression),
                            });

                            self.assert_token(
                                TokenType::RightBracket,
                                "Expected `]` after expression",
                            )?;
                            self.advance_cursor();
                        }
                        TokenType::Dot => {
                            self.advance_cursor();

                            match self.try_parse_identifier()? {
                                Some(identifier) => {
                                    current_prefix = Prefix::Variable(Variable::MemberAccess {
                                        reference: Box::new(current_prefix),
                                        member: identifier,
                                    })
                                }
                                None => {
                                    error = Err(String::from("Expected identifier after `.`"));
                                    break;
                                }
                            };
                        }
                        _ => {
                            break;
                        }
                    }
                }

                if error.is_err() {
                    return Err(error.unwrap_err());
                }

                Ok(Expression::Prefix(current_prefix))
            }
            expression => Ok(expression),
        }
    }

    fn parse_maybe_var_identifier(&self) -> ParsingResult<Expression> {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::Identifier => {
                    self.advance_cursor();
                    return Ok(Expression::Prefix(Prefix::Variable(Variable::Identifier(
                        Identifier(token),
                    ))));
                }
                _ => self.parse_maybe_prefix(),
            };
        }

        Err(String::from("Unexpected end of tokens"))
    }
}
