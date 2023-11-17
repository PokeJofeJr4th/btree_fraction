use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

use crate::{UFrac16, UFrac32, UFrac64};

/// A fraction defined along a binary tree.
///
/// 0s, 1, xs
///
/// `0bxxxx_x100`
///
/// `0bxxxx_xxx1`
#[derive(PartialEq, Eq, Default, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct UFrac8(u8);

impl Debug for UFrac8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>8b}", self.0)
    }
}

impl Display for UFrac8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (num, denom) = self.to_fraction();
        write!(f, "{num}/{denom}")
    }
}

impl UFrac8 {
    pub const ZERO: Self = Self(0);
    /// Lowest non-zero value represented by `UFrac8`; equal to 1/8
    pub const MIN: Self = Self(1);
    pub const ONE: Self = Self(0b1000_0000);
    /// The Golden Ratio approximated as a `UFrac8`; equal to 21/13 or 1.61538461538
    pub const GOLDEN_RATIO: Self = Self(0b1010_1010);
    /// Euler's Number approximated as a `UFrac8`; equal to 19/7 or 2.71428571429
    pub const E: Self = Self(0b1101_1010);
    /// Pi approximated as a `UFrac8`; equal to 16/5 or 3.2
    pub const PI: Self = Self(0b1110_0001);
    /// Highest value represented by `UFrac8`; equal to 8
    pub const MAX: Self = Self(0b1111_1111);

    /// Convert a `UFrac8` into two `u8`s representing the numerator and denominator.
    #[must_use]
    pub fn to_fraction(self) -> (u8, u8) {
        let precision = self.precision();
        #[cfg(test)]
        println!("{self:?}; precision = {precision}");
        if precision == 0 {
            // self.0 is either 0 or 1
            return (self.0 >> 7, 1);
        }
        let mask = u8::MAX << (8 - precision);
        let masked_bits = self.0 & mask;
        #[cfg(test)]
        println!("{self:?} & {mask:0>8b} => {masked_bits:0>8b}");
        let mut lower_num = 0;
        let mut lower_denom = 1;
        let mut mid_num = 1;
        let mut mid_denom = 1;
        let mut upper_num = 1;
        let mut upper_denom = 0;
        for i in 0..(precision) {
            if masked_bits & (1 << (7 - i)) == 0 {
                upper_num = mid_num;
                upper_denom = mid_denom;
                mid_num += lower_num;
                mid_denom += lower_denom;
            } else {
                lower_num = mid_num;
                lower_denom = mid_denom;
                mid_num += upper_num;
                mid_denom += upper_denom;
            }
            #[cfg(test)]
            println!("{lower_num}/{lower_denom} < x < {upper_num}/{upper_denom}");
        }
        (mid_num, mid_denom)
    }

    /// The inverse of a `UFrac8`. For any nonzero `UFrac8`, `self.invert().invert()` is guaranteed to be equal to `self`.
    ///
    /// # Panics
    /// If `self` is equal to `0`
    #[must_use]
    pub const fn invert(self) -> Self {
        if self.0 == 0 {
            panic!("Can't invert `0/1`")
        } else {
            self.invert_unchecked()
        }
    }

    /// The inverse of a `UFrac8`. If `self` is equal to `0`, returns `None`.
    ///
    /// For any nonzero value, `self.invert().unwrap().invert().unwrap()` is guaranteed to be equal to `self`.
    #[must_use]
    pub const fn try_invert(self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            Some(self.invert_unchecked())
        }
    }

    /// The inverse of a `UFrac8`. If `self` is equal to `0`, returns `0`.
    #[must_use]
    pub const fn invert_unchecked(self) -> Self {
        Self(
            self.0
                ^ match u8::MAX.checked_shl(self.0.trailing_zeros() + 1) {
                    Some(mask) => mask,
                    None => 0,
                },
        )
    }

    /// Construct a `UFrac8` from a bit pattern.
    #[must_use]
    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    /// Get the bit pattern out of a `UFrac8`
    #[must_use]
    pub const fn to_bits(self) -> u8 {
        self.0
    }

    /// Get the precision of a `UFrac8`. This will be a value from 0 to 7 representing how many steps down the Farey tree the fraction is.
    /// If `self` is equal to `0` or `1`, this function will return `0`.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn precision(self) -> u8 {
        7u8.saturating_sub(self.0.trailing_zeros() as u8)
    }

    #[must_use]
    /// Get the fraction's parent node on the Farey tree. Returns `None` if `self` is 0 or 1.
    pub const fn parent(self) -> Option<Self> {
        let trailing_zeroes = self.0.trailing_zeros();
        if trailing_zeroes >= 7 {
            None
        } else {
            Some(Self(
                self.0 ^ (1 << trailing_zeroes) | (1 << (trailing_zeroes - 1)),
            ))
        }
    }

    #[must_use]
    /// Get the fraction's left child node on the Farey tree. Returns `None` if called on `0` or a value with 7 bits of precision.
    pub const fn left_child(self) -> Option<Self> {
        if self.0 == 0 || self.is_leaf() {
            None
        } else {
            Some(Self(
                self.0 & !(1 << self.0.trailing_zeros()) | (1 << (self.0.trailing_zeros() - 1)),
            ))
        }
    }

    #[must_use]
    /// Get the fraction's right child node on the Farey tree. Returns `None` if called on a `0` or a value with 7 bits of precision.
    pub const fn right_child(self) -> Option<Self> {
        if self.0 == 0 || self.is_leaf() {
            None
        } else {
            Some(Self(self.0 | (1 << (self.0.trailing_zeros() - 1))))
        }
    }

    #[must_use]
    /// Get the fraction's child nodes on the Farey tree. Returns `None` if called on `0` or a value with 7 bits of precision.
    ///
    /// Equivalent to `(self.left_child()?,self.right_child()?)`
    pub const fn children(self) -> Option<(Self, Self)> {
        if self.0 == 0 || self.is_leaf() {
            None
        } else {
            let right_child = self.0 | (1 << (self.0.trailing_zeros() - 1));
            Some((
                Self(right_child & !(1 << self.0.trailing_zeros())),
                Self(right_child),
            ))
        }
    }

    #[must_use]
    /// Get the other child of the fraction's parent node on the Farey tree. Returns `None` if called on `0` or `1`.
    pub const fn sibling(self) -> Option<Self> {
        if self.0 == Self::ZERO.0 || self.0 == Self::ONE.0 {
            None
        } else {
            Some(Self(self.0 ^ (1 << (self.0.trailing_zeros() - 1))))
        }
    }

    #[must_use]
    /// Get the other child of the fraction's parent node on the Farey tree. Behavior is undefined if called on `0` or `1`.
    pub const fn sibling_unchecked(self) -> Self {
        Self(self.0 ^ (1 << (self.0.trailing_zeros() - 1)))
    }

    #[must_use]
    /// Check if the value has the highest possible precision for `UFrac8`. If `true`, `left_child()` and `right_child()` will both return `None`.
    pub const fn is_leaf(self) -> bool {
        self.0 & 1 != 0
    }
}

impl TryFrom<u8> for UFrac8 {
    type Error = ();
    /// Try to create an integer `UFrac8`. Returns `Err(())` if passed a value greater than or equal to 9.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0 {
            Ok(Self::ZERO)
        } else if value <= 8 {
            Ok(Self(u8::MAX << (8 - value)))
        } else {
            Err(())
        }
    }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_precision_loss
)]
impl TryFrom<f64> for UFrac8 {
    type Error = ();
    /// Try to create a `UFrac8` approximating a float. Returns `Err(())` if passed a negative or `NaN` value.
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value == 0.0 {
            #[cfg(test)]
            println!("{value} is 0");
            return Ok(Self::ZERO);
        } else if value.is_infinite() || value.is_nan() {
            return Err(());
        }
        let mut lower_num = 0;
        let mut lower_denom = 1;
        let mut lower_steps = 0;
        let mut lower_precision = 0;
        let mut mid_num = 1;
        let mut mid_denom = 1;
        let mut upper_num = 1;
        let mut upper_denom = 0;
        let mut upper_steps = 0;
        let mut upper_precision = 0;
        let mut precision = 0;
        let mut steps = 0;
        loop {
            if precision >= 7 {
                break;
            }
            match (mid_num as f64).partial_cmp(&(value * mid_denom as f64)) {
                Some(Ordering::Greater) => {
                    #[cfg(test)]
                    println!("{lower_num}/{lower_denom} < {value} < {mid_num}/{mid_denom}");
                    upper_num = mid_num;
                    upper_denom = mid_denom;
                    upper_steps = steps;
                    upper_precision = precision + 1;
                    mid_num += lower_num;
                    mid_denom += lower_denom;
                }
                Some(Ordering::Equal) | None => break,
                Some(Ordering::Less) => {
                    #[cfg(test)]
                    println!("{mid_num}/{mid_denom} < {value} < {upper_num}/{upper_denom}");
                    lower_num = mid_num;
                    lower_denom = mid_denom;
                    lower_steps = steps;
                    lower_precision = precision + 1;
                    mid_num += upper_num;
                    mid_denom += upper_denom;
                    steps |= 1 << (7 - precision);
                }
            }
            precision += 1;
        }
        match (mid_num as f64).partial_cmp(&(value * mid_denom as f64)) {
            Some(Ordering::Greater) => Ok(Self(upper_steps | (1 << (7 - upper_precision)))),
            Some(Ordering::Equal) | None => Ok(Self(steps | (1 << (7 - precision)))),
            Some(Ordering::Less) => Ok(Self(lower_steps | (1 << (7 - lower_precision)))),
        }
    }
}

impl TryFrom<UFrac16> for UFrac8 {
    type Error = ();
    /// Try to fit a `UFrac16` into a `UFrac8`. Returns `Err(())` if passed a value with 8 or more bits of precision. If you would like to truncate the value instead, try `UFrac16::to_ufrac8_lossy`.
    fn try_from(value: UFrac16) -> Result<Self, Self::Error> {
        Ok(Self(u8::try_from(value.to_bits() >> 8).map_err(|_| ())?))
    }
}

impl TryFrom<UFrac32> for UFrac8 {
    type Error = ();
    /// Try to fit a `UFrac32` into a `UFrac8`. Returns `Err(())` if passed a value with 8 or more bits of precision. If you would like to truncate the value instead, try `UFrac32::to_ufrac8_lossy`.
    fn try_from(value: UFrac32) -> Result<Self, Self::Error> {
        Ok(Self(u8::try_from(value.to_bits() >> 24).map_err(|_| ())?))
    }
}

impl TryFrom<UFrac64> for UFrac8 {
    type Error = ();
    /// Try to fit a `UFrac64` into a `UFrac8`. Returns `Err(())` if passed a value with 8 or more bits of precision. If you would like to truncate the value instead, try `UFrac64::to_ufrac8_lossy`.
    fn try_from(value: UFrac64) -> Result<Self, Self::Error> {
        Ok(Self(u8::try_from(value.to_bits() >> 56).map_err(|_| ())?))
    }
}
