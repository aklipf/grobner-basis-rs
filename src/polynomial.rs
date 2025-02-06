use std::borrow::Borrow;
use std::collections::btree_map::{IntoIter, Iter};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::ops::{Add, Deref, Div, Mul, Rem, Sub};
use std::sync::Once;

use itertools::Itertools;

use crate::monomial::Monomial;
use crate::order::{Lex, Order, OrderedTerm};
use crate::term::lcm;
use crate::variable::{Var, Variable};

use super::term::{Degree, Term};

use super::ring::Ring;

#[derive(Debug, PartialEq, Eq)]
pub struct Polynomial<R: Ring = i32, V: Variable = Var, O: Order = Lex> {
    pub(crate) monomials: BTreeMap<OrderedTerm<V, O>, R>,
}

impl<R: Ring, V: Variable, O: Order> Clone for Polynomial<R, V, O> {
    fn clone(&self) -> Self {
        Self {
            monomials: self.monomials.clone(),
        }
    }
}

impl<R: Ring, V: Variable, O: Order> Degree for Polynomial<R, V, O> {
    fn deg(&self) -> usize {
        self.monomials
            .iter()
            .map(|(term, _)| term.deg())
            .max()
            .unwrap_or(0)
    }
}

pub trait HeadMonomial<R: Ring, V: Variable> {
    fn lead_coeff(&self) -> R;
    fn lead_term(&self) -> Term<V>;
}

impl<R: Ring, V: Variable, O: Order> HeadMonomial<R, V> for Polynomial<R, V, O> {
    fn lead_coeff(&self) -> R {
        self.monomials
            .last_key_value()
            .map_or(R::zero(), |(_, &coeff)| coeff)
    }

    fn lead_term(&self) -> Term<V> {
        self.monomials
            .last_key_value()
            .map_or(Default::default(), |(term, _)| (**term).clone())
    }
}

impl<R: Ring, V: Variable, O: Order> Default for Polynomial<R, V, O> {
    fn default() -> Self {
        Self {
            monomials: Default::default(),
        }
    }
}

impl<B: Borrow<Monomial<R, V>>, R: Ring, V: Variable, O: Order> FromIterator<B>
    for Polynomial<R, V, O>
{
    fn from_iter<T: IntoIterator<Item = B>>(iter: T) -> Self {
        let mut monomials: BTreeMap<OrderedTerm<V, O>, R> = Default::default();

        for borrowed in iter {
            let mono: &Monomial<R, V> = borrowed.borrow();
            let term: OrderedTerm<V, O> = mono.term.clone().into();
            if let Some(coeff) = monomials.get_mut(&term) {
                *coeff = *coeff + mono.coeff;
                if coeff.is_zero() {
                    monomials.remove(&term);
                }
            } else if !mono.coeff.is_zero() {
                monomials.insert(term, mono.coeff);
            }
        }

        Polynomial {
            monomials: monomials,
        }
    }
}

impl<R: Ring, V: Variable, O: Order> IntoIterator for Polynomial<R, V, O> {
    type Item = Monomial<R, V>;

    type IntoIter = MonomialIter<IntoIter<OrderedTerm<V, O>, R>, R, V, O>;

    fn into_iter(self) -> Self::IntoIter {
        MonomialIter {
            iter: self.monomials.into_iter(),
        }
    }
}

pub struct MonomialIter<I: Iterator<Item = (OrderedTerm<V, O>, R)>, R: Ring, V: Variable, O: Order>
{
    iter: I,
}

impl<I: Iterator<Item = (OrderedTerm<V, O>, R)>, R: Ring, V: Variable, O: Order> Iterator
    for MonomialIter<I, R, V, O>
{
    type Item = Monomial<R, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(term, coeff)| Monomial {
            coeff: coeff,
            term: (*term).clone(),
        })
    }
}
#[inline]
pub(crate) fn mul_any_poly<'a, R: Ring, V: Variable, O: Order, T>(
    left: &'a T,
    right: &Polynomial<R, V, O>,
) -> Polynomial<R, V, O>
where
    Monomial<R, V>: Mul<&'a T, Output = Monomial<R, V>>,
{
    let mut monomial: BTreeMap<OrderedTerm<V, O>, R> = Default::default();
    for m in right.iter() {
        let result = m * left;
        monomial.insert(result.term.into(), result.coeff);
    }

    Polynomial {
        monomials: monomial,
    }
}

#[inline]
pub(crate) fn mul_poly_poly<R: Ring, V: Variable, O: Order>(
    left: &Polynomial<R, V, O>,
    right: &Polynomial<R, V, O>,
) -> Polynomial<R, V, O> {
    let mut monomials: Vec<Monomial<R, V>> = Default::default();
    for m_left in left.iter() {
        for m_right in right.iter() {
            monomials.push(m_right * &m_left);
        }
    }

    monomials.into_iter().collect()
}

#[inline]
pub(crate) fn add_poly_poly<R: Ring, V: Variable, O: Order>(
    left: &Polynomial<R, V, O>,
    right: &Polynomial<R, V, O>,
) -> Polynomial<R, V, O> {
    left.iter().chain(right.iter()).collect()
}

#[inline]
pub(crate) fn sub_poly_poly<R: Ring, V: Variable, O: Order>(
    left: &Polynomial<R, V, O>,
    right: &Polynomial<R, V, O>,
) -> Polynomial<R, V, O> {
    left.iter()
        .chain(right.iter().map(|mono| Monomial {
            coeff: mono.coeff.neg(),
            term: mono.term,
        }))
        .collect()
}

#[inline]
pub(crate) fn div_poly_poly<R: Ring, V: Variable, O: Order>(
    left: &Polynomial<R, V, O>,
    right: &Polynomial<R, V, O>,
) -> (Polynomial<R, V, O>, Polynomial<R, V, O>)
where
    R: Rem<R, Output = R> + Div<R, Output = R>,
{
    let mut f: Polynomial<R, V, O> = left.clone();

    let mut rem_monomial: Vec<Monomial<R, V>> = Default::default();

    loop {
        if (f.lead_coeff() % right.lead_coeff()) == R::zero() {
            let c = f.lead_coeff() / right.lead_coeff();
            if let Some(m) = f.lead_term() / right.lead_term() {
                rem_monomial.push(&m * c);
                f = f - (right * (m * c));
                continue;
            }
        }
        break;
    }

    (rem_monomial.into_iter().collect(), f)
}
pub struct MonomialRefIter<
    'a,
    I: Iterator<Item = (&'a OrderedTerm<V, O>, &'a R)>,
    R: Ring + 'a,
    V: Variable + 'a,
    O: Order + 'a,
> {
    iter: I,
}

impl<
        'a,
        I: Iterator<Item = (&'a OrderedTerm<V, O>, &'a R)>,
        R: Ring + 'a,
        V: Variable + 'a,
        O: Order + 'a,
    > Iterator for MonomialRefIter<'a, I, R, V, O>
{
    type Item = Monomial<R, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(term, coeff)| Monomial {
            coeff: coeff.clone(),
            term: term.deref().clone(),
        })
    }
}

impl<R: Ring, V: Variable, O: Order> Polynomial<R, V, O> {
    pub fn div_euclid<T: Borrow<Self>>(&self, rhs: &T) -> (Polynomial<R, V, O>, Polynomial<R, V, O>)
    where
        R: Rem<R, Output = R> + Div<R, Output = R>,
        Term<V>: Mul<R, Output = Monomial<R, V>>,
    {
        let mut f: Polynomial<R, V, O> = self.clone();
        let div: &Polynomial<R, V, O> = rhs.borrow();

        let mut rem_monomial: Vec<Monomial<R, V>> = Default::default();

        loop {
            if (f.lead_coeff() % div.lead_coeff()) == R::zero() {
                let c = f.lead_coeff() / div.lead_coeff();
                if let Some(m) = f.lead_term() / div.lead_term() {
                    rem_monomial.push(&m * c);
                    f = f - (div * (m * c));
                    continue;
                }
            }
            break;
        }

        (rem_monomial.into_iter().collect(), f)
    }

    pub fn iter(&self) -> MonomialRefIter<Iter<'_, OrderedTerm<V, O>, R>, R, V, O> {
        MonomialRefIter {
            iter: self.monomials.iter(),
        }
    }
}

impl<R: Ring, V: Variable, O: Order> Display for Polynomial<R, V, O>
where
    R: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.monomials.is_empty() {
            write!(f, "{}", R::zero())
        } else {
            write!(
                f,
                "{}",
                self.monomials
                    .iter()
                    .rev()
                    .map(|(term, coeff)| if coeff.is_one() && term.exps.len() > 0 {
                        format!("{}", **term)
                    } else {
                        format!("{}{}", coeff, **term)
                    })
                    .join(" + ")
            )
        }
    }
}

pub fn sploy<R: Ring, V: Variable, O: Order>(
    f: &Polynomial<R, V, O>,
    g: &Polynomial<R, V, O>,
) -> Polynomial<R, V, O>
where
    Term<V>: Mul<R, Output = Monomial<R, V>>,
{
    let m = lcm(&f.lead_term(), &g.lead_term());

    (&m / &f.lead_term()).unwrap() * g.lead_coeff() * f
        - ((&m / &g.lead_term()).unwrap() * f.lead_coeff()) * g
}

pub fn buchberger<R: Ring, V: Variable, O: Order>(
    polys: &Vec<Polynomial<R, V, O>>,
) -> Vec<Polynomial<R, V, O>>
where
    R: Rem<R, Output = R> + Div<R, Output = R> + Display,
    Term<V>: Mul<R, Output = Monomial<R, V>>,
{
    let mut g = polys.clone();

    loop {
        let mut next_ideal: Option<Polynomial<R, V, O>> = None;
        for (i, g_i) in g.iter().enumerate() {
            for (j, g_j) in g.iter().enumerate() {
                if i == j {
                    continue;
                }
                let mut s_ij = sploy(g_i, g_j);
                for g_k in g.iter() {
                    (_, s_ij) = s_ij.div_euclid(&g_k);
                }
                if !s_ij.monomials.is_empty() {
                    next_ideal = Some(s_ij);
                }

                if next_ideal.is_some() {
                    break;
                }
            }

            if next_ideal.is_some() {
                break;
            }
        }
        if let Some(ideal) = next_ideal {
            g.push(ideal);
        } else {
            break;
        }
    }

    reduced(g)
}

fn reduced<R: Ring, V: Variable, O: Order>(
    polys: Vec<Polynomial<R, V, O>>,
) -> Vec<Polynomial<R, V, O>>
where
    R: Rem<R, Output = R> + Div<R, Output = R> + Display,
    Term<V>: Mul<R, Output = Monomial<R, V>>,
{
    //polys.sort_by(|left, right| left.lead_term().cmp(right.lead_term()));
    polys
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, str::FromStr};

    use super::*;

    #[test]
    fn add_polynomial() {
        let f: Polynomial<i32, Var> = Polynomial::from_str("x^2+-3xy+2x^2y^3+y^2+2").unwrap();
        let g: Polynomial<i32, Var> = Polynomial::from_str("x+xy+x^2y+x^2+1").unwrap();
        let result: Polynomial<i32, Var> =
            Polynomial::from_str("2x^2+-2xy+2x^2y^3+y^2+x+x^2y+3").unwrap();

        assert_eq!(f + g, result);
    }

    #[test]
    fn sub_polynomial() {
        let f: Polynomial<i32, Var> = Polynomial::from_str("x^2+-3xy+2x^2y^3+y^2+2").unwrap();
        let g: Polynomial<i32, Var> = Polynomial::from_str("x+xy+x^2y+x^2+1").unwrap();
        let result: Polynomial<i32, Var> =
            Polynomial::from_str("-4xy+2x^2y^3+y^2+1+-x+-x^2y").unwrap();

        assert_eq!(f - g, result);
    }

    #[test]
    fn mul_polynomial() {
        let f: Polynomial<i32, Var> = Polynomial::from_str("x^2+-3xy+2x^2y^3+y^2+2").unwrap();
        let g: Polynomial<i32, Var> = Polynomial::from_str("x+xy+x^2y+x^2+1").unwrap();
        let result: Polynomial<i32, Var> =
            Polynomial::from_str("2x^4y^4+2x^4y^3+x^4y+x^4+2x^3y^4+2x^3y^3+-3x^3y^2+-2x^3y+x^3+3x^2y^3+-2x^2y^2+-x^2y+3x^2+xy^3+xy^2+-xy+2x+y^2+2").unwrap();

        assert_eq!(f * g, result);
    }

    #[test]
    fn div_euclid_polynomial() {
        let f: Polynomial<i32, Var, Lex> = Polynomial::from_str("x^2+-3xy+2x^2y^3+y^2+2").unwrap();
        let g: Polynomial<i32, Var, Lex> = Polynomial::from_str("x+xy+x^2y+x^2+1").unwrap();

        let (q, r) = f.div_euclid(&g);

        assert!(Lex::cmp(&r.lead_term().into(), &g.lead_term().into()) == Ordering::Less);
        assert_eq!(f, q * g + r);
    }
}
