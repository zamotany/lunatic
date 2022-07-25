use crate::{
    ast::expression::Expression,
    token::{Token, TokenType},
};

type ParsingResult<'e> = Result<Option<Expression<'e>>, String>;

pub struct Parser<'p> {
    tokens: &'p [Token<'p>],
    current: usize,
}

impl<'p> Parser<'p> {
    pub fn new(tokens: &'p [Token<'p>]) -> Parser<'p> {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParsingResult {
        self.parse_maybe_expression()
    }
}

impl<'p> Parser<'p> {
    fn parse_maybe_expression(&mut self) -> ParsingResult {
        let token = &self.tokens[self.current];
        match token.token_type {
            TokenType::False
            | TokenType::True
            | TokenType::Nil
            | TokenType::Numeral
            | TokenType::LiteralString => {
                self.current += 1;
                Ok(Some(Expression::Literal(token)))
            }
            _ => self.parse_maybe_spread(),
        }
    }

    fn parse_maybe_spread(&mut self) -> ParsingResult {
        self.parse_maybe_function_definition()
    }

    fn parse_maybe_function_definition(&mut self) -> ParsingResult {
        self.parse_maybe_prefix()
    }

    fn parse_maybe_prefix(&mut self) -> ParsingResult {
        let token = &self.tokens[self.current];
        match token.token_type {
            // TODO: var
            // TODO: functioncall
            TokenType::LeftParen => {
                self.current += 1;
                let expression = self.parse_maybe_expression()?;
                match expression {
                    Some(expression) => Ok(Some(Expression::Group(Box::new(expression)))),
                    None => Err(String::from("Expected ')' after expression")),
                }
            }
            _ => self.parse_maybe_table_constructor(),
        }
    }

    fn parse_maybe_table_constructor(&mut self) -> ParsingResult {
        self.parse_maybe_binary()
    }

    fn parse_maybe_binary(&mut self) -> ParsingResult {
        self.parse_maybe_unary()
    }

    fn parse_maybe_unary(&mut self) -> ParsingResult {
        let token = &self.tokens[self.current];
        match token.token_type {
            TokenType::Minus | TokenType::Not | TokenType::Hash | TokenType::Tilde => {
                self.current += 1;
                let right = self.parse_maybe_expression()?;
                match right {
                    Some(right) => Ok(Some(Expression::Unary(token, Box::new(right)))),
                    None => Err(String::from("Failed to parse operand of unary expression")),
                }
            }
            _ => Ok(None),
        }
    }
}
