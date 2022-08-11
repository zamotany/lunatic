use crate::{
    ast::Expression,
    parser::{Parser, ParsingResult},
    token::TokenType,
};

impl<'p> Parser<'p> {
    /// Utility function to parse a binary expression.
    fn try_parse_binary_expression<M, L, R>(
        &self,
        matches_token: M,
        parse_left: L,
        parse_right: R,
    ) -> ParsingResult<Expression>
    where
        M: FnOnce(&TokenType) -> bool,
        L: FnOnce() -> ParsingResult<Expression<'p>>,
        R: FnOnce() -> ParsingResult<Expression<'p>>,
    {
        let left = parse_left()?;
        if left.is_none() {
            return Ok(None);
        }

        if let Some(token) = self.get_token() {
            if matches_token(&token.token_type) {
                self.advance_cursor();
                return match parse_right()? {
                    Some(right) => Ok(Some(Expression::Binary {
                        left: Box::new(left.unwrap()),
                        operator: token,
                        right: Box::new(right),
                    })),
                    None => Err(String::from("Failed to parse operand of binary expression")),
                };
            }
        }

        Ok(left)
    }
}

/// Parsing methods.
impl<'p> Parser<'p> {
    pub(super) fn parse_maybe_binary_or(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::Or,
            || self.parse_maybe_binary_and(),
            || self.parse_maybe_expression(),
        )
    }

    pub(super) fn parse_maybe_binary_and(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::And,
            || self.parse_maybe_binary_relation(),
            || self.parse_maybe_binary_and(),
        )
    }

    pub(super) fn parse_maybe_binary_relation(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| match token_type {
                TokenType::Less
                | TokenType::LessEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::TildeEqual
                | TokenType::EqualEqual => true,
                _ => false,
            },
            || self.parse_maybe_binary_bitwise_or(),
            || self.parse_maybe_binary_relation(),
        )
    }

    pub(super) fn parse_maybe_binary_bitwise_or(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::Pipe,
            || self.parse_maybe_binary_bitwise_xor(),
            || self.parse_maybe_binary_bitwise_or(),
        )
    }

    pub(super) fn parse_maybe_binary_bitwise_xor(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::Tilde,
            || self.parse_maybe_binary_bitwise_and(),
            || self.parse_maybe_binary_bitwise_xor(),
        )
    }

    pub(super) fn parse_maybe_binary_bitwise_and(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::Ampersand,
            || self.parse_maybe_binary_shift(),
            || self.parse_maybe_binary_bitwise_and(),
        )
    }

    pub(super) fn parse_maybe_binary_shift(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| match token_type {
                TokenType::LessLess | TokenType::GreaterGreater => true,
                _ => false,
            },
            || self.parse_maybe_binary_concat(),
            || self.parse_maybe_binary_shift(),
        )
    }

    pub(super) fn parse_maybe_binary_concat(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::DotDot,
            || self.parse_maybe_binary_arithmetic_simple(),
            || self.parse_maybe_binary_concat(),
        )
    }

    pub(super) fn parse_maybe_binary_arithmetic_simple(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| match token_type {
                TokenType::Plus | TokenType::Minus => true,
                _ => false,
            },
            || self.parse_maybe_binary_arithmetic_complex(),
            || self.parse_maybe_binary_arithmetic_simple(),
        )
    }

    pub(super) fn parse_maybe_binary_arithmetic_complex(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| match token_type {
                TokenType::Star | TokenType::Slash | TokenType::SlashSlash | TokenType::Percent => {
                    true
                }
                _ => false,
            },
            || self.parse_maybe_unary(),
            || self.parse_maybe_binary_arithmetic_complex(),
        )
    }

    pub(super) fn parse_maybe_binary_exponent(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::Caret,
            || self.parse_maybe_var_access(),
            || self.parse_maybe_binary_exponent(),
        )
    }
}
