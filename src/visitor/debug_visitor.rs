use crate::{
    ast::{
        expression::{Expression, ExpressionVisitor},
        field::Field,
        prefix::Prefix,
        table_constructor::TableConstructor,
        variable::Variable,
    },
    token::Token,
};

pub struct DebugVisitor {}

impl DebugVisitor {
    pub fn new() -> DebugVisitor {
        DebugVisitor {}
    }
}

impl ExpressionVisitor for DebugVisitor {
    type Output = String;

    fn visit_literal(&self, token: &Token) -> String {
        format!("`{}`", token.lexeme)
    }

    fn visit_unary(&self, operator: &Token, right: &Box<Expression>) -> String {
        format!("[{} r={}]", operator.lexeme, right.visit(self))
    }

    fn visit_binary(
        &self,
        left: &Box<Expression>,
        operator: &Token,
        right: &Box<Expression>,
    ) -> String {
        format!(
            "[{} l={} r={}]",
            operator.lexeme,
            left.visit(self),
            right.visit(self)
        )
    }

    fn visit_table_constructor(&self, table_constructor: &TableConstructor) -> String {
        let mut fields_string = String::new();
        for field in table_constructor.fields.iter() {
            let field_string = match field {
                Field::Anonymous(value) => format!("?={} ", value.visit(self)),
                Field::Expression(key, value) => {
                    format!("{}={} ", key.visit(self), value.visit(self))
                }
                Field::Normal(key, value) => {
                    format!("`{}`={} ", key.token.lexeme, value.visit(self))
                }
            };
            fields_string.push_str(&field_string[..]);
        }
        format!("Tc[{}]", fields_string)
    }

    fn visit_prefix(&self, prefix: &Prefix) -> String {
        match prefix {
            Prefix::Group(expression) => format!("({})", expression.visit(self)),
            Prefix::Variable(variable) => match variable {
                Variable::Identifier(identifier) => format!("{}", identifier.token.lexeme),
                Variable::MemberAccess(prefix, identifier) => {
                    format!("{}.{}", self.visit_prefix(prefix), identifier.token.lexeme)
                }
                _ => String::from("TODO"),
            },
        }
    }
}
