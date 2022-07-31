use crate::{
    ast::expression::Expression,
    token::{Token, TokenType},
};
use std::cell::RefCell;

type ParsingResult<'e> = Result<Option<Expression<'e>>, String>;

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

    pub fn parse(&self) -> ParsingResult {
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

    /// Utility function to parse a single expression for given token.
    fn try_parse_single_token<L, R>(
        &self,
        token_type: TokenType,
        parse_left: L,
        parse_right: R,
    ) -> ParsingResult
    where
        L: FnOnce() -> ParsingResult<'p>,
        R: FnOnce() -> ParsingResult<'p>,
    {
        let left = parse_left()?;

        if let Some(token) = self.get_token() {
            if token.token_type == token_type && left.is_some() {
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
    fn parse_maybe_expression(&self) -> ParsingResult {
        self.parse_maybe_spread()
    }

    fn parse_maybe_spread(&self) -> ParsingResult {
        self.parse_maybe_function_definition()
    }

    fn parse_maybe_function_definition(&self) -> ParsingResult {
        self.parse_maybe_table_constructor()
    }

    fn parse_maybe_table_constructor(&self) -> ParsingResult {
        self.parse_maybe_binary_or()
    }

    fn parse_maybe_binary_or(&self) -> ParsingResult {
        self.try_parse_single_token(
            TokenType::Or,
            || self.parse_maybe_binary_and(),
            || self.parse_maybe_expression(),
        )
    }

    fn parse_maybe_binary_and(&self) -> ParsingResult {
        self.try_parse_single_token(
            TokenType::And,
            || self.parse_maybe_binary_relation(),
            || self.parse_maybe_expression(),
        )
    }

    fn parse_maybe_binary_relation(&self) -> ParsingResult {
        let left = self.parse_maybe_binary_generic()?;
        if left.is_none() {
            return Ok(None);
        }

        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::Less
                | TokenType::LessEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::TildeEqual
                | TokenType::EqualEqual => {
                    self.advance_cursor();
                    let right = self.parse_maybe_binary_generic()?;
                    match right {
                        Some(right) => Ok(Some(Expression::Binary(
                            Box::new(left.unwrap()),
                            token,
                            Box::new(right),
                        ))),
                        None => Err(String::from("Failed to parse operand of binary expression")),
                    }
                }
                _ => Ok(left),
            };
        }

        Ok(None)
    }

    // fn parse_maybe_binary_bitwise_or(&self) -> ParsingResult {}

    // fn parse_maybe_binary_bitwise_xor(&self) -> ParsingResult {}

    // fn parse_maybe_binary_bitwise_and(&self) -> ParsingResult {}

    // fn parse_maybe_binary_shift(&self) -> ParsingResult {}

    // fn parse_maybe_binary_concat(&self) -> ParsingResult {}

    // fn parse_maybe_binary_arithmetic_simple(&self) -> ParsingResult {}

    // fn parse_maybe_binary_arithmetic_complex(&self) -> ParsingResult {}

    // fn parse_maybe_binary_exponent(&self) -> ParsingResult {}

    /// TODO: split into parse_maybe_binary_*
    fn parse_maybe_binary_generic(&self) -> ParsingResult {
        let left = self.parse_maybe_prefix()?;
        if left.is_none() {
            return Ok(None);
        }

        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::LessLess
                | TokenType::Pipe
                | TokenType::Tilde
                | TokenType::Ampersand
                | TokenType::GreaterGreater
                | TokenType::DotDot
                | TokenType::Plus
                | TokenType::Minus
                | TokenType::Star
                | TokenType::Slash
                | TokenType::SlashSlash
                | TokenType::Percent
                | TokenType::Caret => {
                    self.advance_cursor();
                    let right = self.parse_maybe_binary_generic()?;
                    match right {
                        Some(right) => Ok(Some(Expression::Binary(
                            Box::new(left.unwrap()),
                            token,
                            Box::new(right),
                        ))),
                        None => Err(String::from("Failed to parse operand of binary expression")),
                    }
                }
                _ => Ok(left),
            };
        }

        Ok(None)
    }

    fn parse_maybe_prefix(&self) -> ParsingResult {
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
                _ => self.parse_maybe_unary(),
            };
        }

        Ok(None)
    }

    fn parse_maybe_unary(&self) -> ParsingResult {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                TokenType::Minus | TokenType::Not | TokenType::Hash | TokenType::Tilde => {
                    self.advance_cursor();
                    let right = self.parse_maybe_expression()?;
                    match right {
                        Some(right) => Ok(Some(Expression::Unary(token, Box::new(right)))),
                        None => Err(String::from("Failed to parse operand of unary expression")),
                    }
                }
                _ => self.parse_maybe_literal(),
            };
        }

        Ok(None)
    }

    fn parse_maybe_literal(&self) -> ParsingResult {
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
    fn should_parse_conditionals() {
        expect_source_to_equal_ast(
            "true or (false and true) and true",
            "[or l=`true` r=[and l=([and l=`false` r=`true`]) r=`true`]]",
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
            "[and l=`false` r=[or l=[>= l=`5` r=`5`] r=[< l=`11` r=`10`]]]",
        );
    }
}

