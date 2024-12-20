use crate::{Dir2, Pos2};
use pathfinding::{matrix::Matrix, prelude::astar};
use std::{
    fmt::{Debug, Write},
    iter,
    ops::{Index, IndexMut},
};

#[derive(PartialEq, Eq, Hash)]
pub struct Grid<T = char>(Matrix<T>);

impl<T: Copy> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        if self.0.columns != source.0.columns || self.0.rows != source.0.rows {
            *self = source.clone();
        } else {
            self.0.copy_from_slice(&source.0);
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Neighbourhood {
    /// Only the 4 cardinal directions.
    Manhattan,
    /// Cardinal + diagonal directions.
    All,
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn new(rows: usize, columns: usize, value: T) -> Self {
        Self(Matrix::new(rows, columns, value))
    }

    pub fn get(&self, pos: impl TryInto<Pos2>) -> Option<T> {
        self.0.get(pos.try_into().ok()?.into()).copied()
    }

    pub fn positions(&self, ch: T) -> impl Iterator<Item = Pos2> + use<'_, T> {
        self.0
            .items()
            .filter(move |(_, b)| (**b == ch))
            .map(|(pos, _)| pos.into())
    }

    pub fn walk(&self, start: Pos2, dir: Dir2) -> impl Iterator<Item = (Pos2, T)> + use<'_, T> {
        iter::once(start.into())
            .chain(self.0.in_direction(start.into(), dir.into()))
            .map(|pos| (pos.into(), self.0[pos]))
    }

    pub fn step(&self, start: Pos2, dir: Dir2) -> Option<(Pos2, T)> {
        let pos = self.0.move_in_direction(start.into(), dir.into())?;
        Some((pos.into(), self.0[pos]))
    }

    pub fn items(&self) -> impl Iterator<Item = (Pos2, T)> + use<'_, T> {
        self.0.items().map(|(pos, &b)| (pos.into(), b))
    }

    pub fn neighbours(
        &self,
        pos: Pos2,
        neighbourhood: Neighbourhood,
    ) -> impl Iterator<Item = (Pos2, T)> + use<'_, T> {
        self.0
            .neighbours(pos.into(), neighbourhood == Neighbourhood::All)
            .map(|pos| (pos.into(), self.0[pos]))
    }

    pub fn columns(&self) -> usize {
        self.0.columns
    }

    pub fn rows(&self) -> usize {
        self.0.rows
    }

    pub fn row_iter(&self) -> impl Iterator<Item = &[T]> {
        self.0.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = Pos2> {
        self.0.keys().map(Pos2::from)
    }

    pub fn bfs_reachable(
        &self,
        start: Pos2,
        neighbourhood: Neighbourhood,
        mut predicate: impl FnMut(Pos2, T) -> bool,
    ) -> impl Iterator<Item = Pos2> {
        let set = self
            .0
            .bfs_reachable(start.into(), neighbourhood == Neighbourhood::All, |p| {
                predicate(p.into(), self.0[p])
            });
        set.into_iter().map(Pos2::from)
    }

    pub fn swap(&mut self, a: Pos2, b: Pos2) {
        self.0.swap(a.into(), b.into());
    }

    pub fn map<R>(&self, f: impl FnMut(&T) -> R) -> Grid<R> {
        Grid(
            Matrix::from_vec(
                self.0.rows,
                self.0.columns,
                self.0.as_ref().iter().map(f).collect(),
            )
            .unwrap(),
        )
    }
}

impl Grid<bool> {
    pub fn astar_flat(
        &self,
        start: Pos2,
        end: Pos2,
        neighbourhood: Neighbourhood,
    ) -> Option<(impl Iterator<Item = Pos2>, usize)> {
        let (route, cost) = astar(
            &start,
            |&n| {
                self.neighbours(n, neighbourhood)
                    .filter(|(_, ok)| *ok)
                    .map(|(p, _)| (p, 1))
            },
            |&n| n.abs_diff(end),
            |&n| n == end,
        )?;
        Some((route.into_iter().map(Pos2::from), cost))
    }
}

impl From<&str> for Grid<char> {
    fn from(value: &str) -> Self {
        Self(value.lines().map(str::chars).collect())
    }
}

impl<IC, T> FromIterator<IC> for Grid<T>
where
    IC: IntoIterator<Item = T>,
{
    fn from_iter<I: IntoIterator<Item = IC>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T> Index<Pos2> for Grid<T> {
    type Output = <Matrix<T> as Index<(usize, usize)>>::Output;

    fn index(&self, index: Pos2) -> &Self::Output {
        let index: (usize, usize) = index.into();
        self.0.index(index)
    }
}

impl<T> IndexMut<Pos2> for Grid<T> {
    fn index_mut(&mut self, index: Pos2) -> &mut Self::Output {
        let index: (usize, usize) = index.into();
        self.0.index_mut(index)
    }
}

impl Debug for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for &ch in row {
                f.write_char(ch)?
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}

impl Debug for Grid<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for &b in row {
                f.write_char(if b { '.' } else { '#' })?
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}
