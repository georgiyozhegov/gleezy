use gleezy_ast::{Assign, Body, Expression, Identifier, Let, Mutable, Statement, While};
use gleezy_lex::TokenKind;
use gleezy_report::{Report, Result};

use crate::{Parsable, Parse};

impl Parsable for Statement {
    fn parse(source: &mut Parse) -> Result<Self> {
        match source.peek()?.and_then(|token| Some(token.into_kind())) {
            Some(TokenKind::Let) => Ok(Self::Let(Let::parse(source)?)),
            Some(TokenKind::Identifier(..)) => Ok(Self::Assign(Assign::parse(source)?)),
            Some(TokenKind::While) => Ok(Self::While(While::parse(source)?)),
            _ => {
                let report = Report::UnexpectedToken { message: "expected statement" };
                Err(report)
            }
        }
    }
}

impl Parsable for Let {
    fn parse(source: &mut Parse) -> Result<Self> {
        source.eat(TokenKind::Let)?;
        let mutable = Mutable::parse(source)?;
        let assign = Assign::parse(source)?;
        Ok(Self::new(mutable, assign))
    }
}

impl Parsable for Mutable {
    fn parse(source: &mut Parse) -> Result<Self> {
        match source.peek()?.and_then(|token| Some(token.into_kind())) {
            Some(TokenKind::Mutable) => {
                source.next()?;
                Ok(Self::Yes)
            }
            _ => Ok(Self::No),
        }
    }
}

impl Parsable for Assign {
    fn parse(source: &mut Parse) -> Result<Self> {
        let name = Identifier::parse(source)?;
        source.eat(TokenKind::Equal)?;
        let value = Expression::parse(source)?;
        Ok(Self::new(name, value))
    }
}

impl Parsable for Identifier {
    fn parse(source: &mut Parse) -> Result<Self> {
        match source.next()?.into_kind() {
            TokenKind::Identifier(name) => Ok(Self::new(name)),
            _ => {
                let report = Report::UnexpectedToken { message: "expected identifier" };
                Err(report)
            }
        }
    }
}

impl Parsable for While {
    fn parse(source: &mut Parse) -> Result<Self> {
        source.eat(TokenKind::While)?;
        let condition = Expression::parse(source)?;
        source.eat(TokenKind::Do)?;
        let body = Body::parse(source)?;
        source.eat(TokenKind::End)?;
        Ok(Self::new(condition, body))
    }
}

impl Parsable for Body {
    fn parse(source: &mut Parse) -> Result<Self> {
        let mut inner = Vec::new();

        while source.peek()?.is_some_and(|token| token.kind() != &TokenKind::End) {
            let statement = Statement::parse(source)?;
            inner.push(statement);
        }

        Ok(Self::new(inner))
    }
}
