use super::{Args, Identifier, Prefix};

#[derive(Debug)]
pub enum FunctionCall<'a> {
    FunctionCall {
        callee: Box<Prefix<'a>>,
        args: Args<'a>,
    },
    MethodCall {
        callee: Box<Prefix<'a>>,
        method: Identifier<'a>,
        args: Args<'a>,
    },
}

pub trait FunctionCallVisitor<T> {
    fn visit_function_call(&self, callee: &Box<Prefix>, args: &Args) -> T;
    fn visit_method_call(&self, callee: &Box<Prefix>, method: &Identifier, args: &Args) -> T;
}

impl<'a> FunctionCall<'a> {
    pub fn visit<T, V>(&self, visitor: &V) -> T
    where
        V: FunctionCallVisitor<T> + ?Sized,
    {
        match self {
            FunctionCall::FunctionCall { callee, args } => {
                visitor.visit_function_call(callee, args)
            }
            FunctionCall::MethodCall {
                callee,
                method,
                args,
            } => visitor.visit_method_call(callee, method, args),
        }
    }
}
