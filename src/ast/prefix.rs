use super::{Expression, Variable, VariableVisitor};

#[derive(Debug)]
pub enum Prefix<'a> {
    Variable(Variable<'a>),
    // FunctionCall(),
    Group(Box<Expression<'a>>),
}

pub trait PrefixVisitor<T>: VariableVisitor<T> {
    fn visit_prefix_variable(&self, variable: &Variable) -> T {
        variable.visit(self)
    }
    fn visit_prefix_group(&self, expression: &Box<Expression>) -> T;
}

impl<'a> Prefix<'a> {
    pub fn visit<T, V>(&self, visitor: &V) -> T
    where
        V: PrefixVisitor<T> + ?Sized,
    {
        match self {
            Prefix::Variable(variable) => visitor.visit_prefix_variable(variable),
            Prefix::Group(expression) => visitor.visit_prefix_group(expression),
        }
    }
}
