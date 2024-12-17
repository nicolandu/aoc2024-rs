use std::fmt;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

pub mod template;

// Use this file to add helper functions and additional modules.

pub const NEIGHBOURS_ORTHOGONAL_VECTORS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
pub const NEIGHBOURS_DIAGONAL_VECTORS: [(isize, isize); 4] = [(-1, -1), (1, -1), (1, 1), (-1, 1)];
pub const NEIGHBOURS_ALL_VECTORS: [(isize, isize); 7] =
    [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    pub contents: Vec<Vec<T>>,
    pub width: isize,
    pub height: isize,
}

impl<T, U> From<Vec<Vec<U>>> for Grid<T>
where
    U: Into<T>,
{
    fn from(item: Vec<Vec<U>>) -> Self {
        Grid {
            height: item.len().try_into().unwrap(),
            width: item
                .first()
                .map(|row| row.len().try_into().unwrap())
                .unwrap_or(0),
            contents: item
                .into_iter()
                .map(|row| row.into_iter().map(|elem| elem.into()).collect())
                .collect(),
        }
    }
}

impl<T> Index<(isize, isize)> for Grid<T> {
    type Output = T;
    /// Index by (y,x).
    fn index(&self, pos: (isize, isize)) -> &Self::Output {
        self.get(pos).expect("Invalid index into Grid.")
    }
}

impl<T> IndexMut<(isize, isize)> for Grid<T> {
    /// Index by (y,x).
    fn index_mut(&mut self, pos: (isize, isize)) -> &mut Self::Output {
        self.get_mut(pos).expect("Invalid index into Grid.")
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(y, x)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    /// Parse 2D map into Grid<T>. transform takes F((y,x),char) -> T. Panics if lines are of unequal
    /// length.
    pub fn parse<F: FnMut((isize, isize), char) -> T>(input: &str, mut transform: F) -> Grid<T> {
        let g: Grid<T> = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| transform((y.try_into().unwrap(), x.try_into().unwrap()), c))
                    .collect()
            })
            .collect::<Vec<_>>()
            .into();
        assert!(
            g.contents
                .iter()
                .all(|row| row.len() == g.width.try_into().unwrap()),
            "Line length mismatch parsing grid."
        );
        g
    }

    /// Index by (y,x).
    pub fn is_inside(&self, p: (isize, isize)) -> bool {
        0 <= p.0 && p.0 < self.height && 0 <= p.1 && p.1 < self.width
    }

    /// Index by (y,x).
    pub fn get(&self, p: (isize, isize)) -> Option<&T> {
        if !self.is_inside(p) {
            return None;
        }
        self.contents
            .get(usize::try_from(p.0).unwrap())
            .and_then(|row| row.get(usize::try_from(p.1).unwrap()))
    }

    /// Index by (y,x).
    pub fn get_mut(&mut self, p: (isize, isize)) -> Option<&mut T> {
        if !self.is_inside(p) {
            return None;
        }
        self.contents
            .get_mut(usize::try_from(p.0).unwrap())
            .and_then(|row| row.get_mut(usize::try_from(p.1).unwrap()))
    }

    /// Iterator over ((y,x),&val).
    /// Index by (y,x).
    pub fn neighbours_orthogonal(
        &self,
        p: (isize, isize),
    ) -> impl Iterator<Item = ((isize, isize), &T)> + use<'_, T> {
        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(move |(dy, dx)| {
                let y = p.0 + dy;
                let x = p.1 + dx;
                self.get((y, x)).map(|v| ((y, x), v))
            })
    }

    /// Iterator over ((y,x),&val).
    pub fn neighbours_diagonal(
        &self,
        p: (isize, isize),
    ) -> impl Iterator<Item = ((isize, isize), &T)> + use<'_, T> {
        [(-1, -1), (1, -1), (1, 1), (-1, 1)]
            .into_iter()
            .filter_map(move |(dy, dx)| {
                let y = p.0 + dy;
                let x = p.1 + dx;
                self.get((y, x)).map(|v| ((y, x), v))
            })
    }

    /// Iterator over ((y,x),&val).
    pub fn neighbours_all(
        &self,
        p: (isize, isize),
    ) -> impl Iterator<Item = ((isize, isize), &T)> + use<'_, T> {
        [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)]
            .into_iter()
            .filter_map(move |(dy, dx)| {
                let y = p.0 + dy;
                let x = p.1 + dx;
                self.get((y, x)).map(|v| ((y, x), v))
            })
    }

    /// Returns an iterator over ((y,x),val). Consumes the Grid.
    pub fn into_iter_tiles(self) -> impl IntoIterator<Item = ((isize, isize), T)> {
        self.contents.into_iter().enumerate().flat_map(|(y, row)| {
            row.into_iter().enumerate().map(move |(x, elem)| {
                (
                    (isize::try_from(y).unwrap(), isize::try_from(x).unwrap()),
                    elem,
                )
            })
        })
    }

    /// Returns an iterator over ((y,x),&val).
    pub fn iter_tiles(&self) -> impl Iterator<Item = ((isize, isize), &T)> + use<'_, T> {
        self.contents.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, elem)| {
                (
                    (isize::try_from(y).unwrap(), isize::try_from(x).unwrap()),
                    elem,
                )
            })
        })
    }

    /// Returns an iterator over ((y,x),&mut val).
    pub fn iter_tiles_mut(
        &mut self,
    ) -> impl Iterator<Item = ((isize, isize), &mut T)> + use<'_, T> {
        self.contents.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut().enumerate().map(move |(x, elem)| {
                (
                    (isize::try_from(y).unwrap(), isize::try_from(x).unwrap()),
                    elem,
                )
            })
        })
    }

    /// Clone Grid<T> into Grid<U> with transform. transform takes F((y,x),&T) -> U.
    pub fn map_collect<U, F: FnMut((isize, isize), &T) -> U>(&self, mut transform: F) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            contents: self
                .contents
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, elem)| {
                            transform(
                                (isize::try_from(y).unwrap(), isize::try_from(x).unwrap()),
                                elem,
                            )
                        })
                        .collect()
                })
                .collect(),
        }
    }
}
