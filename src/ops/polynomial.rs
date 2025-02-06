use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::{
    monomial::Monomial,
    order::Order,
    polynomial::{
        add_poly_poly, div_poly_poly, mul_any_poly, mul_poly_poly, sub_poly_poly, Polynomial,
    },
    ring::Ring,
    term::Term,
    variable::Variable,
};

// ops any * poly

impl<R: Ring, V: Variable, O: Order> Mul<Term<V>> for Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: Term<V>) -> Self::Output {
        mul_any_poly(&rhs, &self)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<&Term<V>> for Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: &Term<V>) -> Self::Output {
        mul_any_poly(rhs, &self)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<Term<V>> for &Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: Term<V>) -> Self::Output {
        mul_any_poly(&rhs, self)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<&Term<V>> for &Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: &Term<V>) -> Self::Output {
        mul_any_poly(rhs, self)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<Polynomial<R, V, O>> for Term<V> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: Polynomial<R, V, O>) -> Self::Output {
        mul_any_poly(&self, &rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<&Polynomial<R, V, O>> for Term<V> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: &Polynomial<R, V, O>) -> Self::Output {
        mul_any_poly(&self, rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<Polynomial<R, V, O>> for &Term<V> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: Polynomial<R, V, O>) -> Self::Output {
        mul_any_poly(self, &rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<&Polynomial<R, V, O>> for &Term<V> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: &Polynomial<R, V, O>) -> Self::Output {
        mul_any_poly(self, rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<Monomial<R, V>> for Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: Monomial<R, V>) -> Self::Output {
        mul_any_poly(&rhs, &self)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<&Monomial<R, V>> for Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: &Monomial<R, V>) -> Self::Output {
        mul_any_poly(rhs, &self)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<Monomial<R, V>> for &Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: Monomial<R, V>) -> Self::Output {
        mul_any_poly(&rhs, self)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<&Monomial<R, V>> for &Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: &Monomial<R, V>) -> Self::Output {
        mul_any_poly(rhs, self)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<Polynomial<R, V, O>> for Monomial<R, V> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: Polynomial<R, V, O>) -> Self::Output {
        mul_any_poly(&self, &rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<&Polynomial<R, V, O>> for Monomial<R, V> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: &Polynomial<R, V, O>) -> Self::Output {
        mul_any_poly(&self, rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<Polynomial<R, V, O>> for &Monomial<R, V> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: Polynomial<R, V, O>) -> Self::Output {
        mul_any_poly(self, &rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<&Polynomial<R, V, O>> for &Monomial<R, V> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: &Polynomial<R, V, O>) -> Self::Output {
        mul_any_poly(self, rhs)
    }
}

// ops poly * poly

impl<R: Ring, V: Variable, O: Order> Mul<Polynomial<R, V, O>> for Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: Polynomial<R, V, O>) -> Self::Output {
        mul_poly_poly(&self, &rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<&Polynomial<R, V, O>> for Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: &Polynomial<R, V, O>) -> Self::Output {
        mul_poly_poly(&self, rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<Polynomial<R, V, O>> for &Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: Polynomial<R, V, O>) -> Self::Output {
        mul_poly_poly(self, &rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Mul<&Polynomial<R, V, O>> for &Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn mul(self, rhs: &Polynomial<R, V, O>) -> Self::Output {
        mul_poly_poly(self, rhs)
    }
}

// ops poly + poly

impl<R: Ring, V: Variable, O: Order> Add<Polynomial<R, V, O>> for Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn add(self, rhs: Polynomial<R, V, O>) -> Self::Output {
        add_poly_poly(&self, &rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Add<&Polynomial<R, V, O>> for Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn add(self, rhs: &Polynomial<R, V, O>) -> Self::Output {
        add_poly_poly(&self, rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Add<Polynomial<R, V, O>> for &Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn add(self, rhs: Polynomial<R, V, O>) -> Self::Output {
        add_poly_poly(self, &rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Add<&Polynomial<R, V, O>> for &Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn add(self, rhs: &Polynomial<R, V, O>) -> Self::Output {
        add_poly_poly(self, rhs)
    }
}

// ops poly - poly

impl<R: Ring, V: Variable, O: Order> Sub<Polynomial<R, V, O>> for Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn sub(self, rhs: Polynomial<R, V, O>) -> Self::Output {
        sub_poly_poly(&self, &rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Sub<&Polynomial<R, V, O>> for Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn sub(self, rhs: &Polynomial<R, V, O>) -> Self::Output {
        sub_poly_poly(&self, rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Sub<Polynomial<R, V, O>> for &Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn sub(self, rhs: Polynomial<R, V, O>) -> Self::Output {
        sub_poly_poly(self, &rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Sub<&Polynomial<R, V, O>> for &Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    #[inline]
    fn sub(self, rhs: &Polynomial<R, V, O>) -> Self::Output {
        sub_poly_poly(self, rhs)
    }
}

// ops poly / poly

impl<R: Ring, V: Variable, O: Order> Div<Polynomial<R, V, O>> for Polynomial<R, V, O>
where
    R: Rem<R, Output = R> + Div<R, Output = R>,
{
    type Output = (Polynomial<R, V, O>, Polynomial<R, V, O>);

    #[inline]
    fn div(self, rhs: Polynomial<R, V, O>) -> Self::Output {
        div_poly_poly(&self, &rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Div<&Polynomial<R, V, O>> for Polynomial<R, V, O>
where
    R: Rem<R, Output = R> + Div<R, Output = R>,
{
    type Output = (Polynomial<R, V, O>, Polynomial<R, V, O>);

    #[inline]
    fn div(self, rhs: &Polynomial<R, V, O>) -> Self::Output {
        div_poly_poly(&self, rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Div<Polynomial<R, V, O>> for &Polynomial<R, V, O>
where
    R: Rem<R, Output = R> + Div<R, Output = R>,
{
    type Output = (Polynomial<R, V, O>, Polynomial<R, V, O>);

    #[inline]
    fn div(self, rhs: Polynomial<R, V, O>) -> Self::Output {
        div_poly_poly(self, &rhs)
    }
}

impl<R: Ring, V: Variable, O: Order> Div<&Polynomial<R, V, O>> for &Polynomial<R, V, O>
where
    R: Rem<R, Output = R> + Div<R, Output = R>,
{
    type Output = (Polynomial<R, V, O>, Polynomial<R, V, O>);

    #[inline]
    fn div(self, rhs: &Polynomial<R, V, O>) -> Self::Output {
        div_poly_poly(self, rhs)
    }
}
