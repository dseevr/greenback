// TODO: this entire function needs to be optimized and genericized
pub fn add_commas<T: ToString>(input: T) -> String {
    // TODO: replace with a loop that divides by 10 repeatedly once
    //       the generic integer stuff is figured out
    let num_digits = input.to_string().as_bytes().len();

    let mut num_commas = if num_digits > 3 { num_digits / 3 } else { 0 };

    let int_string = input.to_string();

    if num_commas > 0 {
        let mut left_offset = if num_commas > 0 {
            int_string.len() % 3 as usize
        } else {
            0
        };
        let mut byte_string = int_string.as_bytes().to_vec();

        if num_digits % 3 == 0 {
            left_offset = 3;
            num_commas -= 1;
        }

        for _ in 1..(num_commas + 1) {
            byte_string.insert(left_offset, ",".as_bytes()[0]);
            left_offset += 3 + 1; // +1 to account for the byte we inserted
        }

        String::from_utf8(byte_string).unwrap()
    } else {
        int_string // nothing to do
    }
}
