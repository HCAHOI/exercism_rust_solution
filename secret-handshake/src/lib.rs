use std::collections::HashMap;

pub fn actions(n: u8) -> Vec<&'static str> {
    let actions: HashMap<usize, &str> = HashMap::from([
        (0, "wink"),
        (1, "double blink"),
        (2, "close your eyes"),
        (3, "jump"),
    ]);
    let mut cnt: usize = 0;
    let mut res: Vec<&str> = vec![];
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            if cnt != 4 {
                res.push(actions.get(&cnt).unwrap());
            } else {
                res.reverse();
            }
        }
        n >>= 1;
        cnt += 1;
    }

    res
}
