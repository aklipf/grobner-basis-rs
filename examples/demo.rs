use std::str::FromStr;

use grobner_basis::{
    order::{GradLex, Lex},
    polynomial::{buchberger, Polynomial},
    variable::Var,
};
use std::time::Instant;

fn main() {
    let input: Vec<Polynomial<i32, Var, Lex>> = vec![
        Polynomial::from_str("x^2+-y").unwrap(),
        Polynomial::from_str("x^3+-z").unwrap(),
    ];
    let start = Instant::now();
    let grobner_basis = buchberger(&input);
    let duration = start.elapsed();

    println!("grobner_basis:");
    for g_i in grobner_basis {
        println!("{g_i}");
    }
    println!("Time elapsed in buchberger() is: {:?}", duration);

    let input: Vec<Polynomial<i32, Var, GradLex>> = vec![
        Polynomial::from_str("xy^3+-x^2").unwrap(),
        Polynomial::from_str("x^3y^2+-y").unwrap(),
    ];
    let start = Instant::now();
    let grobner_basis = buchberger(&input);
    let duration = start.elapsed();

    println!("grobner_basis:");
    for g_i in grobner_basis {
        println!("{g_i}");
    }
    println!("Time elapsed in buchberger() is: {:?}", duration);
}
