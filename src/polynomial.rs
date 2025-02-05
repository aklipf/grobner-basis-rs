use std::borrow::Borrow;
use std::collections::btree_map::{IntoIter, Iter};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Rem, Sub};

use itertools::Itertools;

use crate::monomial::Monomial;
use crate::order::{Lex, Order};
use crate::term::lcm;

use super::term::{Degree, Term, Variable};

use super::ring::Ring;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial<R: Ring, V: Variable, O: Order<Var = V> = Lex<V>> {
    pub(crate) monomials: BTreeMap<O, R>,
}

impl<R: Ring, V: Variable, O: Order<Var = V>> Degree for Polynomial<R, V, O> {
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

impl<R: Ring, V: Variable, O: Order<Var = V>> HeadMonomial<R, V> for Polynomial<R, V, O> {
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

impl<R: Ring, V: Variable, O: Order<Var = V>> Default for Polynomial<R, V, O> {
    fn default() -> Self {
        Self {
            monomials: Default::default(),
        }
    }
}

impl<B: Borrow<Monomial<R, V>>, R: Ring, V: Variable, O: Order<Var = V>> FromIterator<B>
    for Polynomial<R, V, O>
{
    fn from_iter<T: IntoIterator<Item = B>>(iter: T) -> Self {
        let mut monomials: BTreeMap<O, R> = Default::default();

        for borrowed in iter {
            let mono: &Monomial<R, V> = borrowed.borrow();
            let term: O = mono.term.clone().into();
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

impl<R: Ring, V: Variable, O: Order<Var = V>> IntoIterator for Polynomial<R, V, O> {
    type Item = Monomial<R, V>;

    type IntoIter = MonomialIter<IntoIter<O, R>, O, R>;

    fn into_iter(self) -> Self::IntoIter {
        MonomialIter {
            iter: self.monomials.into_iter(),
        }
    }
}

pub struct MonomialIter<I: Iterator<Item = (O, R)>, O: Order, R: Ring> {
    iter: I,
}

impl<I: Iterator<Item = (O, R)>, O: Order, R: Ring> Iterator for MonomialIter<I, O, R> {
    type Item = Monomial<R, O::Var>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(term, coeff)| Monomial {
            coeff: coeff,
            term: (*term).clone(),
        })
    }
}

impl<R: Ring, V: Variable, O: Order<Var = V>> Mul<Monomial<R, V>> for Polynomial<R, V, O> {
    type Output = Self;

    fn mul(self, rhs: Monomial<R, V>) -> Self::Output {
        let mut monomial: BTreeMap<O, R> = Default::default();
        for x in self.into_iter() {
            let result = x.mul(rhs.clone());
            monomial.insert(result.term.into(), result.coeff);
        }

        Polynomial {
            monomials: monomial,
        }
    }
}

impl<'a, R: Ring, V: Variable, O: Order<Var = V>> Mul<Monomial<R, V>> for &'a Polynomial<R, V, O> {
    type Output = Polynomial<R, V, O>;

    fn mul(self, rhs: Monomial<R, V>) -> Self::Output {
        let mut monomial: BTreeMap<O, R> = Default::default();
        for x in self.iter() {
            let result = x.mul(rhs.clone());
            monomial.insert(result.term.into(), result.coeff);
        }

        Polynomial {
            monomials: monomial,
        }
    }
}

impl<'a, R: Ring, V: Variable, O: Order<Var = V>> Mul<&'a Polynomial<R, V, O>> for Monomial<R, V> {
    type Output = Polynomial<R, V, O>;

    fn mul(self, rhs: &'a Polynomial<R, V, O>) -> Self::Output {
        let mut monomial: BTreeMap<O, R> = Default::default();
        for x in rhs.clone().into_iter() {
            let result = x.mul(self.clone());
            monomial.insert(result.term.into(), result.coeff);
        }

        Polynomial {
            monomials: monomial,
        }
    }
}

impl<R: Ring, V: Variable, O: Order<Var = V>> Add<Self> for Polynomial<R, V, O> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter().chain(rhs.into_iter()).collect()
    }
}

impl<R: Ring, V: Variable, O: Order<Var = V>> Sub<Self> for Polynomial<R, V, O> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .chain(rhs.into_iter().map(|mono| Monomial {
                coeff: mono.coeff.neg(),
                term: mono.term,
            }))
            .collect()
    }
}

impl<R: Ring, V: Variable, O: Order<Var = V>, T: Borrow<Polynomial<R, V, O>>> Mul<T>
    for Polynomial<R, V, O>
{
    type Output = Polynomial<R, V, O>;

    fn mul(self, rhs: T) -> Self::Output {
        let right: &Polynomial<R, V, O> = rhs.borrow();
        let mut monomials: Vec<Monomial<R, V>> = Default::default();
        for m_left in self.iter() {
            for m_right in right.iter() {
                monomials.push(m_right * &m_left);
            }
        }

        monomials.into_iter().collect()
    }
}

pub struct MonomialRefIter<'a, I: Iterator<Item = (&'a O, &'a R)>, O: Order + 'a, R: Ring + 'a> {
    iter: I,
}

impl<'a, I: Iterator<Item = (&'a O, &'a R)>, O: Order, R: Ring> Iterator
    for MonomialRefIter<'a, I, O, R>
{
    type Item = Monomial<R, O::Var>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(term, coeff)| Monomial {
            coeff: coeff.clone(),
            term: term.deref().clone(),
        })
    }
}

impl<R: Ring, V: Variable, O: Order<Var = V>> Polynomial<R, V, O> {
    pub fn div_euclid<T: Borrow<Self>>(&self, rhs: &T) -> (Polynomial<R, V, O>, Polynomial<R, V, O>)
    where
        R: Rem<R, Output = R> + Div<R, Output = R> + Display,
        Term<V>: Mul<R, Output = Monomial<R, V>>,
    {
        let mut f = self.clone();
        let div: &Polynomial<R, V, O> = rhs.borrow();

        let mut rem_monomial: Vec<Monomial<R, V>> = Default::default();

        loop {
            if (f.lead_coeff() % div.lead_coeff()) == R::zero() {
                let c = f.lead_coeff() / div.lead_coeff();
                if let Ok(m) = f.lead_term() / div.lead_term() {
                    rem_monomial.push(&m * c);
                    f = f - (div * (m * c));
                    continue;
                }
            }
            break;
        }

        (rem_monomial.into_iter().collect(), f)
    }

    pub fn iter(&self) -> MonomialRefIter<Iter<'_, O, R>, O, R> {
        MonomialRefIter {
            iter: self.monomials.iter(),
        }
    }
}

pub fn sploy<R: Ring, V: Variable, O: Order<Var = V>>(
    f: &Polynomial<R, V, O>,
    g: &Polynomial<R, V, O>,
) -> Polynomial<R, V, O>
where
    Term<V>: Mul<R, Output = Monomial<R, V>>,
{
    let m = lcm(&f.lead_term(), &g.lead_term());

    (&m / &f.lead_term()) * g.lead_coeff() * f - ((&m / &g.lead_term()) * f.lead_coeff()) * g
}

impl<R: Ring, V: Variable, O: Order<Var = V>> Display for Polynomial<R, V, O>
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

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, str::FromStr};

    use crate::order::Var;

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
        let f: Polynomial<i32, Var, Lex<Var>> =
            Polynomial::from_str("x^2+-3xy+2x^2y^3+y^2+2").unwrap();
        let g: Polynomial<i32, Var, Lex<Var>> = Polynomial::from_str("x+xy+x^2y+x^2+1").unwrap();

        let (q, r) = f.div_euclid(&g);

        assert!(Lex::<Var>::cmp(&r.lead_term().into(), &g.lead_term().into()) == Ordering::Less);
        assert_eq!(f, q * g + r);
    }
}
