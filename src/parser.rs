use crate::{
    ast::{
        expression::Expression, field::Field, identifier::Identifier,
        table_constructor::TableConstructor,
    },
    token::{Token, TokenType},
};
use std::cell::RefCell;

type ParsingResult<T> = Result<Option<T>, String>;

/// Lua parser.
///
/// Specs:
/// - https://www.lua.org/manual/5.4/manual.html#9
/// - http://www.lua.org/manual/5.4/manual.html#3.4.8
pub struct Parser<'p> {
    tokens: &'p [Token<'p>],
    current: RefCell<usize>,
}

/// Public methods.
impl<'p> Parser<'p> {
    pub fn new(tokens: &'p [Token<'p>]) -> Parser<'p> {
        Parser {
            tokens,
            current: RefCell::new(0),
        }
    }

    pub fn parse(&self) -> ParsingResult<Expression> {
        self.parse_maybe_expression()
    }
}

/// Private utilities.
impl<'p> Parser<'p> {
    fn advance_cursor(&self) {
        *self.current.borrow_mut() += 1;
    }

    fn get_token(&self) -> Option<&Token> {
        if *self.current.borrow() >= self.tokens.len() {
            return None;
        }

        Some(&self.tokens[*self.current.borrow()])
    }

    fn is_token_of_type(&self, token_types: &[TokenType]) -> bool {
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

    fn assert_token(&self, token_type: TokenType, message: &str) -> Result<(), String> {
        if !self.is_token_of_type(&[token_type]) {
            return Err(String::from(message));
        }

        Ok(())
    }

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
                    Some(right) => Ok(Some(Expression::Binary(
                        Box::new(left.unwrap()),
                        token,
                        Box::new(right),
                    ))),
                    None => Err(String::from("Failed to parse operand of binary expression")),
                };
            }
        }

        Ok(left)
    }
}

/// Private parsing methods.
impl<'p> Parser<'p> {
    fn parse_maybe_expression(&self) -> ParsingResult<Expression> {
        self.parse_maybe_spread()
    }

    fn parse_maybe_spread(&self) -> ParsingResult<Expression> {
        self.parse_maybe_function_definition()
    }

    fn parse_maybe_function_definition(&self) -> ParsingResult<Expression> {
        self.parse_maybe_binary_or()
    }

    fn parse_maybe_binary_or(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::Or,
            || self.parse_maybe_binary_and(),
            || self.parse_maybe_expression(),
        )
    }

    fn parse_maybe_binary_and(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::And,
            || self.parse_maybe_binary_relation(),
            || self.parse_maybe_binary_and(),
        )
    }

    fn parse_maybe_binary_relation(&self) -> ParsingResult<Expression> {
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

    fn parse_maybe_binary_bitwise_or(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::Pipe,
            || self.parse_maybe_binary_bitwise_xor(),
            || self.parse_maybe_binary_bitwise_or(),
        )
    }

    fn parse_maybe_binary_bitwise_xor(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::Tilde,
            || self.parse_maybe_binary_bitwise_and(),
            || self.parse_maybe_binary_bitwise_xor(),
        )
    }

    fn parse_maybe_binary_bitwise_and(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::Ampersand,
            || self.parse_maybe_binary_shift(),
            || self.parse_maybe_binary_bitwise_and(),
        )
    }

    fn parse_maybe_binary_shift(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| match token_type {
                TokenType::LessLess | TokenType::GreaterGreater => true,
                _ => false,
            },
            || self.parse_maybe_binary_concat(),
            || self.parse_maybe_binary_shift(),
        )
    }

    fn parse_maybe_binary_concat(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::DotDot,
            || self.parse_maybe_binary_arithmetic_simple(),
            || self.parse_maybe_binary_concat(),
        )
    }

    fn parse_maybe_binary_arithmetic_simple(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| match token_type {
                TokenType::Plus | TokenType::Minus => true,
                _ => false,
            },
            || self.parse_maybe_binary_arithmetic_complex(),
            || self.parse_maybe_binary_arithmetic_simple(),
        )
    }

    fn parse_maybe_binary_arithmetic_complex(&self) -> ParsingResult<Expression> {
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

    fn parse_maybe_unary(&self) -> ParsingResult<Expression> {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::Minus | TokenType::Not | TokenType::Hash | TokenType::Tilde => {
                    self.advance_cursor();
                    let right = self.parse_maybe_unary()?;
                    match right {
                        Some(right) => Ok(Some(Expression::Unary(token, Box::new(right)))),
                        None => Err(String::from("Failed to parse operand of unary expression")),
                    }
                }
                _ => self.parse_maybe_binary_exponent(),
            };
        }

        Ok(None)
    }

    fn parse_maybe_binary_exponent(&self) -> ParsingResult<Expression> {
        self.try_parse_binary_expression(
            |token_type| token_type == &TokenType::Caret,
            || self.parse_maybe_prefix(),
            || self.parse_maybe_binary_exponent(),
        )
    }

    fn parse_maybe_prefix(&self) -> ParsingResult<Expression> {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                // TODO: var
                // TODO: functioncall
                TokenType::LeftParen => {
                    self.advance_cursor();
                    let expression = self.parse_maybe_expression()?;
                    self.advance_cursor();
                    match expression {
                        Some(expression) => Ok(Some(Expression::Group(Box::new(expression)))),
                        None => Err(String::from("Expected ')' after expression")),
                    }
                }
                _ => self.parse_maybe_table_constructor(),
            };
        }

        Ok(None)
    }

    fn parse_maybe_table_constructor(&self) -> ParsingResult<Expression> {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::LeftBrace => {
                    self.advance_cursor();
                    let field = self.parse_field()?;

                    if self.is_token_of_type(&[TokenType::Comma, TokenType::Semicolon]) {
                        self.advance_cursor();
                    }

                    self.assert_token(TokenType::RightBrace, "Expected '}' after field list")?;
                    self.advance_cursor();

                    match field {
                        Some(field) => Ok(Some(TableConstructor::new_expression(vec![field]))),
                        None => Ok(Some(TableConstructor::empty_expression())),
                    }
                }
                _ => self.parse_maybe_literal(),
            };
        }

        Ok(None)
    }

    fn parse_field(&self) -> ParsingResult<Field> {
        if let Some(token) = self.get_token() {
            if token.token_type == TokenType::LeftBracket {
                // expression
                todo!();
            } else {
                let key = self.parse_identifier()?;

                self.assert_token(TokenType::Equal, "Expected '=' in field initialization")?;
                self.advance_cursor();

                let value = self.parse_maybe_expression()?;

                if key.is_some() && value.is_some() {
                    return Ok(Some(Field::new(key.unwrap(), value.unwrap())));
                }

                return Ok(None);
            }
        }

        Ok(None)
    }

    fn parse_identifier(&self) -> ParsingResult<Identifier> {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::Identifier => {
                    self.advance_cursor();
                    Ok(Some(Identifier::new(token)))
                }
                _ => Ok(None),
            };
        }

        Ok(None)
    }

    fn parse_maybe_literal(&self) -> ParsingResult<Expression> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{debug_visitor, scanner};

    fn expect_source_to_equal_ast(source: &str, expected: &str) {
        let mut scanner = scanner::Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap().unwrap();
        let debug_visitor = debug_visitor::DebugVisitor::new();
        let output = ast.visit(&debug_visitor);
        assert_eq!(&output[..], expected);
    }

    #[test]
    fn should_parse_expressions() {
        expect_source_to_equal_ast(
            "true or (false and true) and true",
            "[or l=`true` r=[and l=([and l=`false` r=`true`]) r=`true`]]",
        );
        expect_source_to_equal_ast(
            "true and false and true",
            "[and l=`true` r=[and l=`false` r=`true`]]",
        );
        expect_source_to_equal_ast(
            "true or false and true and true",
            "[or l=`true` r=[and l=`false` r=[and l=`true` r=`true`]]]",
        );
        expect_source_to_equal_ast(
            "true or false or false and true",
            "[or l=`true` r=[or l=`false` r=[and l=`false` r=`true`]]]",
        );
        expect_source_to_equal_ast("1 >= 2 or 3", "[or l=[>= l=`1` r=`2`] r=`3`]");
        expect_source_to_equal_ast("false or 1 > 2", "[or l=`false` r=[> l=`1` r=`2`]]");
        expect_source_to_equal_ast("1 >= 2 and 3", "[and l=[>= l=`1` r=`2`] r=`3`]");
        expect_source_to_equal_ast(
            "false and 5 >= 5 or 11 < 10",
            "[or l=[and l=`false` r=[>= l=`5` r=`5`]] r=[< l=`11` r=`10`]]",
        );
        expect_source_to_equal_ast(
            "2 == 2 ^ 1 or true",
            "[or l=[== l=`2` r=[^ l=`2` r=`1`]] r=`true`]",
        );
        expect_source_to_equal_ast("not true or true", "[or l=[not r=`true`] r=`true`]");
        expect_source_to_equal_ast(
            "2 / 2 == 1 and true",
            "[and l=[== l=[/ l=`2` r=`2`] r=`1`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "2 - 1 == 1 and true",
            "[and l=[== l=[- l=`2` r=`1`] r=`1`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "'hello' .. 'world' ~= 0 or true",
            "[or l=[~= l=[.. l=`'hello'` r=`'world'`] r=`0`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "2 << 2 == 8 or true",
            "[or l=[== l=[<< l=`2` r=`2`] r=`8`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "1 & 1 == 3 or true",
            "[or l=[== l=[& l=`1` r=`1`] r=`3`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "1 ~ 1 == 3 or true",
            "[or l=[== l=[~ l=`1` r=`1`] r=`3`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "1 | 1 == 3 or true",
            "[or l=[== l=[| l=`1` r=`1`] r=`3`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "(1 ~= 2 or (true or 2 << 1 == 4)) and false and (true or false)",
            "[and l=([or l=[~= l=`1` r=`2`] r=([or l=`true` r=[== l=[<< l=`2` r=`1`] r=`4`]])]) r=[and l=`false` r=([or l=`true` r=`false`])]]"
        );
    }

    #[test]
    fn should_parse_table_constructor() {
        expect_source_to_equal_ast("{ foo = 1, }", "Tc[`foo`=`1` ]");
    }
}
