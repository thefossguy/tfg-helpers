#[macro_export]
macro_rules! return_formatted_err {
    ($($arg:tt)*) => {
        Err(format!("{}:{} {}", file!(), line!(), format_args!($($arg)*)).into())
    };
}
