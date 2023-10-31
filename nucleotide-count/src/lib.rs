use std::collections::{HashMap, HashSet};

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let valid = HashSet::from(['A', 'C', 'G', 'T']);
    let mut cnt = 0;
    if !valid.contains(&nucleotide) {
        return Err(nucleotide);
    }

    for c in dna.chars() {
        if !valid.contains(&c) {
            return Err(c);
        }
        if c == nucleotide {
            cnt += 1;
        }
    }

    Ok(cnt)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    for c in dna.chars() {
        if !map.contains_key(&c) {
            return Err(c);
        }
        map.entry(c).and_modify(|value| *value += 1);
    }
    Ok(map)
}
