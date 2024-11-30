#[macro_export]
macro_rules! cast {
    ($num:expr) => {
        $num.try_into().unwrap()
    }
}
