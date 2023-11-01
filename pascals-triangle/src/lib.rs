pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

fn row_gen(last: &Vec<u32>) -> Vec<u32> {
    let mut res = vec![1; 1];
    for idx in 0..(last.len() - 1) {
        res.push(last[idx] + last[idx + 1]);
    }
    res.push(1);
    res
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut res = vec![];
        if row_count >= 1 {
            let mut last = vec![1; 1];
            res.push(vec![1]);
            for _ in 1..row_count {
                let temp = row_gen(&last);
                last = temp.clone();
                res.push(temp)
            }
        }
        Self { rows: res }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
