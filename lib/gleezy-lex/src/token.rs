#[derive(Debug)]
pub enum Token {
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
