fn ascii(c: char) -> u8 {
    c as u8
}

fn trans(c: char) -> char {
    if c.is_alphabetic() {
        return (ascii('z') - ascii(c) + ascii('a')) as char;
    }
    c
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(trans(c))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|slice| slice.iter().cloned().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .split(" ")
        .map(|v| v.chars().map(|c| trans(c)).collect::<String>())
        .collect::<String>()
}
