use crate::{
    ast::{
        Expression, ExpressionVisitor, Field, FieldVisitor, Identifier, Prefix, PrefixVisitor,
        VariableVisitor,
    },
    token::Token,
};

pub struct DebugVisitor;

impl PrefixVisitor<String> for DebugVisitor {
    fn visit_prefix_group(&self, expression: &Box<Expression>) -> String {
        format!("({})", expression.visit(self))
    }
}

impl VariableVisitor<String> for DebugVisitor {
    fn visit_variable_identifier(&self, identifier: &Identifier) -> String {
        format!("{}", identifier.0.lexeme)
    }

    fn visit_variable_member_access(&self, reference: &Box<Prefix>, member: &Identifier) -> String {
        format!("{}.{}", reference.visit(self), member.0.lexeme)
    }

    fn visit_variable_expression_member_access(
        &self,
        reference: &Box<Prefix>,
        member: &Box<Expression>,
    ) -> String {
        format!("{}[{}]", reference.visit(self), member.visit(self))
    }
}

impl FieldVisitor<String> for DebugVisitor {
    fn visit_field_expression(&self, key: &Expression, value: &Expression) -> String {
        format!("{}={} ", key.visit(self), value.visit(self))
    }

    fn visit_field_normal(&self, key: &Identifier, value: &Expression) -> String {
        format!("`{}`={} ", key.0.lexeme, value.visit(self))
    }

    fn visit_field_anonymous(&self, value: &Expression) -> String {
        format!("?={} ", value.visit(self))
    }
}

impl ExpressionVisitor<String> for DebugVisitor {
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

    fn visit_table_constructor(&self, fields: &Vec<Field>) -> String {
        let mut fields_string = String::new();
        for field in fields.iter() {
            fields_string.push_str(&field.visit(self)[..]);
        }
        format!("Tc[{}]", fields_string)
    }
}
