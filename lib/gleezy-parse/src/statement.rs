use gleezy_lex::TokenKind;

use crate::{Expression, Parsable, Parse};

#[derive(Debug)]
pub enum Statement {
    Let(Let),
    Assign(Assign),
    While(While),
}

impl Parsable for Statement {
    fn parse(source: &mut Parse) -> Self {
        match source.peek().and_then(|token| Some(token.into_kind())) {
            Some(TokenKind::Let) => Self::Let(Let::parse(source)),
            Some(TokenKind::Identifier(..)) => Self::Assign(Assign::parse(source)),
            Some(TokenKind::While) => Self::While(While::parse(source)),
            kind => panic!("expected statement: {kind:?}"),
        }
    }
}

#[derive(Debug)]
pub struct Let {
    mutable: Mutable,
    assign: Assign,
}

impl Let {
    pub fn new(mutable: Mutable, assign: Assign) -> Self {
        Self { mutable, assign }
    }

    pub fn mutable(&self) -> &Mutable {
        &self.mutable
    }

    pub fn into_mutable(self) -> Mutable {
        self.mutable
    }

    pub fn assign(&self) -> &Assign {
        &self.assign
    }

    pub fn into_assign(self) -> Assign {
        self.assign
    }
}

impl Parsable for Let {
    fn parse(source: &mut Parse) -> Self {
        source.eat(TokenKind::Let);
        let mutable = Mutable::parse(source);
        let assign = Assign::parse(source);
        Self::new(mutable, assign)
    }
}

#[derive(Debug)]
pub enum Mutable {
    Yes,
    No,
}

impl Parsable for Mutable {
    fn parse(source: &mut Parse) -> Self {
        match source.peek().and_then(|token| Some(token.into_kind())) {
            Some(TokenKind::Mutable) => {
                source.next();
                Self::Yes
            }
            _ => Self::No,
        }
    }
}

#[derive(Debug)]
pub struct Assign {
    name: Identifier,
    value: Expression,
}

impl Assign {
    pub fn new(name: Identifier, value: Expression) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn into_name(self) -> Identifier {
        self.name
    }

    pub fn value(&self) -> &Expression {
        &self.value
    }

    pub fn into_value(self) -> Expression {
        self.value
    }
}

impl Parsable for Assign {
    fn parse(source: &mut Parse) -> Self {
        let name = Identifier::parse(source);
        source.eat(TokenKind::Equal);
        let value = Expression::parse(source);
        Self::new(name, value)
    }
}

#[derive(Debug)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn into_name(self) -> String {
        self.name
    }
}

impl Parsable for Identifier {
    fn parse(source: &mut Parse) -> Self {
        match source.next().into_kind() {
            TokenKind::Identifier(name) => Self::new(name),
            kind => panic!("expected identifier: {kind:?}"),
        }
    }
}

#[derive(Debug)]
pub struct While {
    condition: Expression,
    body: Body,
}

impl While {
    pub fn new(condition: Expression, body: Body) -> Self {
        Self { condition, body }
    }

    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    pub fn into_condition(self) -> Expression {
        self.condition
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn into_body(self) -> Body {
        self.body
    }
}

impl Parsable for While {
    fn parse(source: &mut Parse) -> Self {
        source.eat(TokenKind::While);
        let condition = Expression::parse(source);
        source.eat(TokenKind::Do);
        let body = Body::parse(source);
        source.eat(TokenKind::End);
        Self::new(condition, body)
    }
}

#[derive(Debug)]
pub struct Body {
    inner: Vec<Statement>,
}

impl Body {
    pub fn new(inner: Vec<Statement>) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &[Statement] {
        &self.inner
    }

    pub fn into_inner(self) -> Vec<Statement> {
        self.inner
    }
}

impl Parsable for Body {
    fn parse(source: &mut Parse) -> Self {
        let mut inner = Vec::new();

        while source.peek().is_some_and(|token| token.kind() != &TokenKind::End) {
            let statement = Statement::parse(source);
            inner.push(statement);
        }

        Self::new(inner)
    }
}
