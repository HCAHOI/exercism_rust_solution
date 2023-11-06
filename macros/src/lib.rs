#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr,)+) => { macros::hashmap!($($key => $value),+) };
    ( $( $key:expr => $value:expr),* ) => {
        {
            let mut temp_vec = ::std::collections::HashMap::new();
            $(
                temp_vec.insert($key, $value);
                )*
            temp_vec
        }
    };
}
