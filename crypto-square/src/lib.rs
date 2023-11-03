pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return "".to_string();
    }
    let mut norm = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());
    let (c, r) = dim(norm.clone().count());
    let mut result = vec![String::new(); c];
    for i in 0..c * r {
        match norm.next() {
            Some(x) => result[i % c].push(x),
            None => result[i % c].push(' '),
        }
    }
    result.join(" ")
}
fn dim(str_length: usize) -> (usize, usize) {
    let mut c = 1;
    let mut r = 1;
    while c * r < str_length {
        match c > r {
            true => r += 1,
            false => c += 1,
        }
    }
    (c, r)
}
