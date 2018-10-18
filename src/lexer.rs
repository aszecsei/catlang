use token;

pub struct Lexeme {
    token: token::Token,
    literal: String,
    position: token::Pos,
}
