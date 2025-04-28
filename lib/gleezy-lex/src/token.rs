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

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl Into<TokenKind> for Token {
    fn into(self) -> TokenKind {
        self.kind
    }
}

impl Into<Span> for Token {
    fn into(self) -> Span {
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
