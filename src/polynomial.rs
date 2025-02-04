use std::ops::{Add, Div, Mul, Sub};

use crate::monomial::Monomial;

use super::term::{Degree, Term, Variable};

use super::ring::Ring;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial<R: Ring, V: Variable> {
    monomials: Vec<(R, Term<V>)>,
}

impl<R: Ring, V: Variable> Degree for Polynomial<R, V> {
    fn deg(&self) -> usize {
        self.monomials
            .iter()
            .map(|(_, term)| term.deg())
            .max()
            .unwrap_or(0)
    }
}

pub trait HeadMonomial<R: Ring, V: Variable> {
    fn head_coeff(&self) -> &R;
    fn head_term(&self) -> &Term<V>;
    fn head_monomial(&self) -> Monomial<R, V>;
}

/*
fn spol<P: Polnomial>(f: &P, g: &P) -> P {
    let m = f.head_term().lcm(g.head_term());
    let a = *f.head_term();
    let b = *g.head_term();

    (f * (m / a) * g.head_coeff()) - (g * (m / b) * f.head_coeff())
}
*/
