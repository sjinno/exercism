#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* ) => {
            {
                let mut temp_hm = ::std::collections::HashMap::new();
                $(
                    temp_hm.insert($key, $value);
                )*
                temp_hm
            }
    };
    ( $( $key:expr => $value:expr, )* ) => {
        {
            let mut temp_hm = ::std::collections::HashMap::new();
            $(
                temp_hm.insert($key, $value);
            )*
            temp_hm
        }
    };
}
