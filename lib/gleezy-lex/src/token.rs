#[derive(Debug)]
pub enum Token {
    Dummy,
    Integer(i128),
    Plus,
    Minus,
    Star,
    Slash,
}
