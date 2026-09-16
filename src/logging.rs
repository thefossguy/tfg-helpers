#[macro_export]
#[cfg(debug_assertions)]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        eprintln!("{}:{} [DEBUG] {}", file!(), line!(), format_args!($($arg)*));
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! log_debug {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        eprintln!("{}:{} [INFO]  {}", file!(), line!(), format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        eprintln!("{}:{} [WARN]  {}", file!(), line!(), format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_err {
    ($($arg:tt)*) => {
        eprintln!("{}:{} [ERROR] {}", file!(), line!(), format_args!($($arg)*));
    };
}
