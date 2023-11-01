use std::collections::HashSet;

fn word_translate(word: &str) -> String {
    let n = consonant_chars(word);
    let (consonant, other) = word.split_at(n);
    format!("{}{}ay", other, consonant)
}

fn consonant_chars(word: &str) -> usize {
    let vowel = HashSet::from(["a", "e", "i", "o", "u", "xr", "yt"]);
    let flag = vowel.iter().fold(0, |acc, vowel_begin| {
        if word.starts_with(vowel_begin) {
            acc + 1
        } else {
            acc
        }
    }) > 0;

    if word.is_empty() || flag {
        0
    } else if word.starts_with("qu") {
        2
    } else if word.chars().nth(1) == Some('y') {
        1
    } else {
        1 + consonant_chars(&word[1..])
    }
}

pub fn translate(input: &str) -> String {
    input
        .split(" ")
        .map(|s| word_translate(s))
        .collect::<Vec<_>>()
        .join(" ")
}
