use std::fmt::Display;

use itertools::Itertools;

use crate::{order::Order, polynomial::Polynomial, ring::Ring, term::Term, variable::Variable};

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
