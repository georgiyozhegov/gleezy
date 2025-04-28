use gleezy_lex::TokenKind;

use crate::{Expression, Parsable, Parse};

#[derive(Debug)]
pub enum Statement {
    Let(Let),
    Assign(Assign),
    While(While),
}

impl Parsable for Statement {
    fn parse(source: &mut Parse) -> Self {
        match source.peek().expect("expected statement").kind() {
            TokenKind::Let => Self::Let(Let::parse(source)),
            TokenKind::Identifier(..) => Self::Assign(Assign::parse(source)),
            TokenKind::While => Self::While(While::parse(source)),
            _ => panic!("expected statement"),
        }
    }
}

#[derive(Debug)]
pub struct Let {
    assign: Assign,
}

impl Parsable for Let {
    fn parse(source: &mut Parse) -> Self {
        let assign = Assign::parse(source);
        Self { assign }
    }
}

#[derive(Debug)]
pub struct Assign {
    name: Identifier,
    value: Expression,
}

impl Parsable for Assign {
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

#[derive(Debug)]
pub struct While {
    condition: Expression,
    body: Body,
}

impl Parsable for While {
    fn parse(source: &mut Parse) -> Self {
        source.eat(TokenKind::While);
        let condition = Expression::parse(source);
        source.eat(TokenKind::Do);
        let body = Body::parse(source);
        source.eat(TokenKind::End);
        Self { condition, body }
    }
}

#[derive(Debug)]
pub struct Body {
    inner: Vec<Statement>,
}

impl Parsable for Body {
    fn parse(source: &mut Parse) -> Self {
        let mut inner = Vec::new();

        while source.peek().is_some_and(|token| token.kind() != &TokenKind::End) {
            let statement = Statement::parse(source);
            inner.push(statement);
        }

        Self { inner }
    }
}
