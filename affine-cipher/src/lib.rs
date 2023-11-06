static M: usize = 26;

fn idx(c: char) -> usize {
    (c as u8 - 'a' as u8) as usize
}

fn get_char(idx: usize) -> char {
    ('a' as u8 + idx as u8) as char
}

fn is_coprime(a: usize, b: usize) -> bool {
    let m = a.min(b);
    for i in 2..=m {
        if a % i == 0 && b % i == 0 {
            return false;
        }
    }
    true
}

fn mmi(a: usize) -> usize {
    for i in 1..M {
        if (a * i) % M == 1 {
            return i;
        }
    }
    0
}

fn char_encode(c: char, a: i32, b: i32) -> char {
    if c.is_alphabetic() {
        let a = a as usize;
        let b = b as usize;
        get_char((a * idx(c) + b) % M)
    } else {
        c
    }
}

fn char_decode(c: char, a: i32, b: i32) -> char {
    if c.is_alphabetic() {
        let a = a as usize;
        get_char((((mmi(a) as i32 * (idx(c) as i32 - b as i32)) % M as i32 + 26) % M as i32) as usize)
    } else {
        c
    }
}

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a as usize, M) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(plaintext
        .to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(char_encode(c, a, b))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|v| String::from_iter(v.iter()))
        .collect::<Vec<_>>()
        .join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a as usize, M) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    Ok(ciphertext
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(char_decode(c, a, b))
            } else {
                None
            }
        })
        .collect::<String>())
}
