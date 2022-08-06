use crate::{
    ast::{
        expression::{Expression, ExpressionVisitor},
        identifier::Identifier,
        table_constructor::TableConstructor,
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

    fn visit_group(&self, expression: &Box<Expression>) -> String {
        format!("({})", expression.visit(self))
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
            let key_str = match field.key {
                Identifier::Anonymous => "?",
                Identifier::Named(name) => name.lexeme,
            };
            fields_string.push_str(&format!("`{}`={} ", key_str, field.value.visit(self))[..]);
        }
        format!("Tc[{}]", fields_string)
    }
}
