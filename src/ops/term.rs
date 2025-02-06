use std::ops::{Div, Mul};

use crate::{
    term::{div_term_term, mul_term_term, Term},
    variable::Variable,
};

// ops term * term

impl<V: Variable> Mul<Term<V>> for Term<V> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Term<V>) -> Self::Output {
        mul_term_term(&self, &rhs)
    }
}

impl<'a, V: Variable> Mul<&'a Term<V>> for Term<V> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &'a Term<V>) -> Self::Output {
        mul_term_term(&self, rhs)
    }
}

impl<'a, V: Variable> Mul<Term<V>> for &'a Term<V> {
    type Output = Term<V>;

    #[inline]
    fn mul(self, rhs: Term<V>) -> Self::Output {
        mul_term_term(self, &rhs)
    }
}

impl<'a, 'b, V: Variable> Mul<&'b Term<V>> for &'a Term<V> {
    type Output = Term<V>;

    #[inline]
    fn mul(self, rhs: &'b Term<V>) -> Self::Output {
        mul_term_term(self, rhs)
    }
}

// ops term / term

impl<V: Variable> Div<Term<V>> for Term<V> {
    type Output = Option<Term<V>>;

    fn div(self, rhs: Term<V>) -> Self::Output {
        div_term_term(&self, &rhs)
    }
}

impl<'a, V: Variable> Div<&'a Term<V>> for Term<V> {
    type Output = Option<Term<V>>;

    fn div(self, rhs: &'a Term<V>) -> Self::Output {
        div_term_term(&self, rhs)
    }
}

impl<'a, V: Variable> Div<Term<V>> for &'a Term<V> {
    type Output = Option<Term<V>>;

    fn div(self, rhs: Term<V>) -> Self::Output {
        div_term_term(self, &rhs)
    }
}

impl<'a, 'b, V: Variable> Div<&'b Term<V>> for &'a Term<V> {
    type Output = Option<Term<V>>;

    fn div(self, rhs: &'b Term<V>) -> Self::Output {
        div_term_term(self, rhs)
    }
}
