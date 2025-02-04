use std::ops::{Add, Div, Mul, Sub};

use super::term::{Degree, Term, Variable};

use super::ring::Ring;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial<R: Ring, V: Variable> {
    monomials: Vec<(Term<V>, R)>,
}

impl<R: Ring, V: Variable> Degree for Polynomial<R, V> {
    fn deg(&self) -> usize {
        self.monomials
            .iter()
            .map(|(term, _)| term.deg())
            .max()
            .unwrap_or(0)
    }
}

/*
fn spol<P: Polnomial>(f: &P, g: &P) -> P {
    let m = f.head_term().lcm(g.head_term());
    let a = *f.head_term();
    let b = *g.head_term();

    (f * (m / a) * g.head_coeff()) - (g * (m / b) * f.head_coeff())
}
*/
