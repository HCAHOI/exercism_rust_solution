use std::collections::HashMap;

const DISCOUNT: [u32; 5] = [0, 10, 30, 80, 125];
pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counter = HashMap::new();
    for book in books {
        *counter.entry(book).or_insert(0) += 1;
    }

    let mut v: Vec<_> = counter.values().copied().collect();
    v.sort_unstable_by(|a, b| b.cmp(a));
    for i in 0..v.len().saturating_sub(1) {
        v[i] -= v[i + 1];
    }
    if v.len() >= 5 {
        let m = v[2].min(v[4]);
        v[2] -= m;
        v[4] -= m;
        v[3] += 2 * m;
    }
    let deduction: u32 = v.iter().zip(DISCOUNT).map(|(x, y)| x * y).sum();
    (books.len() as u32 * 100 - deduction) * 8
}
