#[derive(Clone)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(_max_weight: u32, _items: &[Item]) -> u32 {
    let mut items = vec![Item {
        weight: 0,
        value: 0,
    }];
    let n = _items.len();
    _items.iter().for_each(|item| items.push(item.clone()));
    let mut dp: Vec<u32> = vec![0; _max_weight as usize + 10];
    for i in 1..=n {
        for j in (items[i].weight..=_max_weight).rev() {
            dp[j as usize] =
                dp[j as usize].max(dp[(j - items[i].weight) as usize] + items[i].value);
        }
    }
    dp[_max_weight as usize]
}
