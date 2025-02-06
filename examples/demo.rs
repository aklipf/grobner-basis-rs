use std::str::FromStr;

use grobner_basis::{
    order::Var,
    polynomial::{buchberger, Polynomial},
};
use std::time::Instant;
fn main() {
    let f: Polynomial<i32, Var> = Polynomial::from_str("x^2+-3xy+2x^2y^3+y^2+2").unwrap();
    let g: Polynomial<i32, Var> = Polynomial::from_str("x+xy+x^2y+x^2+1").unwrap();

    println!("f={f}");
    println!("g={g}");

    let (q, r) = f.div_euclid(&g);

    println!("{}", q.clone() * g.clone() + r.clone());

    println!("q={q}");
    println!("r={r}");

    let input: Vec<Polynomial<i32, Var>> = vec![
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
}
