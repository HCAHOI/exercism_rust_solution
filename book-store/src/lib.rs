fn is_empty(bs: &Vec<i32>) -> bool {
    bs.iter().all(|&i| i == 0)
}

fn get_price(num: i32) -> u32 {
    let discount = match num {
        5 => 75,
        4 => 80,
        3 => 90,
        (1..=2) => 100,
        _ => panic!(),
    };
    8 * num as u32 * discount
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut bs = vec![0; 6];
    for &book in books {
        bs[book as usize] += 1;
    }

    println!("{:?}", bs);

    let mut res = 0;
    while !is_empty(&bs) {
        let mut cnt = 0;
        for i in 1..=5 {
            if bs[i] > 0 {
                bs[i] -= 1;
                cnt += 1;
            }
        }
        res += get_price(cnt);
        cnt = 0;
    }

    res
}
