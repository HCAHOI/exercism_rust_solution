use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    let mut flag = true;

    candidate
        .to_lowercase()
        .chars()
        .filter(|&c| c.is_alphabetic())
        .for_each(|c| {
            if set.contains(&c) {
                flag = false;
            } else {
                set.insert(c);
            }
        });

    flag
}
