use crate::{
    ast::expression::Expression,
    token::{Token, TokenType},
};
use std::cell::RefCell;

type ParsingResult<'e> = Result<Option<Expression<'e>>, String>;

pub struct Parser<'p> {
    tokens: &'p [Token<'p>],
    current: RefCell<usize>,
}

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

    fn parse_maybe_expression(&self) -> ParsingResult {
        self.parse_maybe_spread()
    }

    fn parse_maybe_spread(&self) -> ParsingResult {
        self.parse_maybe_function_definition()
    }

    fn parse_maybe_function_definition(&self) -> ParsingResult {
        self.parse_maybe_prefix()
    }

    fn parse_maybe_prefix(&self) -> ParsingResult {
        if let Some(token) = self.get_token() {
            return match token.token_type {
                // TODO: var
                // TODO: functioncall
                TokenType::LeftParen => {
                    self.advance_cursor();
                    let expression = self.parse_maybe_expression()?;
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

    fn parse_maybe_table_constructor(&self) -> ParsingResult {
        self.parse_maybe_binary()
    }

    fn parse_maybe_binary(&self) -> ParsingResult {
        let left = self.parse_maybe_unary()?;
        if left.is_none() {
            return Ok(None);
        }

        if let Some(token) = self.get_token() {
            return match token.token_type { // & ~ | .. << >> and or
                TokenType::Or
                | TokenType::And
                | TokenType::Less
                | TokenType::LessEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::TildeEqual
                | TokenType::EqualEqual
                | TokenType::LessLess
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
                    let right = self.parse_maybe_expression()?;
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

// 3.4.8 – Precedence

// Operator precedence in Lua follows the table below, from lower to higher priority:

//      or
//      and
//      <     >     <=    >=    ~=    ==
//      |
//      ~
//      &
//      <<    >>
//      ..
//      +     -
//      *     /     //    %
//      unary operators (not   #     -     ~)
//      ^

// As usual, you can use parentheses to change the precedences of an expression. The concatenation ('..') and exponentiation ('^') operators are right associative. All other binary operators are left associative.
