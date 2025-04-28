use gleezy_lex::TokenKind;

use crate::{Parsable, Parse};

#[derive(Debug)]
pub enum Expression {
    Integer(Integer),
}

impl Parsable for Expression {
    fn parse(source: &mut Parse) -> Self {
        match source.peek().and_then(|token| Some(token.into_kind())) {
            Some(TokenKind::Integer(..)) => Self::Integer(Integer::parse(source)),
            kind => panic!("expected expression: {kind:?}"),
        }
    }
}

#[derive(Debug)]
pub struct Integer {
    value: i128,
}

impl Integer {
    pub fn new(value: i128) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &i128 {
        &self.value
    }

    pub fn into_value(&self) -> i128 {
        self.value
    }
}

impl Parsable for Integer {
    fn parse(source: &mut crate::Parse) -> Self {
        match source.next().into_kind() {
            TokenKind::Integer(value) => Self { value },
            kind => panic!("expected integer: {kind:?}"),
        }
    }
}
