use crate::Expression;

#[derive(Debug)]
pub enum Statement {
    Let(Let),
    Assign(Assign),
    While(While),
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

#[derive(Debug)]
pub enum Mutable {
    Yes,
    No,
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
