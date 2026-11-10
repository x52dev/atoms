#[cfg(feature = "alloc")]
use alloc::borrow::Cow;

use crate::truncate;
#[cfg(feature = "alloc")]
use crate::{truncate_with, truncate_with_ellipsis};

mod sealed {
    pub trait Sealed {}

    impl Sealed for str {}
}

/// Extension methods for UTF-8-safe truncation of string slices.
///
/// These methods also work on types that dereference to `str`, such as `String` and
/// `bytestring::ByteString`.
///
/// The ellipsis methods require the `alloc` feature.
///
/// This trait is sealed and cannot be implemented outside this crate.
pub trait SafeTruncate: sealed::Sealed {
    /// Returns a borrowed prefix using the byte limit and UTF-8 rules of [`truncate`].
    ///
    /// ```
    /// use truncate_safe::SafeTruncate;
    ///
    /// assert_eq!("café".truncate_safe(4), "caf");
    /// ```
    #[must_use]
    fn truncate_safe(&self, max_len: usize) -> &str;

    /// Appends `…` when text is removed, as in [`truncate_with_ellipsis`].
    ///
    /// The ellipsis bytes are outside the limit. Unchanged text stays borrowed.
    ///
    /// ```
    /// # #[cfg(feature = "alloc")]
    /// # {
    /// use truncate_safe::SafeTruncate;
    ///
    /// assert_eq!(String::from("café").truncate_with_ellipsis(4), "caf…");
    /// # }
    /// ```
    #[cfg(feature = "alloc")]
    #[must_use]
    fn truncate_with_ellipsis(&self, max_len: usize) -> Cow<'_, str>;

    /// Appends custom text when text is removed, as in [`truncate_with`].
    ///
    /// The ellipsis bytes are outside the limit. Unchanged text and results with an empty
    /// ellipsis stay borrowed. The result borrows only from `self`.
    ///
    /// ```
    /// # #[cfg(feature = "alloc")]
    /// # {
    /// use truncate_safe::SafeTruncate;
    ///
    /// assert_eq!("café".truncate_with(4, "..."), "caf...");
    /// # }
    /// ```
    #[cfg(feature = "alloc")]
    #[must_use]
    fn truncate_with(&self, max_len: usize, ellipsis: &str) -> Cow<'_, str>;
}

impl SafeTruncate for str {
    #[inline]
    fn truncate_safe(&self, max_len: usize) -> &str {
        truncate(self, max_len)
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn truncate_with_ellipsis(&self, max_len: usize) -> Cow<'_, str> {
        truncate_with_ellipsis(self, max_len)
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn truncate_with(&self, max_len: usize, ellipsis: &str) -> Cow<'_, str> {
        truncate_with(self, max_len, ellipsis)
    }
}
