#[macro_export]
macro_rules! vec_vec_i32_1 {
    ($($tail:tt),*) => {
        vec![$(vec!$tail),*] as Vec<Vec<i32>>
    };
}