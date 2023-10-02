use crate::syntax::{Expr, Op};

pub fn eval(expr: Expr) -> i64 {
    match expr {
        Expr::Number(i) => i,
        Expr::Binary(Op::Plus, left, right) => eval(*left) + eval(*right),
        Expr::Binary(Op::Minus, left, right) => eval(*left) - eval(*right)
    }
}
