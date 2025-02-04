use std::{borrow::Borrow, marker::PhantomData};

use itertools::EitherOrBoth;

use crate::term::Variable;

pub trait ProductTerms<V: Variable, T: Borrow<(V, usize)>>:
    Iterator<Item = EitherOrBoth<T>> + Sized
{
    fn product_terms(self) -> ProductTermsIter<V, Self, T> {
        ProductTermsIter {
            iter: self,
            variable: Default::default(),
        }
    }
}

pub struct ProductTermsIter<V, I: Iterator<Item = EitherOrBoth<T>>, T: Borrow<(V, usize)>> {
    iter: I,
    variable: PhantomData<V>,
}

impl<V: Variable, I: Iterator<Item = EitherOrBoth<T>>, T: Borrow<(V, usize)>> Iterator
    for ProductTermsIter<V, I, T>
{
    type Item = (V, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(eob) => match eob {
                EitherOrBoth::Both(l, r) => Some((l.borrow().0, l.borrow().1 + r.borrow().1)),
                EitherOrBoth::Left(l) => Some(*l.borrow()),
                EitherOrBoth::Right(r) => Some(*r.borrow()),
            },
            None => None,
        }
    }
}

impl<T: Iterator<Item = EitherOrBoth<U>>, U: Borrow<(V, usize)>, V: Variable> ProductTerms<V, U>
    for T
{
}
