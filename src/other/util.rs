#[macro_export]
macro_rules! vec_vec_i32_1 {
    ($($tail:tt),*) => {
        vec![$(vec!$tail),*] as Vec<Vec<i32>>
    };
}

#[macro_export]
macro_rules! vec_string {
    ($($tail:tt),*) => {
        vec![$($tail.to_string()),*] as Vec<String>
    };
}