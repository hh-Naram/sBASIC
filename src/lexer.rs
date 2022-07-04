use crate::token;

pub struct Instruction {
    pub line_number: u32,
    pub tokens: Vec<(u32, token::Token)>,
}
