use std::{
    cmp::{min, Ordering},
    fmt::{Debug, Display},
};

// use crate::{UFrac16, UFrac32};

/// A signed fraction defined along a binary tree.
///
/// 0s, 1, xs
///
/// `0bs001_xxxx`
///
/// `0bs1xx_xxxx`
#[derive(PartialEq, Eq, Default, Clone, Copy, Hash)]
pub struct IFrac8(u8);

impl Debug for IFrac8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>8b}", self.0)
    }
}

impl Display for IFrac8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (num, denom) = self.to_fraction();
        write!(f, "{num}/{denom}")
    }
}

impl PartialOrd for IFrac8 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IFrac8 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lhs_precision = self.precision();
        let rhs_precision = other.precision();
        let lhs_positive = self.is_positive();
        let rhs_positive = other.is_positive();
        if lhs_positive && !rhs_positive {
            return Ordering::Greater;
        } else if !lhs_positive && rhs_positive {
            return Ordering::Less;
        }
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
                    if lhs_positive {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else if lhs_positive {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            Ordering::Less => {
                #[cfg(test)]
                println!("rhs more specific");
                if other.0 & (1 << (lhs_precision)) == 0 {
                    #[cfg(test)]
                    println!("it's a 0");
                    if lhs_positive {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                } else if lhs_positive {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
    }
}

impl IFrac8 {
    pub const ZERO: Self = Self(0);
    pub const MIN: Self = Self(0b0100_0000);
    pub const ONE: Self = Self(1);
    pub const GOLDEN_RATIO: Self = Self(0b0101_0101);
    pub const E: Self = Self(0b0101_1011);
    pub const PI: Self = Self(0b0100_0111);
    pub const MAX: Self = Self(0b0111_1111);

    /// Convert a `BTreeFraction` into two `u8`s representing the numerator and denominator. Infinity is represented by `(1, 0)`.
    ///
    /// # Panics
    ///
    /// Panics if the internal format is an invalid bit pattern. This should only happen if you manually set the bits.
    #[must_use]
    pub fn to_fraction(self) -> (i8, i8) {
        let precision = self.precision();
        #[cfg(test)]
        println!("{self:?}; precision = {precision}");
        if precision == 0 {
            // self.0 is either 0 or 1
            #[allow(clippy::cast_possible_wrap)]
            let num = self.abs().0 as i8;
            return (if self.is_positive() { num } else { -num }, 1);
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
        (
            if self.is_positive() {
                mid_num
            } else {
                -mid_num
            },
            mid_denom,
        )
    }

    #[must_use]
    pub const fn invert(self) -> Self {
        if self.0 == 0 {
            Self::MAX
        } else {
            let precision = self.precision();
            Self((1 << precision) | (!self.0 & ((1 << precision) - 1)))
        }
    }

    #[must_use]
    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    #[must_use]
    pub const fn to_bits(self) -> u8 {
        self.0
    }

    #[must_use]
    pub const fn is_positive(self) -> bool {
        self.0 >> 7 == 0
    }

    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.0 >> 7 == 1
    }

    #[must_use]
    pub const fn abs(self) -> Self {
        Self(self.0 & ((1 << 7) - 1))
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn precision(self) -> u8 {
        7u8.saturating_sub((self.0 & ((1 << 7) - 1)).leading_zeros() as u8)
    }
}

impl TryFrom<i8> for IFrac8 {
    type Error = ();
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let (value, is_negative) = (value.abs(), value.is_negative());
        if value == 0 {
            Ok(Self::ZERO)
        } else if value <= 6 {
            Ok(Self(
                (1u8 << value).wrapping_sub(1) | (u8::from(is_negative) << 7),
            ))
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
impl TryFrom<f64> for IFrac8 {
    type Error = ();
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let (value, is_negative) = (value.abs(), value.is_sign_negative());
        if value == 0.0 {
            #[cfg(test)]
            println!("{value} is 0");
            return Ok(Self::ZERO);
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
            if precision >= 6 {
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
            Some(Ordering::Greater) => Ok(Self(
                upper_steps | (1 << upper_precision) | (u8::from(is_negative) << 7),
            )),
            Some(Ordering::Equal) | None => Ok(Self(
                steps | (1 << precision) | (u8::from(is_negative) << 7),
            )),
            Some(Ordering::Less) => Ok(Self(
                lower_steps | (1 << lower_precision) | (u8::from(is_negative) << 7),
            )),
        }
    }
}

// impl TryFrom<UFrac16> for IFrac8 {
//     type Error = ();
//     fn try_from(value: UFrac16) -> Result<Self, Self::Error> {
//         Ok(Self(u8::try_from(value.to_bits()).map_err(|_| ())?))
//     }
// }

// impl TryFrom<UFrac32> for IFrac8 {
//     type Error = ();
//     fn try_from(value: UFrac32) -> Result<Self, Self::Error> {
//         Ok(Self(u8::try_from(value.to_bits()).map_err(|_| ())?))
//     }
// }
