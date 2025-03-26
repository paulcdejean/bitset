#![allow(warnings)]

use std::marker::PhantomData;
use std::ops::{BitAnd, BitOrAssign, Shl};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct BitSet<T, S = usize> {
    bits: S,
    phantom: PhantomData<T>,
}

impl<S: UnsignedNumber, T: UnsignedNumber> BitSet<T, S>
where
    T: Into<S>,
{
    pub fn new() -> BitSet<T, S> {
        return BitSet {
            bits: 0.into(),
            phantom: PhantomData,
        };
    }
    pub fn len(&self) -> usize {
        return self.bits.count_ones() as usize;
    }
    pub fn insert(&mut self, n: T) {
        match T::try_from(S::BITS) {
            Ok(v) => assert!(n >= 0.into() && n < v),
            Err(e) => panic!("S::BITS is greater than T::MAX"),
        }
        self.bits |= (T::from(1) << n).into();
    }
    pub fn contains(&self, n: T) -> bool {
        match T::try_from(S::BITS) {
            Ok(v) => {
                if n < 0.into() || n >= v {
                    return false;
                } else {
                    return self.bits & (S::from(1) << n.into()) > 0.into();
                }
            }
            Err(e) => panic!("S::BITS is greater than T::MAX"),
        }
    }
}

pub trait UnsignedNumber:
    Copy
    + PartialOrd
    + Sized
    + Shl<Output = Self>
    + TryFrom<u32>
    + From<u8>
    + BitOrAssign
    + BitAnd<Output = Self>
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
    fn has_seven() {
        let mut bs: BitSet<usize> = BitSet::new();
        bs.insert(7);
        assert!(bs.contains(7));
        assert!(!bs.contains(8));
    }
}
