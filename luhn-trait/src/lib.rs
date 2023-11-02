pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
fn is_valid(code: String) -> bool {
    let code = &code[..];
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

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}
