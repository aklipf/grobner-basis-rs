use std::{collections::BTreeMap, str::FromStr};

use regex::Regex;

use crate::{
    monomial::Monomial,
    order::{Order, OrderedTerm},
    polynomial::Polynomial,
    ring::Ring,
    term::Term,
    variable::Variable,
};

impl<V: Variable> FromStr for Term<V>
where
    V: FromStr,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_find_terms = Regex::new(r"[a-z](\^[0-9]+)?").unwrap();
        let re_parse_term = Regex::new(r"(?<var>[a-z])(?:\^(?<exp>[0-9]+))?").unwrap();

        let mut terms: Vec<(V, usize)> = Default::default();
        for term in re_find_terms.find_iter(s) {
            let parsed_term = re_parse_term.captures(term.as_str()).unwrap();
            let var = V::from_str(&parsed_term["var"]).or(Err("Invalid variable"))?;
            let exp: usize = parsed_term
                .name("exp")
                .map_or(Ok(1), |x| x.as_str().parse())
                .or(Err("Invalid exponent"))?;

            terms.push((var, exp));
        }

        for ((x, _), (y, _)) in terms.iter().skip(1).zip(terms.iter()) {
            if x <= y {
                return Err("Invalid term".to_owned());
            }
        }

        Ok(Term { exps: terms })
    }
}

impl<R: Ring + FromStr, V: Variable> FromStr for Monomial<R, V>
where
    V: FromStr,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_parse_mono =
            Regex::new(r"^\s*(?<coeff>(?:\-?\d+)|\-)?\s*(?<term>[\S\s]*)?").unwrap();

        if let Some(captured) = re_parse_mono.captures(s) {
            let term: Term<V> = captured
                .name("term")
                .map_or(Ok(Default::default()), |x| Term::from_str(x.as_str()))?;

            let coeff = captured
                .name("coeff")
                .map_or(Ok(R::one()), |x| match x.as_str() {
                    "-" => Ok(R::one().neg()),
                    s => s.parse(),
                })
                .or(Err("Cannot parse a coefficient"))?;

            Ok(Monomial {
                coeff: coeff,
                term: term,
            })
        } else {
            Err("Invalid monomial".to_owned())
        }
    }
}

impl<R: Ring, V: Variable, O: Order> FromStr for Polynomial<R, V, O>
where
    R: FromStr,
    V: FromStr,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_split_mono = Regex::new(r"\s*\+\s*").unwrap();

        let mut monomials: BTreeMap<OrderedTerm<V, O>, R> = Default::default();
        for captured in re_split_mono.split(s) {
            let monomial: Monomial<R, V> = Monomial::from_str(captured)?;
            monomials.insert(monomial.term.into(), monomial.coeff);
        }

        Ok(Polynomial {
            monomials: monomials,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::variable::Var;

    use super::*;

    #[test]
    fn parse_polynomial() {
        let f: Polynomial<i32, Var> = Polynomial::from_str("x^2+-3xy+2x^2y^3+y^2+2").unwrap();
        assert_eq!(f.to_string(), "2x²y³ + x² + -3xy + y² + 2");
    }
}
