use std::iter::Peekable;

use gleezy_lex::{Lex, Token, TokenKind};

use crate::Statement;

pub type Source<'a> = Peekable<Lex<'a>>;

pub struct Parse<'a> {
    source: Source<'a>,
}

impl<'a> Parse<'a> {
    pub fn new(source: Source<'a>) -> Self {
        Self { source }
    }

    pub fn next(&mut self) -> Token {
        self.source.next().expect("expected token")
    }

    pub fn peek(&mut self) -> Option<Token> {
        self.source.peek().cloned()
    }

    pub fn eat(&mut self, kind: TokenKind) {
        let next: TokenKind = self.next().into();
        if next != kind {
            panic!("expected {kind:?} but got {next:?}");
        }
    }
}

impl Iterator for Parse<'_> {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        self.peek()?;
        let statement = Statement::parse(self);
        Some(statement)
    }
}

pub trait Parsable {
    fn parse(source: &mut Parse) -> Self;
}
