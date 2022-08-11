use super::{Expression, Identifier, Prefix};

#[derive(Debug)]
pub enum Variable<'a> {
    Identifier(Identifier<'a>),
    MemberAccess {
        reference: Box<Prefix<'a>>,
        member: Identifier<'a>,
    },
    ExpressionMemberAccess {
        reference: Box<Prefix<'a>>,
        member: Box<Expression<'a>>,
    },
}

pub trait VariableVisitor<T> {
    fn visit_variable_identifier(&self, identifier: &Identifier) -> T;
    fn visit_variable_member_access(&self, reference: &Box<Prefix>, member: &Identifier) -> T;
    fn visit_variable_expression_member_access(
        &self,
        reference: &Box<Prefix>,
        member: &Box<Expression>,
    ) -> T;
}

impl<'a> Variable<'a> {
    pub fn visit<T, V>(&self, visitor: &V) -> T
    where
        V: VariableVisitor<T> + ?Sized,
    {
        match self {
            Variable::Identifier(identifier) => visitor.visit_variable_identifier(identifier),
            Variable::MemberAccess { reference, member } => {
                visitor.visit_variable_member_access(reference, member)
            }
            Variable::ExpressionMemberAccess { reference, member } => {
                visitor.visit_variable_expression_member_access(reference, member)
            }
        }
    }
}
