use std::{cmp::Ordering, iter::Peekable};

use itertools::EitherOrBoth;

use crate::term::Variable;

pub trait JoinTerms<V: Variable>: Iterator<Item = (V, usize)> + Sized {
    fn join_terms<I: Iterator<Item = (V, usize)>>(self, iter: I) -> JoinTermsIter<V, Self, I> {
        JoinTermsIter {
            left: self.peekable(),
            right: iter.peekable(),
        }
    }
}

pub struct JoinTermsIter<
    V: Variable,
    I: Iterator<Item = (V, usize)>,
    J: Iterator<Item = (V, usize)>,
> {
    left: Peekable<I>,
    right: Peekable<J>,
}

impl<V: Variable, I: Iterator<Item = (V, usize)>, J: Iterator<Item = (V, usize)>> Iterator
    for JoinTermsIter<V, I, J>
{
    type Item = EitherOrBoth<(V, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.left.peek(), self.right.peek()) {
            (None, None) => None,
            (None, Some(&r)) => {
                self.right.next();
                Some(EitherOrBoth::Right(r))
            }
            (Some(&l), None) => {
                self.left.next();
                Some(EitherOrBoth::Left(l))
            }
            (Some(&l), Some(&r)) => match l.0.cmp(&r.0) {
                Ordering::Less => {
                    self.left.next();
                    Some(EitherOrBoth::Left(l))
                }
                Ordering::Equal => {
                    self.left.next();
                    self.right.next();
                    Some(EitherOrBoth::Both(l, r))
                }
                Ordering::Greater => {
                    self.right.next();
                    Some(EitherOrBoth::Right(r))
                }
            },
        }
    }
}

impl<T: Iterator<Item = (V, usize)>, V: Variable> JoinTerms<V> for T {
    fn join_terms<I: Iterator<Item = (V, usize)>>(self, iter: I) -> JoinTermsIter<V, Self, I> {
        JoinTermsIter {
            left: self.peekable(),
            right: iter.peekable(),
        }
    }
}
