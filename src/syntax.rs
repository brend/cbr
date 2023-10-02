use std::collections::VecDeque;
use crate::tokens::Token;

#[derive(Debug)]
pub enum Op {
    Plus, Minus,
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Binary(Op, Box<Expr>, Box<Expr>),
}

fn parse_term(tokens: &mut VecDeque<Token>) -> Expr {
    match tokens.pop_front().unwrap() {
        Token::Number(n) => Expr::Number(n),
        t => panic!("expected term, found {:?}", t)
    }
}

fn parse_expr(tokens: &mut VecDeque<Token>) -> Expr {
    let mut e = parse_term(tokens);

    loop {
        match tokens.pop_front() {
            Some(Token::Plus) => {
                let r = parse_term(tokens);
                e = Expr::Binary(Op::Plus, Box::new(e), Box::new(r));
            },
            Some(Token::Minus) => {
                let r = parse_term(tokens);
                e = Expr::Binary(Op::Minus, Box::new(e), Box::new(r));
            },
            None => break,
            Some(t) => {
                tokens.push_front(t);
                break;
            }
        }
    }

    e
}

pub fn parse(tokens: &mut VecDeque<Token>) -> Expr {
    parse_expr(tokens)
}
