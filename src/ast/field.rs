use super::{Expression, Identifier};

#[derive(Debug)]
pub enum Field<'a> {
    Expression {
        key: Expression<'a>,
        value: Expression<'a>,
    },
    Normal {
        key: Identifier<'a>,
        value: Expression<'a>,
    },
    Anonymous {
        value: Expression<'a>,
    },
}

pub trait FieldVisitor<T> {
    fn visit_field_expression(&self, key: &Expression, value: &Expression) -> T;
    fn visit_field_normal(&self, key: &Identifier, value: &Expression) -> T;
    fn visit_field_anonymous(&self, value: &Expression) -> T;
}

impl<'a> Field<'a> {
    pub fn visit<T, V>(&self, visitor: &V) -> T
    where
        V: FieldVisitor<T> + ?Sized,
    {
        match self {
            Field::Expression { key, value } => visitor.visit_field_expression(key, value),
            Field::Normal { key, value } => visitor.visit_field_normal(key, value),
            Field::Anonymous { value } => visitor.visit_field_anonymous(value),
        }
    }
}
