use std::cmp::Ordering;

pub fn find<T: Ord, U: AsRef<[T]>>(array: U, key: T) -> Option<usize> {
    let array = array.as_ref();
    if array.len() == 0 {
        return None;
    }

    let (mut l, mut r) = (0, array.len() - 1);
    while l < r {
        let mid = (l + r) >> 1;
        match array[mid].cmp(&key) {
            Ordering::Less => {
                l = mid + 1;
            }
            Ordering::Greater => {
                r = mid;
            }
            Ordering::Equal => {
                return Some(mid);
            }
        }
    }
    None
}
