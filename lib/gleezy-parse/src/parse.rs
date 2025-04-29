use std::iter::Peekable;

use gleezy_ast::Statement;
use gleezy_lex::{Lex, Token, TokenKind};
use gleezy_report::{Report, Result};

pub type Source<'a> = Peekable<Lex<'a>>;

pub struct Parse<'a> {
    source: Source<'a>,
}

impl<'a> Parse<'a> {
    pub fn new(source: Source<'a>) -> Self {
        Self { source }
    }

    pub fn next(&mut self) -> Result<Token> {
        self.source.next().unwrap_or({
            let report = Report::UnexpectedToken { message: "unexpected EOF" };
            Err(report)
        })
    }

    pub fn peek(&mut self) -> Result<Option<Token>> {
        self.source.peek().cloned().transpose()
    }

    pub fn eat(&mut self, kind: TokenKind) -> Result<()> {
        let next = self.next()?.into_kind();
        if next != kind {
            let report = Report::UnexpectedToken { message: "unexpected token" };
            Err(report)
        } else {
            Ok(())
        }
    }
}

impl Iterator for Parse<'_> {
    type Item = Result<Statement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.peek()?;
        let statement = Statement::parse(self);
        Some(statement)
    }
}

pub trait Parsable {
    fn parse(source: &mut Parse) -> Result<Self> where Self: Sized;
}
