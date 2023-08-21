use std::{
    cmp::{min, Ordering},
    fmt::{Debug, Display},
};

use crate::{UFrac16, UFrac8};

/// A fraction defined along a binary tree.
/// up to 31 bits of data
/// `0b0000_1xxx_xxxx_xxxx_xxxx_xxxx_xxxx_xxxx`
/// `0b0001_xxxx_xxxx_xxxx_xxxx_xxxx_xxxx_xxxx`
#[derive(PartialEq, Eq, Default, Clone, Copy, Hash)]
pub struct UFrac32(u32);

impl Debug for UFrac32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>32b}", self.0)
    }
}

impl Display for UFrac32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (num, denom) = self.to_fraction();
        write!(f, "{num}/{denom}")
    }
}

impl PartialOrd for UFrac32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UFrac32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lhs_precision = self.precision();
        let rhs_precision = other.precision();
        #[cfg(test)]
        println!("precision: {lhs_precision}, {rhs_precision}");
        for i in 0..min(lhs_precision, rhs_precision) {
            match (self.0 & (1 << i)).cmp(&(other.0 & (1 << i))) {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => {}
            }
        }
        #[cfg(test)]
        println!("Switching to difficult comparison");
        match lhs_precision.cmp(&rhs_precision) {
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => {
                if self.0 & (1 << rhs_precision) == 0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            Ordering::Less => {
                #[cfg(test)]
                println!("rhs more specific");
                if other.0 & (1 << lhs_precision) == 0 {
                    #[cfg(test)]
                    println!("it's a 0");
                    Ordering::Greater
                } else {
                    #[cfg(test)]
                    println!("it's a 1");
                    Ordering::Less
                }
            }
        }
    }
}

impl UFrac32 {
    pub const ZERO: Self = Self(0);
    /// Smallest non-zero value represented by `UFrac32`; equal to 1/32
    pub const MIN: Self = Self(0x8000_0000);
    pub const ONE: Self = Self(1);
    /// The Golden Ratio approximated as a `UFrac32`; equal to 2178309/1346269 or 1.61803398875
    pub const GOLDEN_RATIO: Self = Self(0x5555_5555);
    /// Euler's Number approximated as a `UFrac32`; equal to 28667/10546 or 2.71828181301
    pub const E: Self = Self(0x4017_E85B);
    /// Pi approximated as a `UFrac32`; equal to 3861/1229 or 3.14157851912
    pub const PI: Self = Self(0xe5ff_fc07);
    /// Highest value represented by `UFrac32`; equal to 32
    pub const MAX: Self = Self(0xffff_ffff);

    /// Convert a `BTreeFraction` into two `u8`s representing the numerator and denominator. Infinity is represented by `(1, 0)`.
    ///
    /// # Panics
    ///
    /// Panics if the internal format is an invalid bit pattern. This should only happen if you manually set the bits.
    #[must_use]
    pub fn to_fraction(self) -> (u32, u32) {
        let precision = self.precision();
        #[cfg(test)]
        println!("{self:?}; precision = {precision}");
        if precision == 0 {
            return (self.0, 1);
        }
        let mask = u32::MAX >> (32 - precision);
        let masked_bits = self.0 & mask;
        #[cfg(test)]
        println!("{self:?} & {mask:0>32b} => {masked_bits:0>32b}");
        let mut lower_num = 0;
        let mut lower_denom = 1;
        let mut mid_num = 1;
        let mut mid_denom = 1;
        let mut upper_num = 1;
        let mut upper_denom = 0;
        for i in 0..(precision) {
            if masked_bits & (1 << i) == 0 {
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
        UFrac8::from_bits((self.0 & 0x0000_00ff | 0x000_0080) as u8)
    }

    #[must_use]
    /// Convert to a `UFrac16`. For values with 15 or fewer bits of precision, this conversion is lossless. For values with 8 or more bits of precision, this conversion truncates excess data.
    ///
    /// If you would like to limit this to a lossless conversion, try `UFrac8::try_from`.
    pub fn to_ufrac16_lossy(self) -> UFrac16 {
        if let Ok(frac8) = UFrac16::try_from(self) {
            return frac8;
        }
        #[allow(clippy::cast_possible_truncation)]
        UFrac16::from_bits((self.0 & 0x0000_ffff | 0x0000_8000) as u16)
    }

    /// The inverse of a `UFrac32`. For any nonzero value, `self.invert().invert()` is guaranteed to be equal to `self`.
    ///
    /// # Panics
    /// If `self` is equal to `0`
    #[must_use]
    pub const fn invert(self) -> Self {
        if self.0 == 0 {
            panic!("Can't invert `0/1`")
        } else {
            Self(self.0 ^ ((1 << self.precision()) - 1))
        }
    }

    /// The inverse of a `UFrac32`. If `self` is equal to `0`, returns `None`.
    ///
    /// For any nonzero value, `self.invert().unwrap().invert().unwrap()` is guaranteed to be equal to `self`.
    #[must_use]
    pub const fn try_invert(self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            Some(Self(self.0 ^ ((1 << self.precision()) - 1)))
        }
    }

    /// The inverse of a `UFrac32`. If `self` is equal to `0`, returns `0`.
    #[must_use]
    pub const fn invert_unchecked(self) -> Self {
        Self(self.0 ^ ((1 << self.precision()) - 1))
    }

    /// Construct a `UFrac32` from a bit pattern
    #[must_use]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Get the bit pattern out of a `UFrac32`
    #[must_use]
    pub const fn to_bits(self) -> u32 {
        self.0
    }

    /// Get the precision of a value. This will be a value from 0 to 31 representing how many steps down the Farey tree the fraction is.
    /// If `self` is equal to `0` or `1`, this function will return `0`.
    #[must_use]
    pub const fn precision(self) -> u32 {
        31u32.saturating_sub(self.0.leading_zeros())
    }

    #[must_use]
    /// Get the fraction's parent node on the Farey tree. Returns `None` if `self` is 0 or 1.
    pub const fn parent(self) -> Option<Self> {
        let precision = self.precision();
        if precision == 0 {
            None
        } else {
            Some(Self(self.0 ^ (1 << precision) | (1 << (precision - 1))))
        }
    }

    #[must_use]
    /// Get the fraction's left child node on the Farey tree. Returns `None` if called on `0` or a value with 31 bits of precision.
    pub const fn left_child(self) -> Option<Self> {
        if self.0 == 0 || self.is_leaf() {
            None
        } else {
            let precision = self.precision();
            Some(Self(self.0 & !(1 << precision) | (1 << (precision + 1))))
        }
    }

    #[must_use]
    /// Get the fraction's right child node on the Farey tree. Returns `None` if called on a `0` or a value with 31 bits of precision.
    pub const fn right_child(self) -> Option<Self> {
        if self.0 == 0 || self.is_leaf() {
            None
        } else {
            Some(Self(self.0 | (1 << (self.precision() + 1))))
        }
    }

    #[must_use]
    /// Check if the value has the highest possible precision for `UFrac32`. If `true`, `left_child()` and `right_child()` will both return `None`.
    pub const fn is_leaf(self) -> bool {
        self.0 & (1 << 31) != 0
    }
}

impl TryFrom<u32> for UFrac32 {
    type Error = ();
    /// Try to create an integer `UFrac32`. Returns `Err(())` if passed a value greater than or equal to 33.
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            Ok(Self::ZERO)
        } else if value == 32 {
            Ok(Self(u32::MAX))
        } else if value <= 31 {
            Ok(Self((1u32 << value).wrapping_sub(1)))
        } else {
            Err(())
        }
    }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_lossless
)]
impl TryFrom<f64> for UFrac32 {
    type Error = ();
    /// Try to create a `UFrac32` approximating a float. Returns `Err(())` if passed a negative or `NaN` value.
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
            if precision >= 31 {
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
                    steps += 1 << precision;
                }
            }
            precision += 1;
        }
        match (mid_num as f64).partial_cmp(&(value * mid_denom as f64)) {
            Some(Ordering::Greater) => Ok(Self(upper_steps | (1 << upper_precision))),
            Some(Ordering::Equal) | None => Ok(Self(steps | (1 << precision))),
            Some(Ordering::Less) => Ok(Self(lower_steps | (1 << lower_precision))),
        }
    }
}

impl From<UFrac8> for UFrac32 {
    fn from(value: UFrac8) -> Self {
        Self(u32::from(value.to_bits()))
    }
}

impl From<UFrac16> for UFrac32 {
    fn from(value: UFrac16) -> Self {
        Self(u32::from(value.to_bits()))
    }
}
