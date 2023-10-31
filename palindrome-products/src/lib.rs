/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let cs = value.to_string().chars().collect::<Vec<_>>();
        let (mut l, mut r) = (0, cs.len() - 1);
        while l < r {
            if cs[l] != cs[r] {
                return None;
            }
            l += 1;
            r -= 1;
        }
        Some(Palindrome(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        let Self(v) = self;
        v
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_p = Palindrome(u64::MAX);
    let mut max_p = Palindrome(u64::MIN);
    let mut min_v = u64::MAX;
    let mut max_v = u64::MIN;
    let mut flag = false;
    for first in min..=max {
        for second in min..=max {
            let v = first * second;

            if v >= min_v && v <= max_v {
                continue;
            }

            if let Some(p) = Palindrome::new(v) {
                if p.into_inner() < min_p.into_inner() {
                    min_p = p;
                    min_v = v;
                    flag = true;
                }
                if p.into_inner() > max_p.into_inner() {
                    max_v = v;
                    max_p = p;
                }
            }
        }
    }

    if flag {
        Some((min_p, max_p))
    } else {
        None
    }
}
