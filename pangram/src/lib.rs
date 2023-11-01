use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let chars = sentence
        .to_lowercase()
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .collect::<HashSet<_>>();

    chars.len() == 26
}
