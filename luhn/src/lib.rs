/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut count = 0;
    for c in code.chars().rev() {
        if c.is_whitespace() {
            continue;
        }
        if !c.is_digit(10) {
            return false;
        }
        let mut digit = c.to_digit(10).unwrap();
        if count % 2 == 1 {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }
        sum += digit;
        count += 1;
    }
    count > 1 && sum % 10 == 0
}
