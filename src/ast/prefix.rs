use super::{Expression, FunctionCall, FunctionCallVisitor, Variable, VariableVisitor};

#[derive(Debug)]
pub enum Prefix<'a> {
    Variable(Variable<'a>),
    FunctionCall(FunctionCall<'a>),
    Group(Box<Expression<'a>>),
}

pub trait PrefixVisitor<T>: VariableVisitor<T> + FunctionCallVisitor<T> {
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
            Prefix::FunctionCall(function_call) => function_call.visit(visitor),
        }
    }
}
