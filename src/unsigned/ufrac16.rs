use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

use crate::{UFrac32, UFrac64, UFrac8};

/// A fraction defined along a binary tree.
/// up to 15 bits of data
/// `0b0001_xxxx_xxxx_xxxx`
/// `0b001x_xxxx_xxxx_xxxx`
#[derive(PartialEq, Eq, Default, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct UFrac16(u16);

impl Debug for UFrac16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>16b}", self.0)
    }
}

impl Display for UFrac16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (num, denom) = self.to_fraction();
        write!(f, "{num}/{denom}")
    }
}

impl UFrac16 {
    pub const ZERO: Self = Self(0);
    /// Lowest non-zero value represented by `UFrac16`; equal to 1/16
    pub const MIN: Self = Self(1);
    pub const ONE: Self = Self(0x8000);
    /// The Golden Ratio approximated as a `UFrac16`; equal to 987/610, or 1.61803278689
    pub const GOLDEN_RATIO: Self = Self(0xAAAA);
    /// Euler's Number approximated as a `UFrac16`; equal to 791/291, or 2.71821305842
    pub const E: Self = Self(0xDA15);
    /// Pi approximated as a `UFrac16`; equal to 204/65, or 3.13846153846
    pub const PI: Self = Self(0xE03D);
    /// Highest value represented by `UFrac16`; equal to 16
    pub const MAX: Self = Self(0xffff);

    /// Convert a `BTreeFraction` into two `u8`s representing the numerator and denominator. Infinity is represented by `(1, 0)`.
    ///
    /// # Panics
    ///
    /// Panics if the internal format is an invalid bit pattern. This should only happen if you manually set the bits.
    #[must_use]
    pub fn to_fraction(self) -> (u16, u16) {
        let precision = self.precision();
        #[cfg(test)]
        println!("{self:?}; precision = {precision}");
        if precision == 0 {
            return (self.0 >> 15, 1);
        }
        let mask = u16::MAX << (16 - precision);
        let masked_bits = self.0 & mask;
        #[cfg(test)]
        println!("{self:?} & {mask:0>16b} => {masked_bits:0>16b}");
        let mut lower_num = 0;
        let mut lower_denom = 1;
        let mut mid_num = 1;
        let mut mid_denom = 1;
        let mut upper_num = 1;
        let mut upper_denom = 0;
        for i in 0..(precision) {
            if masked_bits & (1 << (15 - i)) == 0 {
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

    /// Convert to a `UFrac8`. For values with 7 or fewer bits of precision, this conversion is lossless. For values with 8 or more bits of precision, this conversion truncates excess data.
    ///
    /// If you would like to limit this to a lossless conversion, try `UFrac8::try_from`.
    #[must_use]
    pub fn to_ufrac8_lossy(self) -> UFrac8 {
        if let Ok(frac8) = UFrac8::try_from(self) {
            return frac8;
        }
        #[allow(clippy::cast_possible_truncation)]
        UFrac8::from_bits(((self.0 & 0xff00) >> 8) as u8)
    }

    /// The inverse of a `UFrac16`. For any nonzero value, `self.invert().invert()` is guaranteed to be equal to `self`.
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

    /// The inverse of a `UFrac16`. If `self` is equal to `0`, returns `None`.
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

    /// The inverse of a `UFrac16`. If `self` is equal to `0`, returns `0`.
    #[must_use]
    pub const fn invert_unchecked(self) -> Self {
        Self(
            self.0
                ^ match u16::MAX.checked_shl(self.0.trailing_zeros() + 1) {
                    Some(mask) => mask,
                    None => 0,
                },
        )
    }

    /// Construct a `UFrac16` from a bit pattern
    #[must_use]
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    /// Get the bit pattern out of a `UFrac16`
    #[must_use]
    pub const fn to_bits(self) -> u16 {
        self.0
    }

    /// Get the precision of a value. This will be a value from 0 to 15 representing how many steps down the Farey tree the fraction is.
    /// If `self` is equal to `0` or `1`, this function will return `0`.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn precision(self) -> u16 {
        15u16.saturating_sub(self.0.trailing_zeros() as u16)
    }

    #[must_use]
    /// Get the fraction's parent node on the Farey tree. Returns `None` if `self` is 0 or 1.
    pub const fn parent(self) -> Option<Self> {
        let trailing_zeroes = self.0.trailing_zeros();
        if trailing_zeroes >= 15 {
            None
        } else {
            Some(Self(
                self.0 ^ (1 << trailing_zeroes) | (1 << (trailing_zeroes - 1)),
            ))
        }
    }

    #[must_use]
    /// Get the fraction's left child node on the Farey tree. Returns `None` if called on `0` or a value with 15 bits of precision.
    pub const fn left_child(self) -> Option<Self> {
        if self.0 == 0 || self.is_leaf() {
            None
        } else {
            let trailing_zeroes = self.0.trailing_zeros();
            Some(Self(
                self.0 & !(1 << trailing_zeroes) | (1 << (trailing_zeroes - 1)),
            ))
        }
    }

    #[must_use]
    /// Get the fraction's right child node on the Farey tree. Returns `None` if called on a `0` or a value with 15 bits of precision.
    pub const fn right_child(self) -> Option<Self> {
        if self.0 == 0 || self.is_leaf() {
            None
        } else {
            Some(Self(self.0 | (1 << (self.0.trailing_zeros() - 1))))
        }
    }

    #[must_use]
    /// Get the fraction's child nodes on the Farey tree. Returns `None` if called on `0` or a value with 15 bits of precision.
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
    /// Check if the value has the highest possible precision for `UFrac16`. If `true`, `left_child()` and `right_child()` will both return `None`.
    pub const fn is_leaf(self) -> bool {
        self.0 & 1 != 0
    }
}

impl TryFrom<u16> for UFrac16 {
    type Error = ();
    /// Try to create an integer `UFrac16`. Returns `Err(())` if passed a value greater than or equal to 17.
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Ok(Self::ZERO)
        } else if value <= 16 {
            Ok(Self(u16::MAX << (16 - value)))
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
impl TryFrom<f64> for UFrac16 {
    type Error = ();
    /// Try to create a `UFrac16` approximating a float. Returns `Err(())` if passed a negative or `NaN` value.
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value == 0.0 {
            #[cfg(test)]
            println!("{value} is 0");
            return Ok(Self::ZERO);
        } else if value.is_sign_negative() || value.is_nan() {
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
            if precision >= 15 {
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
                    steps += 1 << (15 - precision);
                }
            }
            precision += 1;
        }
        match (mid_num as f64).partial_cmp(&(value * mid_denom as f64)) {
            Some(Ordering::Greater) => Ok(Self(upper_steps | (1 << (15 - upper_precision)))),
            Some(Ordering::Equal) | None => Ok(Self(steps | (1 << (15 - precision)))),
            Some(Ordering::Less) => Ok(Self(lower_steps | (1 << (15 - lower_precision)))),
        }
    }
}

impl From<UFrac8> for UFrac16 {
    fn from(value: UFrac8) -> Self {
        Self(u16::from(value.to_bits()) << 8)
    }
}

impl TryFrom<UFrac64> for UFrac16 {
    type Error = ();
    fn try_from(value: UFrac64) -> Result<Self, Self::Error> {
        Ok(Self(u16::try_from(value.to_bits() >> 48).map_err(|_| ())?))
    }
}

impl TryFrom<UFrac32> for UFrac16 {
    type Error = ();
    fn try_from(value: UFrac32) -> Result<Self, Self::Error> {
        Ok(Self(u16::try_from(value.to_bits() >> 16).map_err(|_| ())?))
    }
}
