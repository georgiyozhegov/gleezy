#[derive(Debug)]
pub enum Token {
    Dummy,
    Integer(i128),
    Identifier(String),
    String(String),
    Let,
    Plus,
    Minus,
    Star,
    Slash,
}
