use std::fmt::{Debug, Display};

use crate::variable::Variable;

use super::exponent::{AddExponents, MaxExponents, SubExponents};

use super::join::JoinTerms;

pub trait Degree {
    fn deg(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Term<V: Variable>
where
    (V, usize): Ord,
{
    pub exps: Vec<(V, usize)>,
}

impl<V: Variable> Default for Term<V> {
    fn default() -> Self {
        Self {
            exps: Default::default(),
        }
    }
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

fn number_to_superscript(c: char) -> char {
    match c {
        '0' => '\u{2070}',
        '1' => '\u{00B9}',
        '2' => '\u{00B2}',
        '3' => '\u{00B3}',
        '4' => '\u{2074}',
        '5' => '\u{2075}',
        '6' => '\u{2076}',
        '7' => '\u{2077}',
        '8' => '\u{2078}',
        '9' => '\u{2079}',
        _ => std::char::REPLACEMENT_CHARACTER,
    }
}

impl<V: Variable> Display for Term<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &(var, exp) in self.exps.iter() {
            if exp > 1 {
                write!(
                    f,
                    "{}{}",
                    var,
                    exp.to_string()
                        .chars()
                        .map(number_to_superscript)
                        .collect::<String>()
                )?;
            } else {
                write!(f, "{}", var)?;
            }
        }
        write!(f, "")
    }
}

impl<V: Variable> Degree for Term<V> {
    fn deg(&self) -> usize {
        self.exps.iter().map(|&(_, e)| e).product()
    }
}

#[inline]
pub(crate) fn mul_term_term<V: Variable>(left: &Term<V>, right: &Term<V>) -> Term<V> {
    left.exps
        .iter()
        .join_terms(right.exps.iter())
        .add_exponents()
        .collect()
}

#[inline]
pub(crate) fn div_term_term<V: Variable>(left: &Term<V>, right: &Term<V>) -> Option<Term<V>> {
    left.exps
        .iter()
        .join_terms(right.exps.iter())
        .sub_exponents()
        .collect::<Result<Term<V>, String>>()
        .ok()
}

pub fn lcm<V: Variable>(left: &Term<V>, right: &Term<V>) -> Term<V> {
    left.exps
        .iter()
        .join_terms(right.exps.iter())
        .max_exponents()
        .collect()
}
