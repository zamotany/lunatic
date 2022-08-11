use crate::{ast::Expression, token::Token};
use std::cell::RefCell;

pub type ParsingResult<T> = Result<Option<T>, String>;

/// Lua parser.
///
/// Specs:
/// - https://www.lua.org/manual/5.4/manual.html#9
/// - http://www.lua.org/manual/5.4/manual.html#3.4.8
pub struct Parser<'p> {
    pub(super) tokens: &'p [Token<'p>],
    pub(super) current: RefCell<usize>,
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
