use std::collections::HashMap;

// Convert the dominoes into a directed graph. Search an Eulerian tour using a variation of Hierholzer's algorithm.
pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut num2stone = HashMap::new();
    for (i, &(a, b)) in input.iter().enumerate() {
        num2stone.entry(a).or_insert(Vec::new()).push(i);
        num2stone.entry(b).or_insert(Vec::new()).push(i);
    }
    if num2stone.values().any(|d| d.len() < 2 || d.len() % 2 != 0) {
        return None; // not a Eulerian graph => no chain exists
    }
    let (mut path, mut cycle) = (Vec::new(), Vec::new());
    let mut num = match num2stone.keys().next() {
        None => return Some(vec![]),
        Some(&num) => num,
    };
    loop {
        match num2stone.get_mut(&num).unwrap().pop() {
            Some(idx) => {
                let (a, b) = input[idx];
                let (a, b) = if a == num { (a, b) } else { (b, a) };
                num = b;
                num2stone.get_mut(&num).unwrap().retain(|&x| x != idx);
                path.push((a, b));
            }
            None => match path.pop() {
                None => break,
                Some((a, b)) => {
                    cycle.push((b, a));
                    num = a;
                }
            },
        }
    }
    if cycle.len() == input.len() {
        Some(cycle)
    } else {
        None
    }
}
