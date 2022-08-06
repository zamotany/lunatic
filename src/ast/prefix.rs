use super::{expression::Expression, variable::Variable};

#[derive(Debug)]
pub enum Prefix<'a> {
    Variable(Variable<'a>),
    // FunctionCall(),
    Group(Box<Expression<'a>>),
}
