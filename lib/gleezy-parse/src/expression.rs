use gleezy_lex::TokenKind;

use crate::{Parsable, Parse};

#[derive(Debug)]
pub enum Expression {
    Integer(Integer),
}

impl Parsable for Expression {
    fn parse(source: &mut Parse) -> Self {
        match source.peek().and_then(|token| Some(token.into())) {
            Some(TokenKind::Integer(..)) => Self::Integer(Integer::parse(source)),
            kind => panic!("expected expression: {kind:?}"),
        }
    }
}

#[derive(Debug)]
pub struct Integer {
    value: i128,
}

impl Parsable for Integer {
    fn parse(source: &mut crate::Parse) -> Self {
        match source.next().into() {
            TokenKind::Integer(value) => Self { value },
            kind => panic!("expected integer: {kind:?}"),
        }
    }
}
