/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use aoc::helpers::example_fn;`.
 */

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
