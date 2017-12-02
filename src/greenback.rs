use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use std::iter::{Iterator, Sum};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

use Greenback;
use util::add_commas;

impl Greenback {
    pub fn new(dollars: i32, cents: i32) -> Greenback {
        if cents < 0 || cents > 99 {
            panic!("cents must be >= 0 and <= 99")
        }

        let value = if dollars < 0 {
            dollars * 100 - cents
        } else {
            dollars * 100 + cents
        };

        Greenback::from_cents(value)
    }

    pub fn from_cents(cents: i32) -> Greenback {
        Greenback { raw_value: cents }
    }

    pub fn from_float(f: f32) -> Greenback {
        Greenback {
            raw_value: (f * 100.0).round() as i32,
        }
    }

    pub fn zero() -> Greenback {
        Greenback { raw_value: 0 }
    }

    pub fn dollars(&self) -> i32 {
        self.raw_value / 100
    }

    pub fn cents(&self) -> i32 {
        self.raw_value % 100
    }

    pub fn raw_value(&self) -> i32 {
        self.raw_value
    }
}

impl Add for Greenback {
    type Output = Greenback;

    fn add(self, rhs: Greenback) -> Greenback {
        Greenback {
            raw_value: self.raw_value + rhs.raw_value,
        }
    }
}

impl Sub for Greenback {
    type Output = Greenback;

    fn sub(self, rhs: Greenback) -> Greenback {
        Greenback {
            raw_value: self.raw_value - rhs.raw_value,
        }
    }
}

impl Mul<i32> for Greenback {
    type Output = Greenback;

    fn mul(self, rhs: i32) -> Greenback {
        Greenback {
            raw_value: self.raw_value * rhs,
        }
    }
}

impl Div<i32> for Greenback {
    type Output = Greenback;

    fn div(self, rhs: i32) -> Greenback {
        let cents = (self.raw_value as f32 / rhs as f32).round() as i32;

        Greenback { raw_value: cents }
    }
}

impl AddAssign for Greenback {
    fn add_assign(&mut self, rhs: Greenback) {
        self.raw_value = self.raw_value + rhs.raw_value;
    }
}

impl SubAssign for Greenback {
    fn sub_assign(&mut self, rhs: Greenback) {
        self.raw_value = self.raw_value - rhs.raw_value;
    }
}

impl MulAssign<i32> for Greenback {
    fn mul_assign(&mut self, rhs: i32) {
        self.raw_value *= rhs;
    }
}

impl DivAssign<i32> for Greenback {
    fn div_assign(&mut self, rhs: i32) {
        // round up to nearest cent when dividing
        self.raw_value = (self.raw_value as f32 / rhs as f32).round() as i32;
    }
}

impl PartialEq for Greenback {
    fn eq(&self, rhs: &Greenback) -> bool {
        self.raw_value == rhs.raw_value
    }

    fn ne(&self, rhs: &Greenback) -> bool {
        self.raw_value != rhs.raw_value
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

impl PartialOrd for Greenback {
    fn partial_cmp(&self, rhs: &Greenback) -> Option<Ordering> {
        if self < rhs {
            Some(Ordering::Less)
        } else if self == rhs {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }

    fn lt(&self, rhs: &Greenback) -> bool {
        self.raw_value < rhs.raw_value
    }

    fn le(&self, rhs: &Greenback) -> bool {
        self < rhs || self == rhs
    }

    fn gt(&self, rhs: &Greenback) -> bool {
        self.raw_value > rhs.raw_value
    }

    fn ge(&self, rhs: &Greenback) -> bool {
        self > rhs || self == rhs
    }
}

impl Eq for Greenback {}

impl Ord for Greenback {
    fn cmp(&self, other: &Greenback) -> Ordering {
        self.raw_value.cmp(&other.raw_value)
    }
}


// TODO: figure out a way to have the output format configurable
impl fmt::Display for Greenback {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.raw_value < 0 { "-" } else { "" };

        write!(
            f,
            "{}${}.{cents:>0width$}",
            sign,
            add_commas(self.dollars().abs()),
            cents = self.cents().abs(),
            width = 2
        )
    }
}
