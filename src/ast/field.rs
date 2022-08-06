use super::{expression::Expression, identifier::Identifier};

#[derive(Debug)]
pub struct Field<'a> {
    pub key: Identifier<'a>,
    pub value: Expression<'a>,
}

impl<'a> Field<'a> {
    pub fn new(key: Identifier<'a>, value: Expression<'a>) -> Field<'a> {
        Field { key, value }
    }
}
