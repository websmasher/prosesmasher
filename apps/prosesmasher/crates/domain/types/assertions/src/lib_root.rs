use core::fmt::{Debug, Display};

#[allow(clippy::missing_panics_doc)]
pub fn assert_display_contains<T>(value: T, expected: &str)
where
    T: Display,
{
    let message = value.to_string();
    assert!(
        message.contains(expected),
        "expected `{message}` to contain `{expected}`"
    );
}

pub fn assert_send_sync<T>()
where
    T: Send + Sync,
{
}

#[allow(clippy::missing_panics_doc)]
pub fn assert_clone_preserves_display<T>(value: T)
where
    T: Clone + Display + PartialEq + Debug,
{
    let cloned = value.clone();
    assert_eq!(
        value.to_string(),
        cloned.to_string(),
        "clone should preserve Display"
    );
    assert_eq!(value, cloned, "clone should preserve equality");
}
