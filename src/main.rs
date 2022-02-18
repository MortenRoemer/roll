use crate::die::Expression;
use std::env;

mod die;

fn main() {
    let expr = read_expr();
    let expr = Expression::parse(&expr);
    println!("{}", expr.evaluate());
}

fn read_expr() -> String {
    let mut expr = String::new();

    for arg in env::args().skip(1) {
        expr.push_str(&arg);
    }

    expr
}
