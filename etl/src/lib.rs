use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new = BTreeMap::new();
    h.iter().for_each(|(&score, cs)| {
        String::from_iter(cs.clone().iter())
            .to_lowercase()
            .chars()
            .for_each(|c| {
                new.insert(c.clone(), score);
            })
    });
    new
}
