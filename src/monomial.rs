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

impl<const N: usize, T, V> Mul<Z<N, T>> for Term<V>
where
    V: Variable,
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
    type Output = (Z<N, T>, Self);

    fn mul(self, rhs: Z<N, T>) -> Self::Output {
        (rhs, self)
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
