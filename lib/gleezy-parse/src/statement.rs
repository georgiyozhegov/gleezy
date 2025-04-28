use gleezy_lex::TokenKind;

use crate::{Expression, Parsable, Parse};

#[derive(Debug)]
pub enum Statement {
    Let(Let),
}

impl Parsable for Statement {
    fn parse(source: &mut Parse) -> Self {
        match source.peek().expect("expected statement").kind() {
            TokenKind::Let => Self::Let(Let::parse(source)),
            _ => panic!("expected statement"),
        }
    }
}

#[derive(Debug)]
pub struct Let {
    name: Identifier,
    value: Expression,
}

impl Parsable for Let {
    fn parse(source: &mut Parse) -> Self {
        source.eat(TokenKind::Let);
        let name = Identifier::parse(source);
        source.eat(TokenKind::Equal);
        let value = Expression::parse(source);
        Self { name, value }
    }
}

#[derive(Debug)]
pub struct Identifier {
    name: String,
}

impl Parsable for Identifier {
    fn parse(source: &mut Parse) -> Self {
        match source.next().into() {
            TokenKind::Identifier(name) => Self { name },
            _ => panic!("expected identifier"),
        }
    }
}
