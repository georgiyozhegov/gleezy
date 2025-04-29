use std::{iter::Peekable, str::Chars};

use gleezy_report::{Report, Result, Span};

use crate::{Token, TokenKind};

macro_rules! numeric {
    () => {
        '0'..='9'
    };
}

macro_rules! alphabetic {
    () => {
        'a'..='z' | 'A'..='Z'
    };
}

macro_rules! skip {
    () => {
        ' ' | '\t' | '\n'
    };
}

pub type Source<'a> = Peekable<Chars<'a>>;

pub struct Lex<'a> {
    source: Source<'a>,
    span: Span,
}

impl<'a> Lex<'a> {
    pub fn new(source: Source<'a>) -> Self {
        Self {
            source,
            span: Span::new(1, 1),
        }
    }
}

impl Iterator for Lex<'_> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_while(|c| matches!(c, skip!()));

        let span = self.span.clone();
        let kind = match self.source.peek()? {
            numeric!() => self.numeric(),
            alphabetic!() => self.alphabetic(),
            '"' => self.string(),
            '=' => self.one(TokenKind::Equal),
            '+' => self.one(TokenKind::Plus),
            '-' => self.one(TokenKind::Minus),
            '*' => self.one(TokenKind::Star),
            '/' => self.one(TokenKind::Slash),
            '>' => self.one(TokenKind::Greater),
            '<' => self.one(TokenKind::Less),
            '?' => self.one(TokenKind::Question),
            '!' => self.one(TokenKind::Not),
            '(' => self.one(TokenKind::OpenParenthesis),
            ')' => self.one(TokenKind::CloseParenthesis),
            c => {
                let report = Report::UnknownCharacter { c: *c, span };
                return Some(Err(report));
            }
        };

        Some(Ok(Token::new(kind, span)))
    }
}

impl Lex<'_> {
    fn next(&mut self) -> Option<char> {
        let c = self.source.next()?;
        self.span.update(c);
        Some(c)
    }

    fn take_while(&mut self, predicate: fn(&char) -> bool) -> String {
        let mut output = String::new();

        while self.source.peek().is_some_and(|c| predicate(c)) {
            output.push(self.next().unwrap());
        }

        output
    }

    fn numeric(&mut self) -> TokenKind {
        let string = self.take_while(|c| matches!(c, numeric!() | '_'));
        let string = string.replace('_', "");
        let value = string.parse().unwrap();
        TokenKind::Integer(value)
    }

    fn alphabetic(&mut self) -> TokenKind {
        let string = self.take_while(|c| matches!(c, alphabetic!() | numeric!() | '_'));
        match string.as_str() {
            "let" => TokenKind::Let,
            "mutable" => TokenKind::Mutable,
            "if" => TokenKind::If,
            "or" => TokenKind::Or,
            "else" => TokenKind::Else,
            "then" => TokenKind::Then,
            "while" => TokenKind::While,
            "do" => TokenKind::Do,
            "end" => TokenKind::End,
            _ => TokenKind::Identifier(string),
        }
    }

    fn string(&mut self) -> TokenKind {
        self.next();
        let string = self.take_while(|c| *c != '"');
        self.next();
        TokenKind::String(string)
    }

    fn one(&mut self, kind: TokenKind) -> TokenKind {
        self.next();
        kind
    }
}
