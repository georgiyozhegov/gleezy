#[derive(Debug)]
pub enum Token {
    Dummy,
    Integer(i128),
    Identifier(String),
    Let,
    Plus,
    Minus,
    Star,
    Slash,
}
