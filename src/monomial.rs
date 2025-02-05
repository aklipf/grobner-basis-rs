use std::fmt::Debug;
use std::ops::{Add, Mul, Rem, Sub};

use num::{One, Zero};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Monomial<R: Ring, V: Variable> {
    pub coeff: R,
    pub term: Term<V>,
}

use crate::{
    ring::{Ring, Z},
    term::{Term, Variable},
};

impl<const N: usize, T, V> Mul<Term<V>> for Z<N, T>
where
    V: Variable,
    Z<N, T>: Ring,
    T: Rem<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<Output = T>
        + From<usize>
        + Zero
        + One
        + Default
        + Copy,
{
    type Output = Monomial<Z<N, T>, V>;

    fn mul(self, rhs: Term<V>) -> Self::Output {
        Monomial {
            coeff: self,
            term: rhs,
        }
    }
}

impl<R: Ring, V: Variable> Mul<R> for Term<V> {
    type Output = Monomial<R, V>;

    fn mul(self, rhs: R) -> Self::Output {
        Monomial {
            coeff: rhs,
            term: self,
        }
    }
}

impl<R: Ring, V: Variable> Mul<Self> for Monomial<R, V> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Monomial {
            coeff: self.coeff * rhs.coeff,
            term: self.term * rhs.term,
        }
    }
}
/*
pub trait JoinMonomials<T: Order, R: Ring>: Iterator<Item = (T, R)> + Sized {
    fn join_monos<I: Iterator<Item = (T, R)>>(self, iter: I) -> JoinMonomialsIter<Self, I, T, R> {
        JoinMonomialsIter {
            left: self.peekable(),
            right: iter.peekable(),
        }
    }
}

pub struct JoinMonomialsIter<
    I: Iterator<Item = (T, R)>,
    J: Iterator<Item = (T, R)>,
    T: Order,
    R: Ring,
> {
    left: Peekable<I>,
    right: Peekable<J>,
}

impl<I: Iterator<Item = (T, R)>, J: Iterator<Item = (T, R)>, T: Order, R: Ring> Iterator
    for JoinMonomialsIter<I, J, T, R>
{
    type Item = EitherOrBoth<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.left.peek(), self.right.peek()) {
            (None, None) => None,
            (None, Some(_)) => Some(EitherOrBoth::Right(self.right.next().unwrap())),
            (Some(_), None) => Some(EitherOrBoth::Left(self.left.next().unwrap())),
            (Some(l), Some(r)) => match l.cmp(r) {
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

impl<I: Iterator<Item = (T, R)>, T: Order, R: Ring> JoinMonomials<T, R> for I {}
 */
