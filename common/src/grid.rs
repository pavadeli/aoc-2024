use crate::{Dir2, Pos2};
use pathfinding::matrix::Matrix;
use std::{
    fmt::{Debug, Write},
    iter,
    ops::{Index, IndexMut},
};

#[derive(PartialEq, Eq, Hash)]
pub struct Grid(Matrix<char>);

impl Clone for Grid {
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

impl Grid {
    pub fn get(&self, pos: impl TryInto<Pos2>) -> Option<char> {
        self.0.get(pos.try_into().ok()?.into()).copied()
    }

    pub fn positions(&self, ch: char) -> impl Iterator<Item = Pos2> + use<'_> {
        self.0
            .items()
            .filter(move |(_, b)| (**b == ch))
            .map(|(pos, _)| pos.into())
    }

    pub fn walk(&self, start: Pos2, dir: Dir2) -> impl Iterator<Item = (Pos2, char)> + use<'_> {
        iter::once(start.into())
            .chain(self.0.in_direction(start.into(), dir.into()))
            .map(|pos| (pos.into(), self.0[pos]))
    }

    pub fn step(&self, start: Pos2, dir: Dir2) -> Option<(Pos2, char)> {
        let pos = self.0.move_in_direction(start.into(), dir.into())?;
        Some((pos.into(), self.0[pos]))
    }

    pub fn items(&self) -> impl Iterator<Item = (Pos2, char)> + use<'_> {
        self.0.items().map(|(pos, &b)| (pos.into(), b))
    }

    pub fn neighbours(
        &self,
        pos: Pos2,
        neighbourhood: Neighbourhood,
    ) -> impl Iterator<Item = (Pos2, char)> + use<'_> {
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

    pub fn row_iter(&self) -> impl Iterator<Item = &[char]> {
        self.0.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = Pos2> {
        self.0.keys().map(Pos2::from)
    }

    pub fn bfs_reachable(
        &self,
        start: Pos2,
        neighbourhood: Neighbourhood,
        mut predicate: impl FnMut(Pos2, char) -> bool,
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
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self(value.lines().map(str::chars).collect())
    }
}

impl<IC> FromIterator<IC> for Grid
where
    IC: IntoIterator<Item = char>,
{
    fn from_iter<T: IntoIterator<Item = IC>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Index<Pos2> for Grid {
    type Output = <Matrix<char> as Index<(usize, usize)>>::Output;

    fn index(&self, index: Pos2) -> &Self::Output {
        let index: (usize, usize) = index.into();
        self.0.index(index)
    }
}

impl IndexMut<Pos2> for Grid {
    fn index_mut(&mut self, index: Pos2) -> &mut Self::Output {
        let index: (usize, usize) = index.into();
        self.0.index_mut(index)
    }
}

impl Debug for Grid {
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
