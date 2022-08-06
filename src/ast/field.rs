use super::{expression::Expression, identifier::Identifier};

#[derive(Debug)]
pub enum Field<'a> {
    Expression(Expression<'a>, Expression<'a>),
    Normal(Identifier<'a>, Expression<'a>),
    Anonymous(Expression<'a>),
}
