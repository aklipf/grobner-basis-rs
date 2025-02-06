use std::{cmp::Ordering, str::FromStr};

use grobner_basis::{
    order::{GradLex, Lex, Order},
    polynomial::{buchberger, HeadMonomial, Polynomial},
    variable::Var,
};
use std::time::Instant;

fn main() {
    let f: Polynomial<i32, Var> = Polynomial::from_str("x^2+-3xy+2x^2y^3+y^2+2").unwrap();
    let g: Polynomial<i32, Var> = Polynomial::from_str("x+xy+x^2y+x^2+1").unwrap();

    println!("f={f}");
    println!("g={g}");

    let (q, r) = f / &g;

    println!("{}", q.clone() * g.clone() + r.clone());

    println!("q={q}");
    println!("r={r}");

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
