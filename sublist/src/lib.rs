#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contain<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() < b.len() {
        return false;
    }

    if a.starts_with(b) {
        return true;
    }

    contain(&a[1..], b)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if contain(_first_list, _second_list) {
        Comparison::Superlist
    } else if contain(_second_list, _first_list) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}