//!
//! This module provides the `unwrap_that_mf!` macro for basically
//! unwrapping either `Option` or `Result` with a formatted string.
//!

pub use crate::unwrap_that_mf;

/// like `expect` but formatted string. works for both `Option` and `Result`
#[macro_export]
macro_rules! unwrap_that_mf {
    ($expr:expr, $($rest:tt),*) => {
        FmtExpect::expect_fmt($expr, || panic!($($rest),*))
    };
}

/// Basically `(Option|Result)::expect` but with formatting.
/// Use the `unwrap_that_mf!` macro.
///
/// The only reason this exists is because `Option` and `Result`
/// have a different number of arguments for `unwrap_or_else`. ;-;
pub trait FmtExpect<T> {
    /// `unwrap_or_else` but no arguments
    fn expect_fmt(self, panic: impl FnOnce()) -> T;
}

impl<T> FmtExpect<T> for Option<T> {
    fn expect_fmt(self, panic: impl FnOnce()) -> T {
        self.unwrap_or_else(|| {
            panic();
            unreachable!()
        })
    }
}

impl<T, E> FmtExpect<T> for Result<T, E> {
    fn expect_fmt(self, panic: impl FnOnce()) -> T {
        // ignore the damn argument!!!
        self.unwrap_or_else(|_| {
            panic();
            unreachable!()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unwrap_that_some() {
        let opt = Some(1);
        let n = unwrap_that_mf!(opt, "failed, beep boop.");
        assert_eq!(n, 1);
    }

    #[test]
    #[should_panic(expected = "expected 1, got None")]
    fn unwrap_that_none() {
        let opt: Option<i32> = None;
        unwrap_that_mf!(opt, "expected {}, got {:?}", 1, opt);
    }

    #[test]
    fn unwrap_that_ok() {
        let res: Result<i32, ()> = Ok(1);
        let n = unwrap_that_mf!(res, "failed, beep boop.");
        assert_eq!(n, 1);
    }

    #[test]
    #[should_panic(expected = "expected 1, got Err(())")]
    fn unwrap_that_err() {
        let res: Result<i32, ()> = Err(());
        unwrap_that_mf!(res, "expected {}, got {:?}", 1, res);
    }
}
