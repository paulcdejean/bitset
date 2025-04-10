use std::marker::PhantomData;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl};

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct BitSet<T, S = usize> {
    bits: S,
    phantom: PhantomData<T>,
}

impl<T: UnsignedNumber, S: UnsignedNumber> BitSet<T, S>
where
    T: Into<S>,
{
    pub fn new() -> BitSet<T, S> {
        BitSet {
            bits: 0.into(),
            phantom: PhantomData,
        }
    }
    pub fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }
    pub fn insert(&mut self, value: T) {
        match T::try_from(S::BITS) {
            Ok(v) => assert!(value >= 0.into() && value < v),
            Err(_) => panic!("S::BITS is greater than T::MAX"),
        }
        self.bits |= (T::from(1) << value).into();
    }
    pub fn contains(&self, n: T) -> bool {
        match T::try_from(S::BITS) {
            Ok(v) => {
                if n < 0.into() || n >= v {
                    false
                } else {
                    self.bits & (S::from(1) << n.into()) > 0.into()
                }
            }
            Err(_) => panic!("S::BITS is greater than T::MAX"),
        }
    }
    pub fn capacity(&self) -> usize {
        S::BITS as usize
    }
    pub fn clear(&mut self) {
        self.bits = 0.into();
    }
    pub fn difference(&self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
            phantom: PhantomData,
        }
    }
    pub fn get(&self, value: T) -> Option<T> {
        match self.contains(value) {
            true => Some(value),
            false => None,
        }
    }
    pub fn intersection(&self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
            phantom: PhantomData,
        }
    }
    pub fn is_disjoint(&self, other: Self) -> bool {
        self.bits & other.bits == 0.into()
    }
    pub fn is_empty(&self) -> bool {
        self.bits == 0.into()
    }
    pub fn is_subset(&self, other: Self) -> bool {
        self.bits & !other.bits == 0.into()
    }
    pub fn is_superset(&self, other: Self) -> bool {
        !self.bits & other.bits == 0.into()
    }
    pub fn remove(&mut self, value: T) {
        self.bits &= !(T::from(1) << value).into()
    }
}

pub trait UnsignedNumber:
    Copy
    + PartialOrd
    + Shl<Output = Self>
    + TryFrom<u32>
    + From<u8>
    + BitOrAssign
    + BitOr<Output = Self>
    + BitAndAssign
    + BitAnd<Output = Self>
    + Not<Output = Self>
{
    fn count_ones(self) -> u32;
    const BITS: u32;
}

macro_rules! unsigned_number_impl {
    ($num_type:ty) => {
        impl UnsignedNumber for $num_type {
            fn count_ones(self) -> u32 {
                self.count_ones()
            }
            const BITS: u32 = <$num_type>::BITS;
        }
    };
}

unsigned_number_impl!(usize);
unsigned_number_impl!(u8);
unsigned_number_impl!(u16);
unsigned_number_impl!(u32);
unsigned_number_impl!(u64);
unsigned_number_impl!(u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_seven_not_eight() {
        let mut bs: BitSet<usize> = BitSet::new();
        bs.insert(7);
        assert!(bs.contains(7));
        assert!(!bs.contains(8));
    }
}
