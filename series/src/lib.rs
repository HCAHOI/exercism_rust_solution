pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return vec![];
    }

    let mut res = vec![];
    for last in len..=digits.len() {
        res.push(digits[(last - len)..last].to_string());
    }
    res
}
