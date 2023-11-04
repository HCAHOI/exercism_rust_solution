/// What should the type of _function be?
pub fn map<F, T, U>(input: Vec<T>, mut _function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut res = vec![];
    for v in input {
        let temp = _function(v);
        res.push(temp);
    }
    res
}
