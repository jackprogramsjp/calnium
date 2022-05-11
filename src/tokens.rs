#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Lparen,
    Rparen,
}
