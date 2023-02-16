// macro_rules! log {
//     ($($arg:tt)*) => {
//         $crate::log::log($crate::log::Level::Info, format_args!($($arg)*));
//     };
// }

// macro_rules for log! macro with different log levels (e.g.
// log!(LogLevel::Info.into(), "leafc driver version: {}", Self::VERSION);
// log macro shells out to the widely used log! macro from the log crate
// it is mostly a shim to allow for configuring the log level, log target, and
// log format macro_rules! leafc_log {
//     ($($arg:tt)*) => {
//         $crate::log::log($crate::log::Level::Info, format_args!($($arg)*));
//     };
// }
