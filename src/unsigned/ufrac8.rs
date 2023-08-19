use std::{
    cmp::{min, Ordering},
    fmt::{Debug, Display},
};

use crate::{UFrac16, UFrac32};

/// A fraction defined along a binary tree.
///
/// 0s, 1, xs
///
/// `0b001x_xxxx`
///
/// `0b1xxx_xxxx`
#[derive(PartialEq, Eq, Default, Clone, Copy, Hash)]
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

impl PartialOrd for UFrac8 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UFrac8 {
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
                if self.0 & (1 << (rhs_precision)) == 0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            Ordering::Less => {
                #[cfg(test)]
                println!("rhs more specific");
                if other.0 & (1 << (lhs_precision)) == 0 {
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

impl UFrac8 {
    pub const ZERO: Self = Self(0);
    /// Lowest non-zero value represented by `UFrac8`; equal to 1/8
    pub const MIN: Self = Self(0b1000_0000);
    pub const ONE: Self = Self(1);
    /// The Golden Ratio approximated as a `UFrac8`; equal to 21/13 or 1.61538461538
    pub const GOLDEN_RATIO: Self = Self(0b0101_0101);
    /// Euler's Number approximated as a `UFrac8`; equal to 19/7 or 2.71428571429
    pub const E: Self = Self(0b0101_1011);
    /// Pi approximated as a `UFrac8`; equal to 16/5 or 3.2
    pub const PI: Self = Self(0b1000_0111);
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
            return (self.0, 1);
        }
        let mask = 0b1111_1111 >> (8 - precision);
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

    /// The inverse of a `UFrac8`. For any nonzero `UFrac8`, `self.invert().invert()` is guaranteed to be equal to `self`.
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

    /// The inverse of a `UFrac8`. If `self` is equal to `0`, returns `None`.
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
        7u8.saturating_sub(self.0.leading_zeros() as u8)
    }
}

impl TryFrom<u8> for UFrac8 {
    type Error = ();
    /// Try to create an integer `UFrac8`. Returns `Err(())` if passed a value greater than or equal to 9.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0 {
            Ok(Self::ZERO)
        } else if value == 8 {
            Ok(Self(u8::MAX))
        } else if value <= 7 {
            Ok(Self((1u8 << value).wrapping_sub(1)))
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

impl TryFrom<UFrac16> for UFrac8 {
    type Error = ();
    /// Try to fit a `UFrac16` into a `UFrac8`. Returns `Err(())` if passed a value with 8 or more bits of precision. If you would like to truncate the value instead, try `UFrac16::to_ufrac8_lossy`.
    fn try_from(value: UFrac16) -> Result<Self, Self::Error> {
        Ok(Self(u8::try_from(value.to_bits()).map_err(|_| ())?))
    }
}

impl TryFrom<UFrac32> for UFrac8 {
    type Error = ();
    /// Try to fit a `UFrac32` into a `UFrac8`. Returns `Err(())` if passed a value with 8 or more bits of precision. If you would like to truncate the value instead, try `UFrac32::to_ufrac8_lossy`.
    fn try_from(value: UFrac32) -> Result<Self, Self::Error> {
        Ok(Self(u8::try_from(value.to_bits()).map_err(|_| ())?))
    }
}