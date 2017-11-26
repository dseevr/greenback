pub mod greenback;
pub mod util;

#[derive(Copy,Clone)]
pub struct Greenback {
    // really, the number of cents but since there's also a function
    // called cents(), let's avoid unnecessary confusion
    raw_value: i32,
}

#[cfg(test)]
mod tests {
    use std::cmp::{PartialOrd, Ordering};
    use greenback::Greenback;

    #[test]
    fn test_constructors() {
        let m1 = Greenback::new(1, 23);
        let m2 = Greenback::from_cents(1_23);

        assert!(m1 == m2);
    }

    #[test]
    fn test_dollars() {
        assert!(Greenback::new(12, 23).dollars() == 12);
    }

    #[test]
    fn test_cents() {
        assert!(Greenback::new(12, 23).cents() == 23);
    }

    #[test]
    fn test_add() {
        let m1 = Greenback::new(0, 99);
        let m2 = Greenback::new(1, 15);

        let m3 = m1 + m2;

        assert!(m3.dollars() == 2);
        assert!(m3.cents() == 14);
    }

    #[test]
    fn test_sub() {
        let m1 = Greenback::new(1, 15);
        let m2 = Greenback::new(0, 20);

        let m3 = m1 - m2;

        assert!(m3.dollars() == 0);
        assert!(m3.cents() == 95);
    }

    #[test]
    fn test_mul() {
        let mut m = Greenback::new(1, 10);
        m = m * 5;

        assert!(m.dollars() == 5);
        assert!(m.cents() == 50);
    }

    #[test]
    fn test_div() {
        let mut m = Greenback::new(1, 99);
        m = m / 5;

        assert!(m.dollars() == 0);
        assert!(m.cents() == 40);
    }

    #[test]
    fn test_add_assign() {
        let mut m = Greenback::new(1, 10);
        m += Greenback::new(0, 10);

        assert!(m.dollars() == 1);
        assert!(m.cents() == 20);
    }

    #[test]
    fn test_sub_assign() {
        let mut m = Greenback::new(1, 10);
        m -= Greenback::new(0, 5);

        assert!(m.dollars() == 1);
        assert!(m.cents() == 5);
    }

    #[test]
    fn test_mul_assign() {
        let mut m = Greenback::new(1, 10);
        m *= 5;

        assert!(m.dollars() == 5);
        assert!(m.cents() == 50);
    }

    #[test]
    fn test_div_assign() {
        let mut m = Greenback::new(1, 99);
        m /= 5;

        assert!(m.dollars() == 0);
        assert!(m.cents() == 40);
    }

    #[test]
    fn test_sum() {
        let m1 = Greenback::new(0, 10);
        let m2 = Greenback::new(0, 5);

        let ms = vec![m1, m2];
        let sum: Greenback = ms.iter().cloned().sum();

        assert!(sum == m1 + m2);
    }

    #[test]
    fn test_equality() {
        let m1 = Greenback::new(1, 15);
        let m2 = Greenback::new(1, 15);
        let m3 = Greenback::new(1, 16);

        assert!(m1 == m2);
        assert!(m1 != m3);
    }

    #[test]
    fn test_ordering() {
        let large = Greenback::new(10, 50);
        let same  = Greenback::new(10, 50);
        let small = Greenback::new(0, 25);

        assert_eq!(small.partial_cmp(&large), Some(Ordering::Less));
        assert_eq!(large.partial_cmp(&small), Some(Ordering::Greater));
        assert_eq!(large.partial_cmp(&same), Some(Ordering::Equal));

        assert!(large > small);
        assert!(large >= small);
        assert!(small < large);
        assert!(small <= large);
        assert!(large >= same);
        assert!(large <= same);
    }

    #[test]
    fn test_display() {
        fn test(cents: i32, s: &str) {
            let res = format!("{}", Greenback::from_cents(cents));

            println!("Expected: {}", s);
            println!("Got: {}", res);

            assert!(res == s);
        }

        println!("");

        test(0,          "$0.00");
        test(1,          "$0.01");
        test(11,         "$0.11");
        test(111,        "$1.11");
        test(1111,       "$11.11");
        test(11111,      "$111.11");
        test(111111,     "$1,111.11");
        test(1111111,    "$11,111.11");
        test(11111111,   "$111,111.11");
        test(111111111,  "$1,111,111.11");
        test(1111111111, "$11,111,111.11");

        test(-0,          "$0.00");
        test(-1,          "-$0.01");
        test(-11,         "-$0.11");
        test(-111,        "-$1.11");
        test(-1111,       "-$11.11");
        test(-11111,      "-$111.11");
        test(-111111,     "-$1,111.11");
        test(-1111111,    "-$11,111.11");
        test(-11111111,   "-$111,111.11");
        test(-111111111,  "-$1,111,111.11");
        test(-1111111111, "-$11,111,111.11");
    }
}
