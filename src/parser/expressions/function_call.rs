use crate::{
    ast::{Args, Expression, FunctionCall, Prefix},
    parser::{parsing_error::ParsingError, Parser, ParsingResult},
    token::TokenType,
};

impl<'p> Parser<'p> {
    pub(super) fn parse_maybe_function_call(&self) -> ParsingResult<Expression> {
        match self.parse_maybe_var_access()? {
            Expression::Prefix(prefix) => {
                if let Some(token) = self.get_token() {
                    if token.token_type == TokenType::Colon {
                        self.advance_cursor();

                        if let Some(identifier) = self.try_parse_identifier()? {
                            return match self.try_parse_function_args()? {
                                Some(args) => Ok(Expression::Prefix(Prefix::FunctionCall(
                                    FunctionCall::MethodCall {
                                        callee: Box::new(prefix),
                                        method: identifier,
                                        args,
                                    },
                                ))),
                                None => Ok(Expression::Prefix(prefix)),
                            };
                        }

                        return ParsingError::new(
                            "Expected identifier after `:` in method call",
                            self.get_last_token(),
                        );
                    }
                }

                match self.try_parse_function_args()? {
                    Some(args) => Ok(Expression::Prefix(Prefix::FunctionCall(
                        FunctionCall::FunctionCall {
                            callee: Box::new(prefix),
                            args,
                        },
                    ))),
                    None => Ok(Expression::Prefix(prefix)),
                }
            }
            expression => Ok(expression),
        }
    }

    fn try_parse_function_args(&self) -> ParsingResult<Option<Args>> {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::LeftParen => {
                    self.advance_cursor();

                    if let Some(next_token) = self.get_token() {
                        if next_token.token_type == TokenType::RightParen {
                            self.advance_cursor();
                            return Ok(Some(Args::ExpressionList(Vec::new())));
                        }
                    }

                    let mut args = Vec::new();
                    loop {
                        let expression = self.parse_maybe_expression()?;
                        args.push(expression);

                        if !self.is_token_of_type(&[TokenType::Comma]) {
                            break;
                        } else {
                            self.advance_cursor();
                        }
                    }

                    self.assert_token(TokenType::RightParen, "Expected ')' after arguments list")?;
                    self.advance_cursor();

                    Ok(Some(Args::ExpressionList(args)))
                }
                TokenType::LeftBrace => {
                    let table_constructor = self.parse_table_constructor()?;
                    Ok(Some(Args::TableConstructor(table_constructor)))
                }
                TokenType::LiteralString => Ok(Some(Args::LiteralString(token))),
                _ => Ok(None),
            };
        }

        ParsingError::end_of_tokens(self.get_last_token())
    }
}
