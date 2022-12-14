mod grid;
pub mod parser;

use std::iter::Step;

pub use grid::*;

pub trait RemoveValue<T> {
    /// Removes and returns the first item that is equal to value if any.
    fn remove_value(&mut self, value: &T) -> Option<T>;
}

impl<T: PartialEq> RemoveValue<T> for Vec<T> {
    fn remove_value(&mut self, value: &T) -> Option<T> {
        self.iter()
            .position(|x| x == value)
            .map(|index| self.remove(index))
    }
}

pub trait StrExt {
    /// Divide one string slice into two at an index, without doing bounds checking.
    ///
    /// The argument, `mid`, should be a byte offset from the start of the
    /// string. It must also be on the boundary of a UTF-8 code point.
    ///
    /// The two slices returned go from the start of the string slice to `mid`,
    /// and from `mid` to the end of the string slice.
    ///
    /// # Safety
    ///
    /// Callers of this function are responsible that these preconditions are
    /// satisfied:
    ///
    /// * `mid` must be within bounds of the string slice;
    /// * `mid` must lie on UTF-8 sequence boundaries.
    ///
    /// Failing that, the returned string slice may reference invalid memory or
    /// violate the invariants communicated by the `str` type.
    unsafe fn split_at_unchecked(&self, mid: usize) -> (&str, &str);
}

impl<'a> StrExt for &'a str {
    #[inline]
    unsafe fn split_at_unchecked(&self, mid: usize) -> (&str, &str) {
        // SAFETY: the caller must uphold the safety contract for `get_unchecked`;
        unsafe {
            (
                self.get_unchecked(0..mid),
                self.get_unchecked(mid..self.len()),
            )
        }
    }
}

/// Iterator from `a` (included) to `b` (included).
///
/// ```
/// # use adventofcode::helpers::range;
/// let mut values = range(1, 3);
/// assert_eq!(values.next(), Some(1));
/// assert_eq!(values.next(), Some(2));
/// assert_eq!(values.next(), Some(3));
/// assert_eq!(values.next(), None);
///
/// let mut values = range(3, 1);
/// assert_eq!(values.next(), Some(3));
/// assert_eq!(values.next(), Some(2));
/// assert_eq!(values.next(), Some(1));
/// assert_eq!(values.next(), None);
/// ```
pub fn range<'a, T: 'a + PartialOrd + Step>(a: T, b: T) -> Box<dyn Iterator<Item = T> + 'a> {
    if a < b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}
