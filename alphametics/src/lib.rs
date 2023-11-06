use std::collections::HashMap;

fn char2num(c: char, map: &HashMap<char, u8>) -> u8 {
    *map.get(&c).unwrap()
}

fn word2num(word: &str, map: &HashMap<char, u8>) -> Result<u64, ()> {
    let mut num: u64 = 0;
    if char2num(word.chars().nth(0).unwrap(), map) == 0 && word.len() != 1 {
        return Err(());
    }
    for c in word.chars() {
        num = num * 10 + char2num(c, map) as u64;
    }
    Ok(num)
}

fn DFS(
    idx: usize,
    letters: &Vec<char>,
    words: &Vec<String>,
    map: &mut HashMap<char, u8>,
    used: &mut Vec<bool>,
) -> bool {
    if idx == letters.len() {
        // check
        let l = words.len();
        let nums = words
            .iter()
            .filter_map(|word| {
                if let Ok(num) = word2num(word, map) {
                    Some(num)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if l != nums.len() {
            return false;
        }
        let sum = nums.iter().sum::<u64>();
        return if sum == nums[nums.len() - 1] * 2 {
            true
        } else {
            false
        };
    } else {
        // select a number for letters[idx] and go to next
        for i in 0..10 {
            if used[i] {
                continue;
            }
            if i == 0 && letters[idx] == words[0].chars().nth(0).unwrap() {
                continue;
            }
            map.insert(letters[idx], i as u8);
            used[i] = true;
            if DFS(idx + 1, letters, words, map, used) {
                return true;
            }
            used[i] = false;
        }
        false
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // get words string
    let mut words = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    // clean the "+" and "=="
    words.retain(|s| *s != "+" && *s != "==");
    // get letters
    let letters = words.iter().flat_map(|s| s.chars()).collect::<Vec<char>>();
    //unique
    let mut unique_letters = letters.clone();
    unique_letters.sort();
    unique_letters.dedup();
    let letters = unique_letters;
    // DFS
    let mut map = HashMap::new();
    let mut used = vec![false; 10];
    if DFS(0, &letters, &words, &mut map, &mut used) {
        Some(map)
    } else {
        None
    }
}
