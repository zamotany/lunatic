use super::{Field, FieldVisitor};

#[derive(Debug)]
pub struct TableConstructor<'a> {
    pub fields: Vec<Field<'a>>,
}

pub trait TableConstructorVisitor<T>: FieldVisitor<T> {
    fn visit_fields(&self, fields: &Vec<Field>) -> T;
}

impl<'a> TableConstructor<'a> {
    pub fn visit<T, V>(&self, visitor: &V) -> T
    where
        V: TableConstructorVisitor<T> + ?Sized,
    {
        visitor.visit_fields(&self.fields)
    }
}
