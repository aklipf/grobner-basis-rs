use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Monomial<R: Ring, V: Variable> {
    pub coeff: R,
    pub term: Term<V>,
}

use crate::variable::Variable;
use crate::{ring::Ring, term::Term};

#[inline]
pub(crate) fn mul_ring_term<R: Ring, V: Variable>(left: R, right: Term<V>) -> Monomial<R, V> {
    Monomial {
        coeff: left,
        term: right,
    }
}

#[inline]
pub(crate) fn mul_ring_mono<R: Ring, V: Variable>(
    left: R,
    right: Monomial<R, V>,
) -> Monomial<R, V> {
    Monomial {
        coeff: left * right.coeff,
        term: right.term,
    }
}

#[inline]
pub(crate) fn mul_term_mono<R: Ring, V: Variable>(
    left: Term<V>,
    right: Monomial<R, V>,
) -> Monomial<R, V> {
    Monomial {
        coeff: right.coeff,
        term: left * right.term,
    }
}

#[inline]
pub(crate) fn mul_mono_mono<R: Ring, V: Variable>(
    left: &Monomial<R, V>,
    right: &Monomial<R, V>,
) -> Monomial<R, V> {
    Monomial {
        coeff: left.coeff * right.coeff,
        term: &left.term * &right.term,
    }
}
