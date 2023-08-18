use std::{
    cmp::{min, Ordering},
    fmt::{Debug, Display},
};

/// A fraction defined along a binary tree.
/// 4 bits of exponent, 13 bits of data
/// `0b1100_xxxx_xxxx_xxxx`
/// `0b111x_xxxx_xxxx_xxxx`
#[derive(PartialEq, Eq, Default, Clone, Copy)]
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

impl PartialOrd for UFrac16 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UFrac16 {
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

impl UFrac16 {
    pub const INFINITY: Self = Self(0x0fff);
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);

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
            if self.0 == 0 {
                return (0, 1);
            } else if self.0 == 1 {
                return (1, 1);
            } else if self.0 == 0b0000_1111_1111_1111 {
                return (1, 0);
            }
            panic!("Invalid bit pattern: {self:?}")
        }
        let mask = 0b1111_1111_1111_1111 >> (16 - precision);
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

    #[must_use]
    pub const fn invert(self) -> Self {
        if self.0 == 0 {
            Self(0b0000_1111_1111_1111)
        } else if self.0 == 1 {
            Self(1)
        } else {
            Self((self.0 & 0b1111_0000_0000_0000) + (!self.0 & ((1 << self.precision()) - 1)))
        }
    }

    #[must_use]
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    #[must_use]
    pub const fn to_bits(self) -> u16 {
        self.0
    }

    #[must_use]
    pub const fn precision(self) -> u16 {
        let val = self.0 >> 12;
        if val > 13 {
            13
        } else {
            val
        }
    }
}

impl TryFrom<u16> for UFrac16 {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Ok(Self(0))
        } else if value == 1 {
            Ok(Self(1))
        } else if value <= 13 {
            Ok(Self(((value - 1) << 12) + ((1 << (value - 1)) - 1)))
        } else {
            Err(())
        }
    }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
impl TryFrom<f32> for UFrac16 {
    type Error = ();
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value == 0.0 {
            #[cfg(test)]
            println!("{value} is 0");
            return Ok(Self(0));
        } else if (value - 1.0).abs() < f32::EPSILON {
            #[cfg(test)]
            println!("{value} is 1");
            return Ok(Self(1));
        } else if value.is_infinite() {
            #[cfg(test)]
            println!("{value} is infinite");
            return Ok(Self(0b0000_1111_1111_1111));
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
            if precision >= 13 {
                break;
            }
            match (mid_num as f32).partial_cmp(&(value * mid_denom as f32)) {
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
        match (mid_num as f32).partial_cmp(&(value * mid_denom as f32)) {
            Some(Ordering::Greater) => Ok(Self(upper_steps + (upper_precision << 12))),
            Some(Ordering::Equal) | None => Ok(Self(steps + (precision << 12))),
            Some(Ordering::Less) => Ok(Self(lower_steps + (lower_precision << 12))),
        }
    }
}
