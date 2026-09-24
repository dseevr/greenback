pub mod error;
pub mod greenback;
pub mod util;

pub use error::GreenbackError;

/// A U.S. dollar amount represented as an exact integer number of cents.
///
/// Internally this wraps an [`i64`], giving a representable range of roughly
/// ±$92 quadrillion — comfortably beyond any real-world use case — while
/// avoiding the floating-point rounding errors that make `f32`/`f64`
/// unsuitable for money.
///
/// All arithmetic operators (`+`, `-`, `*`, `/`) panic on overflow or
/// division by zero, matching the behavior of Rust's built-in integer types
/// in debug builds — but consistently in *release* builds too, unlike raw
/// `i64` arithmetic. Callers who need to avoid panicking can use the
/// `checked_*` methods instead, which return `Option<Greenback>`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Greenback {
    // really, the number of cents but since there's also a function
    // called cents(), let's avoid unnecessary confusion
    raw_value: i64,
}

#[cfg(test)]
mod tests {
    use crate::Greenback;
    use crate::GreenbackError;
    use std::cmp::Ordering;
    use std::str::FromStr;

    #[test]
    fn test_constructors() {
        let m1 = Greenback::new(1, 23).unwrap();
        let m2 = Greenback::from_cents(1_23);

        assert_eq!(m1, m2);
    }

    #[test]
    fn test_new_rejects_invalid_cents() {
        assert_eq!(
            Greenback::new(1, 100),
            Err(GreenbackError::InvalidCents(100))
        );
        assert_eq!(Greenback::new(1, -1), Err(GreenbackError::InvalidCents(-1)));
    }

    #[test]
    fn test_dollars() {
        assert_eq!(Greenback::new(12, 23).unwrap().dollars(), 12);
    }

    #[test]
    fn test_cents() {
        assert_eq!(Greenback::new(12, 23).unwrap().cents(), 23);
    }

    #[test]
    fn test_negative_new() {
        let m = Greenback::new(-1, 50).unwrap();
        assert_eq!(m.raw_value(), -150);
    }

    #[test]
    fn test_add() {
        let m1 = Greenback::new(0, 99).unwrap();
        let m2 = Greenback::new(1, 15).unwrap();

        let m3 = m1 + m2;

        assert_eq!(m3.dollars(), 2);
        assert_eq!(m3.cents(), 14);
    }

    #[test]
    fn test_sub() {
        let m1 = Greenback::new(1, 15).unwrap();
        let m2 = Greenback::new(0, 20).unwrap();

        let m3 = m1 - m2;

        assert_eq!(m3.dollars(), 0);
        assert_eq!(m3.cents(), 95);
    }

    #[test]
    fn test_mul() {
        let m = Greenback::new(1, 10).unwrap();
        let m = m * 5;

        assert_eq!(m.dollars(), 5);
        assert_eq!(m.cents(), 50);
    }

    #[test]
    fn test_div() {
        let m = Greenback::new(1, 99).unwrap();
        let m = m / 5;

        assert_eq!(m.dollars(), 0);
        assert_eq!(m.cents(), 40);
    }

    #[test]
    fn test_div_precision_on_large_values() {
        // Regression test: the old f32-based division lost precision here
        // and produced 41152264 instead of the exact 41152263.
        let m = Greenback::from_cents(123_456_789);
        let m = m / 3;

        assert_eq!(m.raw_value(), 41_152_263);
    }

    #[test]
    fn test_add_assign() {
        let mut m = Greenback::new(1, 10).unwrap();
        m += Greenback::new(0, 10).unwrap();

        assert_eq!(m.dollars(), 1);
        assert_eq!(m.cents(), 20);
    }

    #[test]
    fn test_sub_assign() {
        let mut m = Greenback::new(1, 10).unwrap();
        m -= Greenback::new(0, 5).unwrap();

        assert_eq!(m.dollars(), 1);
        assert_eq!(m.cents(), 5);
    }

    #[test]
    fn test_mul_assign() {
        let mut m = Greenback::new(1, 10).unwrap();
        m *= 5;

        assert_eq!(m.dollars(), 5);
        assert_eq!(m.cents(), 50);
    }

    #[test]
    fn test_div_assign() {
        let mut m = Greenback::new(1, 99).unwrap();
        m /= 5;

        assert_eq!(m.dollars(), 0);
        assert_eq!(m.cents(), 40);
    }

    #[test]
    fn test_sum() {
        let m1 = Greenback::new(0, 10).unwrap();
        let m2 = Greenback::new(0, 5).unwrap();

        let ms = [m1, m2];
        let sum: Greenback = ms.iter().cloned().sum();

        assert_eq!(sum, m1 + m2);
    }

    #[test]
    fn test_equality() {
        let m1 = Greenback::new(1, 15).unwrap();
        let m2 = Greenback::new(1, 15).unwrap();
        let m3 = Greenback::new(1, 16).unwrap();

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn test_ordering() {
        let large = Greenback::new(10, 50).unwrap();
        let same = Greenback::new(10, 50).unwrap();
        let small = Greenback::new(0, 25).unwrap();

        assert_eq!(small.partial_cmp(&large), Some(Ordering::Less));
        assert_eq!(large.partial_cmp(&small), Some(Ordering::Greater));
        assert_eq!(large.partial_cmp(&same), Some(Ordering::Equal));

        assert!(large > small);
        assert!(large >= small);
        assert!(small < large);
        assert!(small <= large);
        assert!(large >= same);
        assert!(large <= same);

        let huge = Greenback::new(999, 99).unwrap();

        let mut items = [huge, small, large];
        items.sort();

        assert_eq!(*items.first().unwrap(), small);
        assert_eq!(*items.last().unwrap(), huge);
    }

    #[test]
    fn test_default_is_zero() {
        assert_eq!(Greenback::default(), Greenback::zero());
    }

    #[test]
    fn test_neg() {
        let m = Greenback::new(1, 50).unwrap();
        assert_eq!((-m).raw_value(), -150);
        assert_eq!((-(-m)).raw_value(), 150);
    }

    #[test]
    fn test_checked_arithmetic_overflow() {
        let big = Greenback::from_cents(i64::MAX);
        assert_eq!(big.checked_add(Greenback::from_cents(1)), None);

        let one = Greenback::from_cents(1);
        assert_eq!(one.checked_div(0), None);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_mul_panics_on_overflow() {
        let big = Greenback::from_cents(i64::MAX);
        let _ = big * 2;
    }

    #[test]
    #[should_panic(expected = "attempted to divide a Greenback by zero")]
    fn test_div_panics_on_zero() {
        let m = Greenback::new(1, 0).unwrap();
        let _ = m / 0;
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            Greenback::from_str("12.34").unwrap(),
            Greenback::new(12, 34).unwrap()
        );
        assert_eq!(
            Greenback::from_str("$12.34").unwrap(),
            Greenback::new(12, 34).unwrap()
        );
        assert_eq!(
            Greenback::from_str("$1,234.56").unwrap(),
            Greenback::new(1234, 56).unwrap()
        );
        assert_eq!(
            Greenback::from_str("-$1,234.56").unwrap(),
            Greenback::new(-1234, 56).unwrap()
        );
        assert_eq!(
            Greenback::from_str("12").unwrap(),
            Greenback::new(12, 0).unwrap()
        );
        assert_eq!(
            Greenback::from_str("12.3").unwrap(),
            Greenback::new(12, 30).unwrap()
        );

        assert!(Greenback::from_str("").is_err());
        assert!(Greenback::from_str("abc").is_err());
        assert!(Greenback::from_str("12.345").is_err());
    }

    #[test]
    fn test_display() {
        fn test(cents: i64, s: &str) {
            let res = format!("{}", Greenback::from_cents(cents));

            assert_eq!(res, s, "cents={cents}");
        }

        test(0, "$0.00");
        test(1, "$0.01");
        test(11, "$0.11");
        test(111, "$1.11");
        test(1111, "$11.11");
        test(11111, "$111.11");
        test(111111, "$1,111.11");
        test(1111111, "$11,111.11");
        test(11111111, "$111,111.11");
        test(111111111, "$1,111,111.11");
        test(1111111111, "$11,111,111.11");

        test(-0, "$0.00");
        test(-1, "-$0.01");
        test(-11, "-$0.11");
        test(-111, "-$1.11");
        test(-1111, "-$11.11");
        test(-11111, "-$111.11");
        test(-111111, "-$1,111.11");
        test(-1111111, "-$11,111.11");
        test(-11111111, "-$111,111.11");
        test(-111111111, "-$1,111,111.11");
        test(-1111111111, "-$11,111,111.11");
    }
}
