use super::{identifier::Identifier, expression::Expression, prefix::Prefix};

#[derive(Debug)]
pub enum Variable<'a> {
    Identifier(Identifier<'a>),
    ExpressionAccess(Box<Prefix<'a>>, Box<Expression<'a>>),
    MemberAccess(Box<Prefix<'a>>, Identifier<'a>),
}
