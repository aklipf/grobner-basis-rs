use std::collections::BTreeMap;

use crate::order::{Lex, Order};

use super::term::{Degree, Term, Variable};

use super::ring::Ring;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial<R: Ring, V: Variable, O: Order<Var = V> = Lex<V>> {
    monomials: BTreeMap<O, R>,
}

impl<R: Ring, V: Variable, O: Order<Var = V>> Degree for Polynomial<R, V, O> {
    fn deg(&self) -> usize {
        self.monomials
            .iter()
            .map(|(term, _)| term.deg())
            .max()
            .unwrap_or(0)
    }
}

pub trait HeadMonomial<R: Ring, V: Variable> {
    fn head_coeff(&self) -> R;
    fn head_term(&self) -> Term<V>;
}

impl<R: Ring, V: Variable, O: Order<Var = V>> HeadMonomial<R, V> for Polynomial<R, V, O> {
    fn head_coeff(&self) -> R {
        self.monomials
            .first_key_value()
            .map_or(R::zero(), |(_, &coeff)| coeff)
    }

    fn head_term(&self) -> Term<V> {
        self.monomials
            .first_key_value()
            .map_or(Default::default(), |(term, _)| (**term).clone())
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
