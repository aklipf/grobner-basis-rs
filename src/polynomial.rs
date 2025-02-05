use std::borrow::Borrow;
use std::collections::btree_map::IntoIter;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

use itertools::Itertools;

use crate::monomial::Monomial;
use crate::order::{Lex, Order};
use crate::term::lcm;

use super::term::{Degree, Term, Variable};

use super::ring::Ring;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial<R: Ring, V: Variable, O: Order<Var = V> = Lex<V>> {
    pub(crate) monomials: BTreeMap<O, R>,
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

impl<R: Ring, V: Variable, O: Order<Var = V>> Default for Polynomial<R, V, O> {
    fn default() -> Self {
        Self {
            monomials: Default::default(),
        }
    }
}

impl<B: Borrow<Monomial<R, V>>, R: Ring, V: Variable, O: Order<Var = V>> FromIterator<B>
    for Polynomial<R, V, O>
{
    fn from_iter<T: IntoIterator<Item = B>>(iter: T) -> Self {
        let mut monomials: BTreeMap<O, R> = Default::default();

        for borrowed in iter {
            let mono: &Monomial<R, V> = borrowed.borrow();
            let term: O = mono.term.clone().into();
            if let Some(coeff) = monomials.get_mut(&term) {
                *coeff = *coeff + mono.coeff;
                if coeff.is_zero() {
                    monomials.remove(&term);
                }
            } else if !mono.coeff.is_zero() {
                monomials.insert(term, mono.coeff);
            }
        }

        Polynomial {
            monomials: monomials,
        }
    }
}

impl<R: Ring, V: Variable, O: Order<Var = V>> IntoIterator for Polynomial<R, V, O> {
    type Item = Monomial<R, V>;

    type IntoIter = MonomialIter<IntoIter<O, R>, O, R>;

    fn into_iter(self) -> Self::IntoIter {
        MonomialIter {
            iter: self.monomials.into_iter(),
        }
    }
}

pub struct MonomialIter<I: Iterator<Item = (O, R)>, O: Order, R: Ring> {
    iter: I,
}

impl<I: Iterator<Item = (O, R)>, O: Order, R: Ring> Iterator for MonomialIter<I, O, R> {
    type Item = Monomial<R, O::Var>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(term, coeff)| Monomial {
            coeff: coeff,
            term: (*term).clone(),
        })
    }
}

impl<R: Ring, V: Variable, O: Order<Var = V>> Mul<Monomial<R, V>> for Polynomial<R, V, O> {
    type Output = Self;

    fn mul(self, rhs: Monomial<R, V>) -> Self::Output {
        let mut monomial: BTreeMap<O, R> = Default::default();
        for x in self.into_iter() {
            let result = x.mul(rhs.clone());
            monomial.insert(result.term.into(), result.coeff);
        }

        Polynomial {
            monomials: monomial,
        }
    }
}

impl<'a, R: Ring, V: Variable, O: Order<Var = V>> Mul<&'a Polynomial<R, V, O>> for Monomial<R, V> {
    type Output = Polynomial<R, V, O>;

    fn mul(self, rhs: &'a Polynomial<R, V, O>) -> Self::Output {
        let mut monomial: BTreeMap<O, R> = Default::default();
        for x in rhs.clone().into_iter() {
            let result = x.mul(self.clone());
            monomial.insert(result.term.into(), result.coeff);
        }

        Polynomial {
            monomials: monomial,
        }
    }
}

impl<R: Ring, V: Variable, O: Order<Var = V>> Add<Self> for Polynomial<R, V, O> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter().chain(rhs.into_iter()).collect()
    }
}

impl<R: Ring, V: Variable, O: Order<Var = V>> Sub<Self> for Polynomial<R, V, O> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .chain(rhs.into_iter().map(|mono| Monomial {
                coeff: mono.coeff.neg(),
                term: mono.term,
            }))
            .collect()
    }
}

pub fn sploy<R: Ring, V: Variable, O: Order<Var = V>>(
    f: &Polynomial<R, V, O>,
    g: &Polynomial<R, V, O>,
) -> Polynomial<R, V, O>
where
    Term<V>: Mul<R, Output = Monomial<R, V>>,
{
    let m = lcm(&f.head_term(), &g.head_term());

    (&m / &f.head_term()) * g.head_coeff() * f - ((&m / &g.head_term()) * f.head_coeff()) * g
}

impl<R: Ring, V: Variable, O: Order<Var = V>> Display for Polynomial<R, V, O>
where
    R: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.monomials
                .iter()
                .rev()
                .map(|(term, coeff)| if coeff.is_one() && term.exps.len() > 0 {
                    format!("{}", **term)
                } else {
                    format!("{}{}", coeff, **term)
                })
                .join(" + ")
        )
    }
}
