mod binary;
mod core;
mod prefix;
mod table_constructor;
mod unary;
mod variable;

use crate::{
    ast::Expression,
    parser::{Parser, ParsingResult},
};

impl<'p> Parser<'p> {
    pub(super) fn parse_maybe_expression(&self) -> ParsingResult<Expression> {
        self.parse_maybe_spread()
    }
}
