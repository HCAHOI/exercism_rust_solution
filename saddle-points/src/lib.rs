pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut row_max = vec![];
    let mut col_min = vec![];

    for (row, line) in input.iter().enumerate() {
        let mut max_v = u64::MIN;
        for (_, &v) in line.iter().enumerate() {
            if max_v < v {
                max_v = v;
            }
        }
        for (col, &v) in line.iter().enumerate() {
            if max_v == v {
                row_max.push((row, col));
            }
        }
    }

    for col in 0..input[0].len() {
        let mut min_v = u64::MAX;
        for row in 0..input.len() {
            let v = input[row][col];
            if min_v > v {
                min_v = v;
            }
        }
        for row in 0..input.len() {
            let v = input[row][col];
            if min_v == v {
                col_min.push((row, col));
            }
        }
    }

    println!("{:?}", row_max);
    println!("{:?}", col_min);
    row_max
        .iter()
        .filter_map(|&pos| {
            if col_min.contains(&pos) {
                Some(pos)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
