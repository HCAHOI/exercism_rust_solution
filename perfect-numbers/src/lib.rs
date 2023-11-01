use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn factors(num: u64) -> Vec<u64> {
    (1..num)
        .filter_map(|v| if num % v == 0 { Some(v) } else { None })
        .collect()
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let sum = factors(num).iter().sum();
    println!("{}", sum);
    match num.cmp(&sum) {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Less => Some(Classification::Abundant),
        Ordering::Greater => Some(Classification::Deficient),
    }
}
