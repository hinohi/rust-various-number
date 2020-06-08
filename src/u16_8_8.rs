use core::ops::Add;

use num::traits::Zero;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct U16u8u8(pub u8, pub u8);

impl Add for U16u8u8 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let (n0, overflowed) = self.1.overflowing_add(rhs.1);
        let n1 = self.0 + rhs.0 + if overflowed { 1 } else { 0 };
        U16u8u8(n1, n0)
    }
}

impl Zero for U16u8u8 {
    fn zero() -> U16u8u8 {
        U16u8u8(0, 0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {}
}
