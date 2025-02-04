use std::ops::{Add, Deref, Mul, Neg, Rem, Sub};

use num_traits::{One, Zero};

pub trait Ring:
    Add<Self, Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Mul<Self, Output = Self>
    + One<Output = Self>
    + Zero<Output = Self>
    + Copy
    + Sized
{
}

pub trait Finite<const N: usize>: Ring {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Z<const N: usize, T>
where
    T: Rem<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<Output = T>
        + From<usize>
        + Zero
        + One
        + Default
        + Copy,
{
    value: T,
}

pub type Z2 = Z<2, usize>;

impl<const N: usize, T> Z<N, T>
where
    T: Rem<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<Output = T>
        + From<usize>
        + Zero
        + One
        + Default
        + Copy,
{
    fn module() -> T {
        T::from(N)
    }
}

impl<const N: usize, T> Deref for Z<N, T>
where
    T: Rem<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<Output = T>
        + From<usize>
        + Zero
        + One
        + Default
        + Copy,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<const N: usize, T> From<usize> for Z<N, T>
where
    T: Rem<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<Output = T>
        + From<usize>
        + Zero
        + One
        + Default
        + Copy,
{
    fn from(value: usize) -> Self {
        Self {
            value: T::from(value),
        }
    }
}

impl<const N: usize, T> Zero for Z<N, T>
where
    T: Rem<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<Output = T>
        + From<usize>
        + Zero
        + One
        + Default
        + Copy,
{
    fn zero() -> Self {
        Self { value: T::zero() }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl<const N: usize, T> One for Z<N, T>
where
    T: Rem<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<Output = T>
        + From<usize>
        + Zero
        + One
        + PartialEq
        + Default
        + Copy,
{
    fn one() -> Self {
        Self { value: T::one() }
    }

    fn is_one(&self) -> bool {
        self.value.is_one()
    }
}

impl<const N: usize, T> Mul<Self> for Z<N, T>
where
    T: Rem<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<Output = T>
        + From<usize>
        + Zero
        + One
        + Default
        + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: (self.value * rhs.value) % Self::module(),
        }
    }
}

impl<const N: usize, T> Add<Self> for Z<N, T>
where
    T: Rem<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<Output = T>
        + From<usize>
        + Zero
        + One
        + Default
        + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: (self.value + rhs.value) % Self::module(),
        }
    }
}

impl<const N: usize, T> Sub<Self> for Z<N, T>
where
    T: Rem<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<Output = T>
        + From<usize>
        + Zero
        + One
        + Default
        + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.neg())
    }
}

impl<const N: usize, T> Neg for Z<N, T>
where
    T: Rem<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<Output = T>
        + From<usize>
        + Zero
        + One
        + Default
        + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            value: Self::module() - self.value,
        }
    }
}

impl<const N: usize, T> Ring for Z<N, T> where
    T: Rem<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<Output = T>
        + From<usize>
        + Zero
        + One
        + PartialEq
        + Default
        + Copy
{
}

impl<const N: usize, T> Finite<N> for Z<N, T> where
    T: Rem<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<Output = T>
        + From<usize>
        + Zero
        + One
        + PartialEq
        + Default
        + Copy
{
}

mod tests {
    use super::*;

    #[test]
    fn finit_ring() {
        assert_eq!(Z::<5, usize>::module(), 5);
        assert_eq!(*Z::<5, usize>::from(3), 3);

        assert_eq!(*Z::<5, usize>::one(), 1);
        assert!(Z::<5, usize>::one().is_one());
        assert!(!Z::<5, usize>::one().is_zero());
        assert_eq!(*Z::<5, usize>::zero(), 0);
        assert!(Z::<5, usize>::zero().is_zero());
        assert!(!Z::<5, usize>::one().is_zero());

        assert_eq!(*Z::<5, usize>::from(3).neg(), 2);

        assert_eq!(*Z::<5, usize>::from(3).add(Z::<5, usize>::from(4)), 2);

        assert_eq!(*Z::<5, usize>::from(3).sub(Z::<5, usize>::from(4)), 4);
        assert_eq!(*Z::<5, usize>::from(4).sub(Z::<5, usize>::from(3)), 1);

        assert_eq!(*Z::<5, usize>::from(4).mul(Z::<5, usize>::from(3)), 2);
        assert_eq!(*Z::<5, usize>::from(2).mul(Z::<5, usize>::from(3)), 1);
        assert_eq!(*Z::<5, usize>::from(2).mul(Z::<5, usize>::from(2)), 4);
    }
}
