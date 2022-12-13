use std::iter::Enumerate;

#[derive(Debug, Default, Clone)]
pub struct Grid<T> {
    elements: Box<[T]>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn new_from_width(elements: Vec<T>, width: usize) -> Result<Self, Vec<T>> {
        if elements.len() % width != 0 {
            return Err(elements);
        }
        Ok(Self {
            elements: elements.into_boxed_slice(),
            width,
        })
    }

    pub fn new_from_height(elements: Vec<T>, height: usize) -> Result<Self, Vec<T>> {
        if elements.len() % height != 0 {
            return Err(elements);
        }
        let width = elements.len() / height;
        Ok(Self {
            elements: elements.into_boxed_slice(),
            width,
        })
    }

    pub fn elements(&self) -> &[T] {
        &self.elements
    }

    pub fn flatten(self) -> Vec<T> {
        self.elements.into()
    }

    pub fn flatten_box(self) -> Box<[T]> {
        self.elements
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.elements.len() / self.width
    }

    pub fn unflat_index(&self, index: usize) -> GridIndex {
        unflat_index(index, self.width)
    }

    pub fn get<I: Into<GridIndex>>(&self, index: I) -> Option<&T> {
        let GridIndex { x, y } = index.into();
        if x < self.width() && y < self.height() {
            // Safety: `x` and `y` are in bounds.
            Some(unsafe { self.get_unchecked(x, y) })
        } else {
            None
        }
    }

    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> &T {
        self.elements.get_unchecked(flat_index(x, y, self.width))
    }

    pub fn get_mut<I: Into<GridIndex>>(&mut self, index: I) -> Option<&mut T> {
        let GridIndex { x, y } = index.into();
        if x < self.width() && y < self.height() {
            // Safety: `x` and `y` are in bounds.
            Some(unsafe { self.get_unchecked_mut(x, y) })
        } else {
            None
        }
    }

    pub unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.elements
            .get_unchecked_mut(flat_index(x, y, self.width))
    }

    pub fn iter_with_index(&self) -> IterWithIndex<T> {
        IterWithIndex {
            inner: self.elements.iter().enumerate(),
            width: self.width,
        }
    }

    pub fn iter_with_index_mut(&mut self) -> IterWithIndexMut<T> {
        IterWithIndexMut {
            inner: self.elements.iter_mut().enumerate(),
            width: self.width,
        }
    }

    pub fn into_iter_with_index(self) -> IntoIterWithIndex<T> {
        let width = self.width;
        let elements = self.flatten();

        IntoIterWithIndex {
            inner: elements.into_iter().enumerate(),
            width,
        }
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = (GridIndex, T);

    type IntoIter = IntoIterWithIndex<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter_with_index()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = (GridIndex, &'a T);

    type IntoIter = IterWithIndex<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_with_index()
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = (GridIndex, &'a mut T);

    type IntoIter = IterWithIndexMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_with_index_mut()
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GridIndex {
    pub x: usize,
    pub y: usize,
}

/// Returns the top, bottom, right and left neighbors if they are in
/// grid's bounds.
pub fn grid_neighbors(index: GridIndex, width: usize, height: usize) -> [Option<GridIndex>; 4] {
    let GridIndex { x, y } = index;

    let right = {
        let x = x + 1;
        if x == width {
            None
        } else {
            Some(GridIndex { x, y })
        }
    };

    let bottom = {
        let y = y + 1;
        if y == height {
            None
        } else {
            Some(GridIndex { x, y })
        }
    };

    let left = x.checked_sub(1).map(|x| GridIndex { x, y });

    let top = y.checked_sub(1).map(|y| GridIndex { x, y });

    [top, bottom, right, left]
}

impl From<(usize, usize)> for GridIndex {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

pub struct IntoIterWithIndex<T> {
    inner: Enumerate<std::vec::IntoIter<T>>,
    width: usize,
}

impl<T> Iterator for IntoIterWithIndex<T> {
    type Item = (GridIndex, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(i, elt)| (unflat_index(i, self.width), elt))
    }
}

pub struct IterWithIndex<'a, T> {
    inner: Enumerate<std::slice::Iter<'a, T>>,
    width: usize,
}

impl<'a, T> Iterator for IterWithIndex<'a, T> {
    type Item = (GridIndex, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(i, elt)| (unflat_index(i, self.width), elt))
    }
}

pub struct IterWithIndexMut<'a, T> {
    inner: Enumerate<std::slice::IterMut<'a, T>>,
    width: usize,
}

impl<'a, T> Iterator for IterWithIndexMut<'a, T> {
    type Item = (GridIndex, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(i, elt)| (unflat_index(i, self.width), elt))
    }
}

#[inline]
fn unflat_index(index: usize, width: usize) -> GridIndex {
    let x = index % width;
    let y = index / width;
    GridIndex { x, y }
}

#[inline]
fn flat_index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}
