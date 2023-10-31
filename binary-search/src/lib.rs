pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }

    let (mut l, mut r) = (0, array.len() - 1);
    while l < r {
        let mid = (l + r) >> 1;
        if array[mid] < key {
            l = mid + 1;
        } else {
            r = mid;
        }
    }

    if array[l] == key {
        Some(l)
    } else {
        None
    }
}
