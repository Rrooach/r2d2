// #[cfg(debug_assertions)]
use std::cell::Cell;

#[cfg(debug_assertions)]
thread_local! {
    static VERBOSE: Cell<bool> = Cell::new(false);
}

#[cfg(debug_assertions)]
pub fn set_verbose(verbose: bool) {
    VERBOSE.with(|v| v.set(verbose))
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_info {
    (target: $target:expr, $($arg:tt)+) => (
        if crate::verbose::verbose_mode(){
            log::info!(target: $target, $crate::Level::Error, $($arg)+)
        }
    );
    ($($arg:tt)+) => (
        if crate::verbose::verbose_mode(){
            log::info!($($arg)+)
        }
    )
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_warn {
    (target: $target:expr, $($arg:tt)+) => (
        if crate::verbose::verbose_mode(){
            log::warn!(target: $target, $crate::Level::Error, $($arg)+)
        }
    );
    ($($arg:tt)+) => (
        if crate::verbose::verbose_mode(){
            log::warn!($($arg)+)
        }
    )
}

#[cfg(debug_assertions)]
#[inline]
pub(crate) fn verbose_mode() -> bool {
    VERBOSE.with(|v| v.get())
}

#[cfg(not(debug_assertions))]
#[inline(always)]
pub fn set_verbose(_verbose: bool) {}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_info {
    (target: $target:expr, $($arg:tt)+) => {};
    ($($arg:tt)+) => {};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_warn {
    (target: $target:expr, $($arg:tt)+) => {};
    ($($arg:tt)+) => {};
}

#[cfg(not(debug_assertions))]
pub(crate) fn verbose_mode() -> bool {
    false
}

thread_local! {
    static FUZZER_ID: Cell<u64> = Cell::new(0);
}

#[inline]
pub fn set_fuzzer_id(id: u64) {
    FUZZER_ID.with(|r| r.set(id));
}

#[inline]
pub fn fuzzer_id() -> u64 {
    FUZZER_ID.with(|r| r.get())
}

#[macro_export]
macro_rules! fuzzer_debug {
    ($t: tt, $($arg:tt)*) => (
        log::debug!(std::concat!("fuzzer-{}: ", $t), crate::fuzzer_log::fuzzer_id(), $($arg)*)
    )
}

#[macro_export]
macro_rules! fuzzer_info {
    ($($arg:tt)*) => {{
        use std::fmt::Write;
        let mut buffer = String::new();
        write!(buffer, $($arg)*).unwrap();
        println!("[{}]: {}", chrono::Utc::now().format("%Y-%m-%d][%H:%M:%S"), buffer, );
    }};
}

#[macro_export]
macro_rules! fuzzer_warn {
    ($t: tt, $($arg:tt)*) => (
        log::warn!(std::concat!("fuzzer-{}: ", $t), crate::fuzzer_log::fuzzer_id(), $($arg)*)
    )
}

#[macro_export]
macro_rules! fuzzer_error {
    ($t: tt, $($arg:tt)*) => (
        log::error!(std::concat!("fuzzer-{}: ", $t), crate::fuzzer_log::fuzzer_id(), $($arg)*)
    )
}
