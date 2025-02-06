use std::{
    borrow::Borrow,
    collections::{
        btree_map::{IntoIter, Iter},
        BTreeMap,
    },
    ops::Deref,
};

use crate::{
    monomial::Monomial,
    order::{Order, OrderedTerm},
    polynomial::Polynomial,
    ring::Ring,
    variable::Variable,
};

impl<B: Borrow<Monomial<R, V>>, R: Ring, V: Variable, O: Order> FromIterator<B>
    for Polynomial<R, V, O>
{
    fn from_iter<T: IntoIterator<Item = B>>(iter: T) -> Self {
        let mut monomials: BTreeMap<OrderedTerm<V, O>, R> = Default::default();

        for borrowed in iter {
            let mono: &Monomial<R, V> = borrowed.borrow();
            let term: OrderedTerm<V, O> = mono.term.clone().into();
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

impl<R: Ring, V: Variable, O: Order> IntoIterator for Polynomial<R, V, O> {
    type Item = Monomial<R, V>;

    type IntoIter = MonomialIter<IntoIter<OrderedTerm<V, O>, R>, R, V, O>;

    fn into_iter(self) -> Self::IntoIter {
        MonomialIter {
            iter: self.monomials.into_iter(),
        }
    }
}

pub struct MonomialIter<I: Iterator<Item = (OrderedTerm<V, O>, R)>, R: Ring, V: Variable, O: Order>
{
    iter: I,
}

impl<I: Iterator<Item = (OrderedTerm<V, O>, R)>, R: Ring, V: Variable, O: Order> Iterator
    for MonomialIter<I, R, V, O>
{
    type Item = Monomial<R, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(term, coeff)| Monomial {
            coeff: coeff,
            term: (*term).clone(),
        })
    }
}

pub struct MonomialRefIter<
    'a,
    I: Iterator<Item = (&'a OrderedTerm<V, O>, &'a R)>,
    R: Ring + 'a,
    V: Variable + 'a,
    O: Order + 'a,
> {
    iter: I,
}

impl<
        'a,
        I: Iterator<Item = (&'a OrderedTerm<V, O>, &'a R)>,
        R: Ring + 'a,
        V: Variable + 'a,
        O: Order + 'a,
    > Iterator for MonomialRefIter<'a, I, R, V, O>
{
    type Item = Monomial<R, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(term, coeff)| Monomial {
            coeff: coeff.clone(),
            term: term.deref().clone(),
        })
    }
}

impl<R: Ring, V: Variable, O: Order> Polynomial<R, V, O> {
    pub fn iter(&self) -> MonomialRefIter<Iter<'_, OrderedTerm<V, O>, R>, R, V, O> {
        MonomialRefIter {
            iter: self.monomials.iter(),
        }
    }
}
