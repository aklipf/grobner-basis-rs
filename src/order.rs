use std::{cmp::Ordering, marker::PhantomData, ops::Deref};

use itertools::EitherOrBoth;

use crate::{
    join::JoinTerms,
    term::{Degree, Term},
    variable::Variable,
};

pub trait Order {
    fn cmp<V: Variable>(a: &Term<V>, b: &Term<V>) -> Ordering;
}

#[macro_export]
macro_rules! order_from_terms {
    ( $order_name:tt , $term_order:tt ) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub struct $order_name {}

        impl Order for $order_name {
            fn cmp<V: Variable>(left: &Term<V>, right: &Term<V>) -> std::cmp::Ordering {
                left.exps
                    .iter()
                    .join_terms(right.exps.iter())
                    .find_map($term_order)
                    .unwrap_or(Ordering::Equal)
            }
        }
    };
}

fn lex_variables<V: Variable>(terms: EitherOrBoth<&(V, usize)>) -> Option<Ordering> {
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

order_from_terms!(Lex, lex_variables);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GradLex {}

impl Order for GradLex {
    fn cmp<V: Variable>(left: &Term<V>, right: &Term<V>) -> std::cmp::Ordering {
        match left.deg().cmp(&right.deg()) {
            Ordering::Equal => Lex::cmp(left, right),
            c => c,
        }
    }
}

#[derive(Debug)]
pub struct OrderedTerm<V: Variable, O: Order> {
    terms: Term<V>,
    cmp_fn: PhantomData<O>,
}

impl<V: Variable, O: Order> Clone for OrderedTerm<V, O> {
    fn clone(&self) -> Self {
        Self {
            terms: self.terms.clone(),
            cmp_fn: PhantomData,
        }
    }
}

impl<V: Variable, O: Order> Deref for OrderedTerm<V, O> {
    type Target = Term<V>;

    fn deref(&self) -> &Self::Target {
        &self.terms
    }
}

impl<V: Variable, O: Order> From<Term<V>> for OrderedTerm<V, O> {
    fn from(value: Term<V>) -> Self {
        OrderedTerm {
            terms: value,
            cmp_fn: PhantomData,
        }
    }
}

impl<V: Variable, O: Order> Ord for OrderedTerm<V, O> {
    fn cmp(&self, other: &Self) -> Ordering {
        O::cmp(&self.terms, &other.terms)
    }
}

impl<V: Variable, O: Order> PartialOrd for OrderedTerm<V, O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V: Variable, O: Order> PartialEq for OrderedTerm<V, O> {
    fn eq(&self, other: &Self) -> bool {
        O::cmp(&self.terms, &other.terms) == Ordering::Equal
    }
}

impl<V: Variable, O: Order> Eq for OrderedTerm<V, O> {}

#[cfg(test)]
mod tests {
    use crate::variable::Var;
    use std::str::FromStr;

    use super::*;

    fn take_order<O: Order>() {}

    #[test]
    fn test_lex_order() {
        take_order::<Lex>();

        let a2 = Term::from_str("a^2").unwrap();
        let ab = Term::from_str("ab").unwrap();
        let a = Term::from_str("a").unwrap();
        let b2 = Term::from_str("b^2").unwrap();
        let b = Term::from_str("b").unwrap();

        let mut terms: Vec<Term<Var>> =
            vec![a.clone(), b2.clone(), a2.clone(), b.clone(), ab.clone()];
        terms.sort_by(Lex::cmp);

        assert_eq!(terms, vec![b, b2, a, ab, a2]);
    }
}
