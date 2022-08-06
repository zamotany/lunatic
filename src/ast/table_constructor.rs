use super::{expression::Expression, field::Field};

#[derive(Debug)]
pub struct TableConstructor<'a> {
    pub fields: Vec<Field<'a>>,
}

impl<'a> TableConstructor<'a> {
    pub fn new(fields: Vec<Field<'a>>) -> TableConstructor<'a> {
        TableConstructor { fields }
    }

    pub fn new_expression(fields: Vec<Field<'a>>) -> Expression {
        Expression::TableConstructor(TableConstructor::new(fields))
    }
}
