use core::fmt;
use std::{fmt::Debug, fmt::Display, str::FromStr};

pub trait Variable: Copy + Clone + Debug + Display + PartialEq + Eq + Ord {}

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
