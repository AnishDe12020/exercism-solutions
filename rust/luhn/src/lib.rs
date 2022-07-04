/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut len = 0;

    let chars = code.chars().rev().filter(|c| !c.is_whitespace());

    for (index, num) in chars.enumerate() {
        len += 1;
        let digit = match num.to_digit(10) {
            Some(d) => d,
            None => return false,
        };
        if index % 2 == 0 {
            sum += digit;
        } else {
            let double = digit * 2;
            if double > 9 {
                sum += 1 + (double % 10);
            } else {
                sum += double;
            }
        }
    }

    len > 1 && sum % 10 == 0
}
