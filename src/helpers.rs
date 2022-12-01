/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use aoc::helpers::example_fn;`.
 */

pub trait VecExt<T> {
    /// Removes and returns the first item that is equal to value if any.
    fn remove_value(&mut self, value: &T) -> Option<T>;
}

impl<T: PartialEq> VecExt<T> for Vec<T> {
    fn remove_value(&mut self, value: &T) -> Option<T> {
        self.iter()
            .position(|x| x == value)
            .map(|index| self.remove(index))
    }
}
