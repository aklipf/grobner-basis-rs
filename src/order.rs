use std::{cmp::Ordering, fmt, fmt::Display, ops::Deref};

use itertools::EitherOrBoth;

use crate::{
    join::JoinTerms,
    term::{Term, Variable},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Var(usize);

pub fn var(idx: usize) -> Var {
    Var(idx)
}

impl Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x_{}", self.0)
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

    use itertools::Itertools;

    use super::*;

    #[test]
    fn lex_order() {
        let mut terms: BTreeSet<Lex<Var>> = Default::default();
        terms.insert([(var(1), 2)].into_iter().collect::<Term<Var>>().into());
        terms.insert(
            [(var(1), 1), (var(2), 1)]
                .into_iter()
                .collect::<Term<Var>>()
                .into(),
        );
        terms.insert([(var(1), 1)].into_iter().collect::<Term<Var>>().into());
        terms.insert([(var(2), 2)].into_iter().collect::<Term<Var>>().into());
        terms.insert([(var(2), 1)].into_iter().collect::<Term<Var>>().into());

        assert_eq!(
            terms.into_iter().rev().map(|x| x.to_string()).join(" > "),
            "x_1^2 > x_1x_2 > x_1 > x_2^2 > x_2"
        );
    }
}
