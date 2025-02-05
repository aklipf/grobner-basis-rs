use std::{
    cmp::Ordering,
    fmt::{self, Display},
    ops::Deref,
    str::FromStr,
};

use itertools::EitherOrBoth;

use crate::{
    join::JoinTerms,
    term::{Term, Variable},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Var(pub usize);

pub fn var(idx: usize) -> Var {
    Var(idx)
}

impl FromStr for Var {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let var = s.chars().next().ok_or("Cannot parse variable")?;
        let var_id = (var as usize) - 0x61;
        if var_id < 26 {
            Ok(Var(var_id))
        } else {
            Err("Cannot parse variable".to_owned())
        }
    }
}

#[macro_export]
macro_rules! var {
    ( x ) => {
        Var(23)
    };
    ( y ) => {
        Var(24)
    };
    ( z ) => {
        Var(25)
    };
}

#[allow(dead_code)]
fn number_to_subscript(c: char) -> char {
    match c {
        '0' => '\u{2080}',
        '1' => '\u{2081}',
        '2' => '\u{2082}',
        '3' => '\u{2083}',
        '4' => '\u{2084}',
        '5' => '\u{2085}',
        '6' => '\u{2086}',
        '7' => '\u{2087}',
        '8' => '\u{2088}',
        '9' => '\u{2089}',
        _ => std::char::REPLACEMENT_CHARACTER,
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = (self.0 + 1) as u32;
        let mut encoded = "".to_string();

        while current > 0 {
            let r = (current - 1) % 26;
            current = (current - 1) / 26;
            encoded.push(unsafe { std::char::from_u32_unchecked(r + 0x61) });
        }
        write!(f, "{}", encoded.chars().rev().collect::<String>())
    }
}

impl Variable for Var {}
pub trait OrderCmp<V: Variable> {
    fn cmp_order(terms: EitherOrBoth<&(V, usize)>) -> Option<Ordering>;
}
pub trait Order:
    Clone + Deref<Target = Term<Self::Var>> + From<Term<Self::Var>> + Ord + OrderCmp<Self::Var>
{
    type Var: Variable;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderedTerm<V: Variable> {
    terms: Term<V>,
}

impl<V: Variable> Deref for OrderedTerm<V> {
    type Target = Term<V>;

    fn deref(&self) -> &Self::Target {
        &self.terms
    }
}

impl<V: Variable> From<Term<V>> for OrderedTerm<V> {
    fn from(value: Term<V>) -> Self {
        OrderedTerm { terms: value }
    }
}

impl<V: Variable> Ord for OrderedTerm<V>
where
    Self: OrderCmp<V>,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.terms
            .exps
            .iter()
            .join_terms(other.terms.exps.iter())
            .find_map(Self::cmp_order)
            .unwrap_or(Ordering::Equal)
    }
}

impl<V: Variable> PartialOrd for OrderedTerm<V>
where
    Self: OrderCmp<V>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub type Lex<V> = OrderedTerm<V>;

impl<V: Variable> Order for Lex<V> {
    type Var = V;
}

impl<V: Variable> OrderCmp<V> for Lex<V> {
    fn cmp_order(terms: EitherOrBoth<&(V, usize)>) -> Option<Ordering> {
        match terms {
            EitherOrBoth::Both(&(_, left), &(_, right)) => {
                if left < right {
                    Some(Ordering::Less)
                } else if left > right {
                    Some(Ordering::Greater)
                } else {
                    None
                }
            }
            EitherOrBoth::Left(_) => Some(Ordering::Greater),
            EitherOrBoth::Right(_) => Some(Ordering::Less),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn lex_order() {
        let mut terms: BTreeSet<Lex<Var>> = Default::default();
        let a2 = Term::from_str("a^2").unwrap();
        let ab = Term::from_str("ab").unwrap();
        let a = Term::from_str("a").unwrap();
        let b2 = Term::from_str("b^2").unwrap();
        let b = Term::from_str("b").unwrap();
        terms.insert(a2.clone().into());
        terms.insert(ab.clone().into());
        terms.insert(a.clone().into());
        terms.insert(b2.clone().into());
        terms.insert(b.clone().into());

        assert_eq!(
            terms
                .into_iter()
                .rev()
                .map(|t| t.terms)
                .collect::<Vec<Term<Var>>>(),
            vec![a2, ab, a, b2, b]
        );
    }
}
