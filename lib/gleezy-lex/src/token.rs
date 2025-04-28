use crate::Span;

#[derive(Clone, Debug)]
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

    pub fn into_kind(self) -> TokenKind {
        self.kind
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn into_span(self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Integer(i128),
    Identifier(String),
    String(String),
    True,
    False,
    Let,
    Mutable,
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
