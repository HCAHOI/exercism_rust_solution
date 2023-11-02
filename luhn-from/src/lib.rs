pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code = &self.code[..];
        let mut count = 0;
        let mut sum = 0;
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
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self {
            code: input.to_string(),
        }
    }
}
