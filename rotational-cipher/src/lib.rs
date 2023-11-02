fn ascii(c: char) -> u8 {
    c as u8
}

fn char_move(c: char, key: i8) -> char {
    if c.is_alphabetic() && c.is_uppercase() {
        return ((ascii(c) - ascii('A') + key as u8) % 26 + ascii('A')) as char;
    } else if c.is_alphabetic() && c.is_lowercase() {
        return ((ascii(c) - ascii('a') + key as u8) % 26 + ascii('a')) as char;
    } else {
        return c;
    }
}

pub fn rotate(input: &str, key: i8) -> String {
    let key = (key % 26 + 26) % 26;
    input.chars().map(|c| char_move(c, key)).collect::<String>()
}
