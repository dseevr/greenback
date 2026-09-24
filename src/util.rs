//! Small formatting helpers used internally by [`crate::Greenback`]'s
//! `Display` implementation.

/// Group the digits of a non-negative integer's decimal representation with
/// commas every three digits (e.g. `1234567` -> `"1,234,567"`).
///
/// This function only ever receives an unsigned magnitude — sign handling is
/// the caller's responsibility — which keeps the digit-grouping logic simple
/// and avoids the classic bug of a comma landing next to a leading minus
/// sign.
pub fn add_commas(value: u64) -> String {
    let digits = value.to_string();
    let num_digits = digits.len();

    if num_digits <= 3 {
        return digits;
    }

    let mut result = String::with_capacity(num_digits + num_digits / 3);

    for (i, ch) in digits.chars().enumerate() {
        let remaining = num_digits - i;
        if i > 0 && remaining % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::add_commas;

    #[test]
    fn test_add_commas() {
        let cases: &[(u64, &str)] = &[
            (0, "0"),
            (1, "1"),
            (12, "12"),
            (123, "123"),
            (1234, "1,234"),
            (12345, "12,345"),
            (123456, "123,456"),
            (1234567, "1,234,567"),
            (12345678, "12,345,678"),
            (123456789, "123,456,789"),
            (1234567890, "1,234,567,890"),
            (100, "100"),
            (1000, "1,000"),
            (10000, "10,000"),
            (100000, "100,000"),
            (1000000, "1,000,000"),
        ];

        for (input, expected) in cases {
            assert_eq!(add_commas(*input), *expected, "input: {input}");
        }
    }
}
