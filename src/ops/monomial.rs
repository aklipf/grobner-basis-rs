use std::ops::Mul;

use crate::{
    impl_ring_mono_mul, impl_ring_term_mul,
    monomial::{mul_mono_mono, mul_ring_mono, mul_ring_term, mul_term_mono, Monomial},
    ring::Ring,
    term::Term,
    variable::Variable,
};

// ops ring * term

impl<R: Ring, V: Variable> Mul<R> for Term<V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: R) -> Self::Output {
        mul_ring_term(rhs, self)
    }
}

impl<R: Ring, V: Variable> Mul<R> for &Term<V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: R) -> Self::Output {
        mul_ring_term(rhs, self.clone())
    }
}

impl_ring_term_mul!(i32);
impl_ring_term_mul!(i64);

// ops ring * monomial

impl<R: Ring, V: Variable> Mul<R> for Monomial<R, V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: R) -> Self::Output {
        mul_ring_mono(rhs, self)
    }
}

impl<R: Ring, V: Variable> Mul<R> for &Monomial<R, V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: R) -> Self::Output {
        mul_ring_mono(rhs, self.clone())
    }
}

impl_ring_mono_mul!(i32);
impl_ring_mono_mul!(i64);

// ops term * monomial

impl<R: Ring, V: Variable> Mul<Term<V>> for Monomial<R, V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: Term<V>) -> Self::Output {
        mul_term_mono(rhs, self)
    }
}

impl<R: Ring, V: Variable> Mul<Term<V>> for &Monomial<R, V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: Term<V>) -> Self::Output {
        mul_term_mono(rhs, self.clone())
    }
}

impl<R: Ring, V: Variable> Mul<&Term<V>> for Monomial<R, V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: &Term<V>) -> Self::Output {
        mul_term_mono(rhs.clone(), self)
    }
}

impl<R: Ring, V: Variable> Mul<&Term<V>> for &Monomial<R, V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: &Term<V>) -> Self::Output {
        mul_term_mono(rhs.clone(), self.clone())
    }
}

impl<R: Ring, V: Variable> Mul<Monomial<R, V>> for Term<V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: Monomial<R, V>) -> Self::Output {
        mul_term_mono(self, rhs)
    }
}

impl<R: Ring, V: Variable> Mul<Monomial<R, V>> for &Term<V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: Monomial<R, V>) -> Self::Output {
        mul_term_mono(self.clone(), rhs)
    }
}

impl<R: Ring, V: Variable> Mul<&Monomial<R, V>> for Term<V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: &Monomial<R, V>) -> Self::Output {
        mul_term_mono(self, rhs.clone())
    }
}

impl<R: Ring, V: Variable> Mul<&Monomial<R, V>> for &Term<V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: &Monomial<R, V>) -> Self::Output {
        mul_term_mono(self.clone(), rhs.clone())
    }
}

// ops monomial * monomial

impl<R: Ring, V: Variable> Mul<Monomial<R, V>> for Monomial<R, V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: Monomial<R, V>) -> Self::Output {
        mul_mono_mono(&self, &rhs)
    }
}

impl<R: Ring, V: Variable> Mul<&Monomial<R, V>> for Monomial<R, V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: &Monomial<R, V>) -> Self::Output {
        mul_mono_mono(&self, rhs)
    }
}

impl<R: Ring, V: Variable> Mul<Monomial<R, V>> for &Monomial<R, V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: Monomial<R, V>) -> Self::Output {
        mul_mono_mono(self, &rhs)
    }
}

impl<R: Ring, V: Variable> Mul<&Monomial<R, V>> for &Monomial<R, V> {
    type Output = Monomial<R, V>;

    #[inline]
    fn mul(self, rhs: &Monomial<R, V>) -> Self::Output {
        mul_mono_mono(self, rhs)
    }
}
