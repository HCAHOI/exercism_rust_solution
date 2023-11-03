use std::{collections::HashMap, thread};

fn count_frequency(input: Vec<String>) -> HashMap<char, usize> {
    let mut res = HashMap::new();
    for s in input {
        for c in s.chars() {
            if c.is_alphabetic() {
                *res.entry(c).or_insert(0) += 1;
            }
        }
    }
    res
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut handles = vec![];
    let input = input
        .iter()
        .map(|s| s.to_lowercase().to_string())
        .collect::<Vec<_>>();
    for i in 0..worker_count {
        let begin = input.len() / worker_count * i;
        let mut end = (input.len() / worker_count * (i + 1)).min(input.len());
        if i == worker_count - 1 {
            end = input.len();
        }
        println!("{} {}", begin, end);
        let subset = (begin..end).map(|j| input[j].clone()).collect::<Vec<_>>();
        let handle = thread::spawn(move || count_frequency(subset));
        handles.push(handle);
    }

    let mut maps = vec![];
    for handle in handles {
        maps.push(handle.join().unwrap());
    }

    let mut res = HashMap::new();
    for map in maps {
        map.iter().for_each(|(&c, &cnt)| {
            *res.entry(c).or_insert(0) += cnt;
        })
    }
    res
}
