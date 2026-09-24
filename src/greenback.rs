use crate::Greenback;
use crate::error::GreenbackError;
use crate::util::add_commas;
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

impl Greenback {
    /// Construct a `Greenback` from a whole-dollar amount and a cents
    /// component (`0..=99`).
    ///
    /// Returns [`GreenbackError::InvalidCents`] if `cents` is outside that
    /// range. For negative amounts, pass a negative `dollars` value with a
    /// non-negative `cents` (e.g. `Greenback::new(-1, 50)` is `-$1.50`).
    pub fn new(dollars: i64, cents: i64) -> Result<Greenback, GreenbackError> {
        if !(0..=99).contains(&cents) {
            return Err(GreenbackError::InvalidCents(cents));
        }

        let value = if dollars < 0 {
            dollars * 100 - cents
        } else {
            dollars * 100 + cents
        };

        Ok(Greenback::from_cents(value))
    }

    /// Construct a `Greenback` directly from a raw number of cents.
    ///
    /// This never fails: any `i64` is a valid number of cents.
    pub fn from_cents(cents: i64) -> Greenback {
        Greenback { raw_value: cents }
    }

    /// The additive identity: `$0.00`.
    pub fn zero() -> Greenback {
        Greenback { raw_value: 0 }
    }

    /// The whole-dollar portion of this amount (truncated toward zero).
    pub fn dollars(self) -> i64 {
        self.raw_value / 100
    }

    /// The cents portion of this amount, in `-99..=99`.
    pub fn cents(self) -> i64 {
        self.raw_value % 100
    }

    /// The total value in cents.
    pub fn raw_value(self) -> i64 {
        self.raw_value
    }

    /// Add two amounts, returning `None` on overflow instead of panicking.
    pub fn checked_add(self, rhs: Greenback) -> Option<Greenback> {
        self.raw_value
            .checked_add(rhs.raw_value)
            .map(Greenback::from_cents)
    }

    /// Subtract two amounts, returning `None` on overflow instead of
    /// panicking.
    pub fn checked_sub(self, rhs: Greenback) -> Option<Greenback> {
        self.raw_value
            .checked_sub(rhs.raw_value)
            .map(Greenback::from_cents)
    }

    /// Multiply by a scalar, returning `None` on overflow instead of
    /// panicking.
    pub fn checked_mul(self, rhs: i64) -> Option<Greenback> {
        self.raw_value.checked_mul(rhs).map(Greenback::from_cents)
    }

    /// Divide by a scalar (rounding to the nearest cent), returning `None`
    /// on overflow or division by zero instead of panicking.
    pub fn checked_div(self, rhs: i64) -> Option<Greenback> {
        if rhs == 0 {
            return None;
        }

        round_div(self.raw_value, rhs).map(Greenback::from_cents)
    }
}

/// Divide `numerator` by `denominator`, rounding to the nearest integer
/// (half away from zero), using only integer arithmetic so no precision is
/// lost the way it would be by round-tripping through `f32`/`f64`.
///
/// Returns `None` if `denominator` is zero or the result would overflow.
fn round_div(numerator: i64, denominator: i64) -> Option<i64> {
    if denominator == 0 {
        return None;
    }

    let quotient = numerator.checked_div(denominator)?;
    let remainder = numerator.checked_rem(denominator)?;

    if remainder == 0 {
        return Some(quotient);
    }

    // Round half away from zero: if the remainder is at least half the
    // denominator (in magnitude), bump the quotient away from zero.
    let remainder_doubled = (remainder.unsigned_abs()).checked_mul(2)?;
    if remainder_doubled >= denominator.unsigned_abs() {
        let bump = if (numerator < 0) != (denominator < 0) {
            -1
        } else {
            1
        };
        quotient.checked_add(bump)
    } else {
        Some(quotient)
    }
}

impl Add for Greenback {
    type Output = Greenback;

    fn add(self, rhs: Greenback) -> Greenback {
        self.checked_add(rhs)
            .unwrap_or_else(|| panic!("Greenback addition overflow: {self:?} + {rhs:?}"))
    }
}

impl Sub for Greenback {
    type Output = Greenback;

    fn sub(self, rhs: Greenback) -> Greenback {
        self.checked_sub(rhs)
            .unwrap_or_else(|| panic!("Greenback subtraction overflow: {self:?} - {rhs:?}"))
    }
}

impl Mul<i64> for Greenback {
    type Output = Greenback;

    fn mul(self, rhs: i64) -> Greenback {
        self.checked_mul(rhs)
            .unwrap_or_else(|| panic!("Greenback multiplication overflow: {self:?} * {rhs}"))
    }
}

impl Div<i64> for Greenback {
    type Output = Greenback;

    fn div(self, rhs: i64) -> Greenback {
        if rhs == 0 {
            panic!("attempted to divide a Greenback by zero");
        }

        self.checked_div(rhs)
            .unwrap_or_else(|| panic!("Greenback division overflow: {self:?} / {rhs}"))
    }
}

impl AddAssign for Greenback {
    fn add_assign(&mut self, rhs: Greenback) {
        *self = *self + rhs;
    }
}

impl SubAssign for Greenback {
    fn sub_assign(&mut self, rhs: Greenback) {
        *self = *self - rhs;
    }
}

impl MulAssign<i64> for Greenback {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl DivAssign<i64> for Greenback {
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}

impl Neg for Greenback {
    type Output = Greenback;

    fn neg(self) -> Greenback {
        self.raw_value
            .checked_neg()
            .map(Greenback::from_cents)
            .unwrap_or_else(|| panic!("Greenback negation overflow: {self:?}"))
    }
}

impl Sum for Greenback {
    fn sum<I>(iter: I) -> Greenback
    where
        I: Iterator<Item = Greenback>,
    {
        iter.fold(Greenback::zero(), Add::add)
    }
}

impl FromStr for Greenback {
    type Err = GreenbackError;

    /// Parse a dollar amount from a string. Accepts an optional leading
    /// `-`, an optional `$`, optional thousands separators (`,`), and an
    /// optional fractional part of one or two digits (e.g. `"12"`,
    /// `"12.3"`, `"$1,234.56"`, `"-$1,234.56"`).
    fn from_str(s: &str) -> Result<Greenback, GreenbackError> {
        let original = s;
        let mut s = s.trim();

        let negative = if let Some(rest) = s.strip_prefix('-') {
            s = rest;
            true
        } else {
            false
        };

        let s = s.strip_prefix('$').unwrap_or(s);
        let s = s.replace(',', "");

        if s.is_empty() {
            return Err(GreenbackError::ParseError(original.to_string()));
        }

        let (dollars_str, cents_str) = match s.split_once('.') {
            Some((d, c)) => (d, c),
            None => (s.as_str(), ""),
        };

        if dollars_str.is_empty() && cents_str.is_empty() {
            return Err(GreenbackError::ParseError(original.to_string()));
        }

        let dollars: i64 = if dollars_str.is_empty() {
            0
        } else {
            dollars_str
                .parse()
                .map_err(|_| GreenbackError::ParseError(original.to_string()))?
        };

        let cents: i64 = match cents_str.len() {
            0 => 0,
            1 => {
                cents_str
                    .parse::<i64>()
                    .map_err(|_| GreenbackError::ParseError(original.to_string()))?
                    * 10
            }
            2 => cents_str
                .parse()
                .map_err(|_| GreenbackError::ParseError(original.to_string()))?,
            _ => return Err(GreenbackError::ParseError(original.to_string())),
        };

        let magnitude = dollars
            .checked_mul(100)
            .and_then(|d| d.checked_add(cents))
            .ok_or_else(|| GreenbackError::ParseError(original.to_string()))?;

        Ok(Greenback::from_cents(if negative {
            -magnitude
        } else {
            magnitude
        }))
    }
}

impl fmt::Display for Greenback {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.raw_value < 0 { "-" } else { "" };

        write!(
            f,
            "{}${}.{cents:>0width$}",
            sign,
            add_commas(self.dollars().unsigned_abs()),
            cents = self.cents().unsigned_abs(),
            width = 2
        )
    }
}
