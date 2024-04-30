#[macro_export]
macro_rules! invalid_operation {
    ($($arg:tt)*) => {
        $crate::Error::new($crate::ErrorKind::InvalidOperation).add_context(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! return_invalid_operation {
    ($($arg:tt)*) => {
        return Err($crate::invalid_operation!($($arg)*));
    };
}

// process_error
#[macro_export]
macro_rules! process_error {
    ($($arg:tt)*) => {
        $crate::Error::new($crate::ErrorKind::ProcessError).add_context(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! return_process_error {
    ($($arg:tt)*) => {
        return Err($crate::process_error!($($arg)*));
    };
}
