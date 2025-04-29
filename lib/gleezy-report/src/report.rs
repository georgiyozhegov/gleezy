use crate::Span;

pub type Result<T> = std::result::Result<T, Report>;

#[derive(Debug, Clone)]
pub enum Report {
    UnknownCharacter {
        c: char,
        span: Span,
    },
    UnexpectedToken {
        message: &'static str,
    },
}
