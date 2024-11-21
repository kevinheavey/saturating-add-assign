#![doc = include_str!("../README.md")]

/// Convenience macro for `AddAssign` with saturating arithmetic.
#[macro_export]
macro_rules! saturating_add_assign {
    ($i:expr, $v:expr) => {{
        $i = $i.saturating_add($v)
    }};
}
