use crate::{Dir2, Pos2};
use pathfinding::matrix::Matrix;
use std::{
    iter,
    ops::{Index, IndexMut},
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid(Matrix<u8>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Neighbourhood {
    /// Only the 4 cardinal directions.
    Manhattan,
    /// Cardinal + diagonal directions.
    All,
}

impl Grid {
    pub fn get(&self, pos: impl TryInto<Pos2>) -> Option<char> {
        self.0
            .get(pos.try_into().ok()?.into())
            .copied()
            .map(char::from)
    }

    pub fn positions(&self, ch: char) -> impl Iterator<Item = Pos2> + use<'_> {
        let ch = ch as u8;
        self.0
            .items()
            .filter(move |(_, b)| (**b == ch))
            .map(|(pos, _)| pos.into())
    }

    pub fn walk(&self, start: Pos2, dir: Dir2) -> impl Iterator<Item = (Pos2, char)> + use<'_> {
        iter::once(start.into())
            .chain(self.0.in_direction(start.into(), dir.into()))
            .map(|pos| (pos.into(), self.0[pos] as char))
    }

    pub fn step(&self, start: Pos2, dir: Dir2) -> Option<(Pos2, char)> {
        let pos = self.0.move_in_direction(start.into(), dir.into())?;
        Some((pos.into(), self.0[pos] as char))
    }

    pub fn items(&self) -> impl Iterator<Item = (Pos2, char)> + use<'_> {
        self.0.items().map(|(pos, &b)| (pos.into(), b as char))
    }

    pub fn neighbours(
        &self,
        pos: Pos2,
        neighbourhood: Neighbourhood,
    ) -> impl Iterator<Item = (Pos2, char)> + use<'_> {
        self.0
            .neighbours(pos.into(), neighbourhood == Neighbourhood::All)
            .map(|pos| (pos.into(), self.0[pos] as char))
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self(value.lines().map(str::bytes).collect())
    }
}

impl Index<Pos2> for Grid {
    type Output = <Matrix<u8> as Index<(usize, usize)>>::Output;

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
