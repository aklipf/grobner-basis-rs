use std::ops::Mul;

use itertools::EitherOrBoth;

use super::join::JoinTerms;

pub trait Variable: Copy + Clone + PartialEq + Eq + Ord {}

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
            exps: iter.into_iter().collect(),
        }
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
            .map(|exps| match exps {
                EitherOrBoth::Both(x, y) => (x.0, x.1 * y.1),
                EitherOrBoth::Left(x) => x,
                EitherOrBoth::Right(x) => x,
            })
            .collect()
    }
}
