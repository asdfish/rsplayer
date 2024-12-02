#[macro_export]
macro_rules! cast {
    ($num:expr_2021) => {
        $num.try_into().unwrap()
    }
}
