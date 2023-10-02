use std::collections::VecDeque;

#[derive(Debug)]
pub enum Token {
    Number(i64),
    Id(String),
    Plus,
    Minus,
    LParen,
    RParen,
}

pub fn tokenize(input: &str) -> VecDeque<Token> {
    let mut tokens = VecDeque::new();

    for c in input.chars() {
        if c.is_whitespace() { continue; }

        let t = match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '0'..='9' => Token::Number(format!("{}", c).parse::<i64>().unwrap()),
            _ => panic!("unexpected character: {}", c)
        };

        tokens.push_back(t);
    }

    return tokens;
}