enum Dir {
    Right,
    Down,
    Left,
    Up,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![];
    } else if size == 1 {
        return vec![vec![1]];
    }

    let mut mat = vec![vec![0; size as usize]; size as usize];
    let size = size as usize;
    let mut row = 0;
    let mut col = 0;
    let mut cnt = 1;
    let mut cur_dir = Dir::Right;

    loop {
        if mat[row][col] != 0 {
            return mat;
        }
        mat[row][col] = cnt;
        cnt += 1;
        // next dir
        match cur_dir {
            Dir::Right => {
                if col == size - 1 || mat[row][col + 1] != 0 {
                    cur_dir = Dir::Down;
                    row += 1;
                } else {
                    col += 1;
                }
            }
            Dir::Down => {
                if row == size - 1 || mat[row + 1][col] != 0 {
                    cur_dir = Dir::Left;
                    col -= 1;
                } else {
                    row += 1;
                }
            }
            Dir::Left => {
                if col == 0 || mat[row][col - 1] != 0 {
                    cur_dir = Dir::Up;
                    row -= 1;
                } else {
                    col -= 1;
                }
            }
            Dir::Up => {
                if row == 0 || mat[row - 1][col] != 0 {
                    cur_dir = Dir::Right;
                    col += 1;
                } else {
                    row -= 1;
                }
            }
        }
    }
}
