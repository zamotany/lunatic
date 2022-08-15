use crate::{
    ast::{
        Args, ArgsVisitor, Expression, ExpressionVisitor, Field, FieldVisitor, FunctionCallVisitor,
        Identifier, Prefix, PrefixVisitor, TableConstructor, TableConstructorVisitor,
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

impl FunctionCallVisitor<String> for DebugVisitor {
    fn visit_function_call(&self, callee: &Box<Prefix>, args: &Args) -> String {
        format!("[{} a:{}]", callee.visit(self), args.visit(self))
    }

    fn visit_method_call(&self, callee: &Box<Prefix>, method: &Identifier, args: &Args) -> String {
        format!(
            "[{}:{} a:{}]",
            callee.visit(self),
            method.0.lexeme,
            args.visit(self)
        )
    }
}

impl ArgsVisitor<String> for DebugVisitor {
    fn visit_args_expression_list(&self, expressions: &Vec<Expression>) -> String {
        let mut expressions_string = String::new();
        for expression in expressions.iter() {
            expressions_string.push_str(&expression.visit(self)[..]);
            expressions_string.push_str(", ")
        }
        format!("{}", expressions_string)
    }

    fn visit_args_table_constructor(&self, table_constructor: &TableConstructor) -> String {
        table_constructor.visit(self)
    }

    fn visit_args_literal_string(&self, token: &Token) -> String {
        self.visit_literal(token)
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

impl TableConstructorVisitor<String> for DebugVisitor {
    fn visit_fields(&self, fields: &Vec<Field>) -> String {
        let mut fields_string = String::new();
        for field in fields.iter() {
            fields_string.push_str(&field.visit(self)[..]);
        }
        format!("Tc[{}]", fields_string)
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
}
