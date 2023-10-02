mod tokens;
mod syntax;
mod interpreter;

use std::io::{Read};
use tokens::tokenize;
use syntax::parse;
use interpreter::eval;

fn main() {
    let mut input = String::new();
    //io::stdin().read_to_string(&mut input).unwrap();
    input = String::from("9 - 2 + 1");
    println!("input: {}", input);

    let mut tokens = tokenize(&input);

    //for t in &tokens {
    //    println!("{:?}", t);
    //}

    let expr = parse(&mut tokens);

    if !tokens.is_empty() {
        panic!("could not parse entire expression; remaining tokens: {:?}", tokens);
    }

    println!("expression: {:?}", expr);
    println!("result: {}", eval(expr));
}
