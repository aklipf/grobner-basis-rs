use std::{borrow::Borrow, marker::PhantomData};

use itertools::EitherOrBoth;

use crate::term::Variable;

pub trait AddExponents<V: Variable, T: Borrow<(V, usize)>>:
    Iterator<Item = EitherOrBoth<T>> + Sized
{
    fn add_exponents(self) -> AddExponentsIter<V, Self, T> {
        AddExponentsIter {
            iter: self,
            variable: Default::default(),
        }
    }
}

pub struct AddExponentsIter<V, I: Iterator<Item = EitherOrBoth<T>>, T: Borrow<(V, usize)>> {
    iter: I,
    variable: PhantomData<V>,
}

impl<V: Variable, I: Iterator<Item = EitherOrBoth<T>>, T: Borrow<(V, usize)>> Iterator
    for AddExponentsIter<V, I, T>
{
    type Item = (V, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(eob) => match eob {
                EitherOrBoth::Both(l, r) => {
                    let &(var, left_exp) = l.borrow();
                    let &(_, right_exp) = r.borrow();
                    Some((var, left_exp + right_exp))
                }
                EitherOrBoth::Left(l) => Some(*l.borrow()),
                EitherOrBoth::Right(r) => Some(*r.borrow()),
            },
            None => None,
        }
    }
}

impl<T: Iterator<Item = EitherOrBoth<U>>, U: Borrow<(V, usize)>, V: Variable> AddExponents<V, U>
    for T
{
}

pub trait SubExponents<V: Variable, T: Borrow<(V, usize)>>:
    Iterator<Item = EitherOrBoth<T>> + Sized
{
    fn sub_exponents(self) -> SubExponentsIter<V, Self, T> {
        SubExponentsIter {
            iter: self,
            variable: Default::default(),
        }
    }
}

pub struct SubExponentsIter<V, I: Iterator<Item = EitherOrBoth<T>>, T: Borrow<(V, usize)>> {
    iter: I,
    variable: PhantomData<V>,
}

impl<V: Variable, I: Iterator<Item = EitherOrBoth<T>>, T: Borrow<(V, usize)>> Iterator
    for SubExponentsIter<V, I, T>
{
    type Item = Result<(V, usize), String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(eob) => match eob {
                EitherOrBoth::Both(l, r) => {
                    let &(var, left_exp) = l.borrow();
                    let &(_, right_exp) = r.borrow();

                    if left_exp >= right_exp {
                        Some(Ok((var, left_exp - right_exp)))
                    } else {
                        Some(Err(format!(
                            "The resulting exponent of {}^{}/{}^{} is negative",
                            var, left_exp, var, right_exp
                        )))
                    }
                }
                EitherOrBoth::Left(l) => Some(Ok(*l.borrow())),
                EitherOrBoth::Right(r) => {
                    let (var, exp) = r.borrow();
                    Some(Err(format!(
                        "The resulting exponent of {}^0/{}^{} is negative",
                        var, var, exp
                    )))
                }
            },
            None => None,
        }
    }
}

impl<T: Iterator<Item = EitherOrBoth<U>>, U: Borrow<(V, usize)>, V: Variable> SubExponents<V, U>
    for T
{
}

pub trait MaxExponents<V: Variable, T: Borrow<(V, usize)>>:
    Iterator<Item = EitherOrBoth<T>> + Sized
{
    fn max_exponents(self) -> MaxExponentsIter<V, Self, T> {
        MaxExponentsIter {
            iter: self,
            variable: Default::default(),
        }
    }
}

pub struct MaxExponentsIter<V, I: Iterator<Item = EitherOrBoth<T>>, T: Borrow<(V, usize)>> {
    iter: I,
    variable: PhantomData<V>,
}

impl<V: Variable, I: Iterator<Item = EitherOrBoth<T>>, T: Borrow<(V, usize)>> Iterator
    for MaxExponentsIter<V, I, T>
{
    type Item = (V, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(eob) => match eob {
                EitherOrBoth::Both(l, r) => {
                    let &(var, left_exp) = l.borrow();
                    let &(_, right_exp) = r.borrow();
                    Some((var, left_exp.max(right_exp)))
                }
                EitherOrBoth::Left(l) => Some(*l.borrow()),
                EitherOrBoth::Right(r) => Some(*r.borrow()),
            },
            None => None,
        }
    }
}

impl<T: Iterator<Item = EitherOrBoth<U>>, U: Borrow<(V, usize)>, V: Variable> MaxExponents<V, U>
    for T
{
}
