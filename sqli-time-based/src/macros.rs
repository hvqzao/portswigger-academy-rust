#[macro_export]
macro_rules! dbg_print {
    ($check:expr, $($arg:tt)*) => {
        if $check {
            println!($($arg)*);
        }
    }
}
