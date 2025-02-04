use std::fmt::{Debug, Display};
use std::ops::{Div, Mul};

use super::exponent::{AddExponents, MaxExponents, SubExponents};

use super::join::JoinTerms;

pub trait Variable: Copy + Clone + Display + PartialEq + Eq + Ord {}

pub trait Degree {
    fn deg(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Term<V: Variable>
where
    (V, usize): Ord,
{
    exps: Vec<(V, usize)>,
}

impl<V: Variable> FromIterator<(V, usize)> for Term<V> {
    fn from_iter<T: IntoIterator<Item = (V, usize)>>(iter: T) -> Self {
        Self {
            exps: iter.into_iter().filter(|&(_, exp)| exp > 0).collect(),
        }
    }
}

impl<'a, V: Variable> FromIterator<&'a (V, usize)> for Term<V> {
    fn from_iter<T: IntoIterator<Item = &'a (V, usize)>>(iter: T) -> Self {
        Self {
            exps: iter
                .into_iter()
                .cloned()
                .filter(|&(_, exp)| exp > 0)
                .collect(),
        }
    }
}

impl<V: Variable> Display for Term<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &(var, exp) in self.exps.iter() {
            write!(f, "{}^{}", var, exp)?;
        }
        write!(f, "")
    }
}

impl<V: Variable> Degree for Term<V> {
    fn deg(&self) -> usize {
        self.exps.iter().map(|&(_, e)| e).product()
    }
}

impl<V: Variable> Mul<Self> for Term<V> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.exps
            .into_iter()
            .join_terms(rhs.exps.into_iter())
            .add_exponents()
            .collect()
    }
}

impl<V: Variable> Div<Self> for Term<V> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.exps
            .into_iter()
            .join_terms(rhs.exps.into_iter())
            .sub_exponents()
            .collect::<Result<Term<V>, String>>()
            .expect("Division error")
    }
}

impl<'a, 'b, V: Variable> Mul<&'b Term<V>> for &'a Term<V> {
    type Output = Term<V>;

    fn mul(self, rhs: &'b Term<V>) -> Term<V> {
        self.exps
            .iter()
            .join_terms(rhs.exps.iter())
            .add_exponents()
            .collect()
    }
}

impl<'a, 'b, V: Variable> Div<&'b Term<V>> for &'a Term<V> {
    type Output = Term<V>;

    fn div(self, rhs: &'b Term<V>) -> Term<V> {
        self.exps
            .iter()
            .join_terms(rhs.exps.iter())
            .sub_exponents()
            .collect::<Result<Term<V>, String>>()
            .expect("Division error")
    }
}

pub fn lcm<V: Variable>(left: &Term<V>, right: &Term<V>) -> Term<V> {
    left.exps
        .iter()
        .join_terms(right.exps.iter())
        .max_exponents()
        .collect()
}
