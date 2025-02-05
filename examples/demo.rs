use std::str::FromStr;

use grobner_basis::{order::Var, polynomial::Polynomial};

fn main() {
    let f: Polynomial<i32, Var> = Polynomial::from_str("x^2+-3xy+2x^2y^3+y^2+2").unwrap();
    let g: Polynomial<i32, Var> = Polynomial::from_str("x+xy+x^2y+x^2+1").unwrap();

    println!("{f}");
    println!("{g}");

    println!("{}", g * f)
}
