/// Wraps any given value in [`Option`] if app is run in debug mode.
pub fn if_debug<T, F: FnOnce() -> T>(v: F) -> Option<T> {
    if cfg!(debug_assertions) {
        Some(v())
    } else {
        None
    }
}

/// Wraps any given value in [`Option`] if app is not run in debug mode.
pub fn if_release<T, F: FnOnce() -> T>(v: F) -> Option<T> {
    if cfg!(debug_assertions) {
        None
    } else {
        Some(v())
    }
}
