use std::{borrow::Borrow, cmp::Ordering, iter::Peekable, marker::PhantomData};

use itertools::EitherOrBoth;

use crate::variable::Variable;

pub trait JoinTerms<V: Variable, T: Borrow<(V, usize)>>: Iterator<Item = T> + Sized {
    fn join_terms<I: Iterator<Item = U>, U: Borrow<(V, usize)>>(
        self,
        iter: I,
    ) -> JoinTermsIter<V, Self, I, T, U> {
        JoinTermsIter {
            left: self.peekable(),
            right: iter.peekable(),
            variable: Default::default(),
        }
    }
}

pub struct JoinTermsIter<
    V: Variable,
    I: Iterator<Item = T>,
    J: Iterator<Item = U>,
    T: Borrow<(V, usize)>,
    U: Borrow<(V, usize)>,
> {
    left: Peekable<I>,
    right: Peekable<J>,
    variable: PhantomData<V>,
}

impl<
        V: Variable,
        I: Iterator<Item = T>,
        J: Iterator<Item = U>,
        T: Borrow<(V, usize)>,
        U: Borrow<(V, usize)>,
    > Iterator for JoinTermsIter<V, I, J, T, U>
{
    type Item = EitherOrBoth<T, U>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.left.peek(), self.right.peek()) {
            (None, None) => None,
            (None, Some(_)) => Some(EitherOrBoth::Right(self.right.next().unwrap())),
            (Some(_), None) => Some(EitherOrBoth::Left(self.left.next().unwrap())),
            (Some(l), Some(r)) => match l.borrow().0.cmp(&r.borrow().0) {
                Ordering::Less => Some(EitherOrBoth::Left(self.left.next().unwrap())),
                Ordering::Equal => Some(EitherOrBoth::Both(
                    self.left.next().unwrap(),
                    self.right.next().unwrap(),
                )),
                Ordering::Greater => Some(EitherOrBoth::Right(self.right.next().unwrap())),
            },
        }
    }
}

impl<T: Iterator<Item = U>, U: Borrow<(V, usize)>, V: Variable> JoinTerms<V, U> for T {}
