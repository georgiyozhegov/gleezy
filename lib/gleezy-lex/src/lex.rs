use std::{iter::Peekable, str::Chars};

use crate::Token;

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
}

impl<'a> Lex<'a> {
    pub fn new(source: Source<'a>) -> Self {
        Self { source }
    }
}

impl Iterator for Lex<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_while(|c| matches!(c, skip!()));

        let token = match self.source.peek()? {
            numeric!() => {
                let string = self.take_while(|c| matches!(c, numeric!() | '_'));
                let string = string.replace('_', "");
                let value = string.parse().unwrap();
                Token::Integer(value)
            }
            alphabetic!() => {
                let string = self.take_while(|c| matches!(c, alphabetic!() | numeric!() | '_'));
                match string.as_str() {
                    "let" => Token::Let,
                    _ => Token::Identifier(string),
                }
            }
            '"' => {
                self.source.next();
                let string = self.take_while(|c| *c != '"');
                self.source.next();
                Token::String(string)
            }
            '+' => self.one(Token::Plus),
            '-' => self.one(Token::Minus),
            '*' => self.one(Token::Star),
            '/' => self.one(Token::Slash),
            c => panic!("unknown character: {c}"),
        };

        Some(token)
    }
}

impl Lex<'_> {
    fn take_while(&mut self, predicate: fn(&char) -> bool) -> String {
        let mut output = String::new();

        while self.source.peek().is_some_and(|c| predicate(c)) {
            output.push(self.source.next().unwrap());
        }

        output
    }

    fn one(&mut self, token: Token) -> Token {
        self.source.next();
        token
    }
}
