use std::fmt::{Debug, Display};
use std::ops::{Add, Deref, Mul, Neg, Sub};
use std::str::FromStr;

use num::{Integer, One, Zero};

pub trait Ring:
    Add<Self, Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Mul<Self, Output = Self>
    + One<Output = Self>
    + Zero<Output = Self>
    + Copy
    + Debug
    + PartialEq
    + Eq
    + Sized
{
}

impl<
        T: Add<Self, Output = Self>
            + Sub<Output = Self>
            + Neg<Output = Self>
            + Mul<Self, Output = Self>
            + One<Output = Self>
            + Zero<Output = Self>
            + Copy
            + Debug
            + PartialEq
            + Eq
            + Sized,
    > Ring for T
{
}

pub trait Mod<T: Integer> {
    const N: T;
}

macro_rules! static_finit_ring {
    ( $name:ident($mod:literal:$type:tt )) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $name($type);

        impl Mod<$type> for $name {
            const N: $type = $mod;
        }

        impl FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let Ok(value) = s.parse::<$type>() {
                    if value < $name::N {
                        return Ok($name(value));
                    }
                }
                Err("Invalid ring value".to_owned())
            }
        }

        impl Deref for $name {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Into<$type> for $name {
            fn into(self) -> $type {
                self.0
            }
        }

        impl Display for $name {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Zero for $name {
            fn zero() -> Self {
                Self($type::zero())
            }

            fn is_zero(&self) -> bool {
                self.0.is_zero()
            }
        }

        impl One for $name {
            fn one() -> Self {
                Self($type::one())
            }

            fn is_one(&self) -> bool {
                self.0.is_one()
            }
        }

        impl Add<$name> for $name {
            type Output = $name;

            #[inline]
            fn add(self, rhs: $name) -> Self::Output {
                Self((self.0 + rhs.0) % Self::N)
            }
        }

        impl Neg for $name {
            type Output = $name;

            #[inline]
            fn neg(self) -> Self::Output {
                Self(Self::N - self.0)
            }
        }

        impl Sub<$name> for $name {
            type Output = $name;

            #[inline]
            fn sub(self, rhs: $name) -> Self::Output {
                self.add(rhs.neg())
            }
        }

        impl Mul<$name> for $name {
            type Output = $name;

            #[inline]
            fn mul(self, rhs: $name) -> Self::Output {
                Self((self.0 * rhs.0) % Self::N)
            }
        }
    };
}

static_finit_ring!(Z2(2:u32));

#[cfg(test)]
mod tests {
    use super::*;

    static_finit_ring!(Z5(5:u32));

    #[test]
    fn finit_ring() {
        assert_eq!(Z5::N, 5);

        assert_eq!(Z5::one(), Z5(1));
        assert!(Z5::one().is_one());
        assert!(!Z5::one().is_zero());
        assert_eq!(Z5::zero(), Z5(0));
        assert!(Z5::zero().is_zero());
        assert!(!Z5::one().is_zero());

        assert_eq!(*-Z5(3), 2);

        assert_eq!(*(Z5(3) + Z5(4)), 2);

        assert_eq!(*(Z5(3) - Z5(4)), 4);
        assert_eq!(*(Z5(4) - Z5(3)), 1);

        assert_eq!(*(Z5(4) * Z5(3)), 2);
        assert_eq!(*(Z5(2) * Z5(3)), 1);
        assert_eq!(*(Z5(2) * Z5(2)), 4);
    }
}
