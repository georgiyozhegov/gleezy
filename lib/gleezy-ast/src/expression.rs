use gleezy_lex::TokenKind;

use crate::Identifier;

#[derive(Debug)]
pub enum Expression {
    Integer(Integer),
    Identifier(Identifier),
    Str(Str),
    Boolean(Boolean),
    Binary(Binary),
    Unary(Unary),
}

#[derive(Debug)]
pub struct Integer {
    value: i128,
}

impl Integer {
    pub fn new(value: i128) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &i128 {
        &self.value
    }

    pub fn into_value(&self) -> i128 {
        self.value
    }
}

#[derive(Debug)]
pub struct Str {
    value: String,
}

impl Str {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn into_value(self) -> String {
        self.value
    }
}

#[derive(Debug)]
pub enum Boolean {
    True,
    False,
}

#[derive(Debug)]
pub struct Binary {
    kind: BinaryKind,
    left: Box<Expression>,
    right: Box<Expression>,
}

impl Binary {
    pub fn new(kind: BinaryKind, left: Box<Expression>, right: Box<Expression>) -> Self {
        Self { kind, left, right }
    }

    pub fn kind(&self) -> &BinaryKind {
        &self.kind
    }

    pub fn into_kind(self) -> BinaryKind {
        self.kind
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }

    pub fn into_left(self) -> Expression {
        *self.left
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }

    pub fn into_right(self) -> Expression {
        *self.right
    }
}

#[derive(Debug)]
pub enum BinaryKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    Greater,
    Less,
}

#[derive(Debug)]
pub struct Unary {
    kind: UnaryKind,
    inner: Box<Expression>,
}

impl Unary {
    pub fn new(kind: UnaryKind, inner: Expression) -> Self {
        Self { kind, inner: Box::new(inner) }
    }

    pub fn kind(&self) -> &UnaryKind {
        &self.kind
    }

    pub fn into_kind(self) -> UnaryKind {
        self.kind
    }

    pub fn inner(&self) -> &Expression {
        &self.inner
    }

    pub fn into_inner(self) -> Expression {
        *self.inner
    }
}

#[derive(Debug)]
pub enum UnaryKind {
    Not,
    Negate,
}
