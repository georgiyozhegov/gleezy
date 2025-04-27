use crate::Span;

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Integer(i128),
    Identifier(String),
    String(String),
    True,
    False,
    Let,
    If,
    Or,
    Else,
    Then,
    While,
    Do,
    End,
    Equal,            // =
    Plus,             // +
    Minus,            // -
    Star,             // *
    Slash,            // /
    Greater,          // >
    Less,             // <
    Question,         // ?
    Not,              // !
    OpenParenthesis,  // (
    CloseParenthesis, // )
}
