use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::{Add, Mul, Rem, Sub};

use num::{One, Zero};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Monomial<R: Ring, V: Variable> {
    pub coeff: R,
    pub term: Term<V>,
}

use crate::variable::Variable;
use crate::{
    ring::{Ring, Z},
    term::Term,
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

impl<'a, R: Ring, V: Variable> Mul<R> for &'a Term<V> {
    type Output = Monomial<R, V>;

    fn mul(self, rhs: R) -> Self::Output {
        Monomial {
            coeff: rhs,
            term: self.clone(),
        }
    }
}

impl<R: Ring, V: Variable, T: Borrow<Monomial<R, V>>> Mul<T> for Monomial<R, V> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let right: &Monomial<R, V> = rhs.borrow();
        Monomial {
            coeff: self.coeff * right.coeff,
            term: self.term * &right.term,
        }
    }
}
