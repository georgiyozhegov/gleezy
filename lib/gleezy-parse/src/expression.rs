use gleezy_ast::{BinaryKind, Boolean, Expression, Integer, Str, UnaryKind};
use gleezy_lex::TokenKind;
use gleezy_report::{Report, Result};

use crate::{Parsable, Parse};

impl Parsable for Expression {
    fn parse(source: &mut Parse) -> Result<Self> {
        match source.peek()?.and_then(|token| Some(token.into_kind())) {
            Some(TokenKind::Integer(..)) => {
                let expression = Self::Integer(Integer::parse(source)?);
                Ok(expression)
            }
            _ => {
                let report = Report::UnexpectedToken { message: "expected expression" };
                Err(report)
            }
        }
    }
}

impl Parsable for Integer {
    fn parse(source: &mut crate::Parse) -> Result<Self> {
        match source.next()?.into_kind() {
            TokenKind::Integer(value) => Ok(Self::new(value)),
            _ => {
                let report = Report::UnexpectedToken { message: "expected integer" };
                Err(report)
            }
        }
    }
}

impl Parsable for Str {
    fn parse(source: &mut Parse) -> Result<Self> {
        match source.next()?.into_kind() {
            TokenKind::String(value) => Ok(Self::new(value)),
            _ => {
                let report = Report::UnexpectedToken { message: "expected string" };
                Err(report)
            }
        }
    }
}

impl Parsable for Boolean {
    fn parse(source: &mut Parse) -> Result<Self> {
        match source.next()?.into_kind() {
            TokenKind::True => Ok(Self::True),
            TokenKind::False => Ok(Self::False),
            _ => {
                let report = Report::UnexpectedToken { message: "expected boolean" };
                Err(report)
            }
        }
    }
}

impl Parsable for BinaryKind {
    fn parse(source: &mut Parse) -> Result<Self> {
        match source.next()?.into_kind() {
            TokenKind::Plus => Ok(Self::Add),
            TokenKind::Minus => Ok(Self::Subtract),
            TokenKind::Star => Ok(Self::Multiply),
            TokenKind::Slash => Ok(Self::Divide),
            TokenKind::Question => Ok(Self::Equal),
            TokenKind::Greater => Ok(Self::Greater),
            TokenKind::Less => Ok(Self::Less),
            _ => {
                let report = Report::UnexpectedToken { message: "expected boolean" };
                Err(report)
            }
        }
    }
}

impl Parsable for UnaryKind {
    fn parse(source: &mut Parse) -> Result<Self> {
        match source.next()?.into_kind() {
            TokenKind::Not => Ok(Self::Not),
            TokenKind::Minus => Ok(Self::Negate),
            _ => {
                let report = Report::UnexpectedToken { message: "expected unary kind" };
                Err(report)
            }
        }
    }
}
