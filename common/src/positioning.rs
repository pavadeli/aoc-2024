use std::{
    iter::Sum,
    ops::{Add, AddAssign, Sub},
};

/// East
pub const E: Dir2 = Dir2(0, 1);

/// South
pub const S: Dir2 = Dir2(1, 0);

/// West
pub const W: Dir2 = Dir2(0, -1);

/// North
pub const N: Dir2 = Dir2(-1, 0);

/// North-East
pub const NE: Dir2 = Dir2(-1, 1);

/// South-East
pub const SE: Dir2 = Dir2(1, 1);

/// North-West
pub const NW: Dir2 = Dir2(-1, -1);

/// South-West
pub const SW: Dir2 = Dir2(1, -1);

/// Four main directions
pub const DIRECTIONS_4: [Dir2; 4] = [E, S, W, N];

/// Eight main directions with diagonals
pub const DIRECTIONS_8: [Dir2; 8] = [NE, E, SE, S, SW, W, NW, N];

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct Pos2(pub usize, pub usize);

impl Pos2 {
    pub fn saturating_add_dir(self, dir: Dir2) -> Self {
        Self(
            self.0.saturating_add_signed(dir.0),
            self.1.saturating_add_signed(dir.1),
        )
    }

    pub fn abs_diff(self, other: Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

impl From<(usize, usize)> for Pos2 {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<Pos2> for (usize, usize) {
    fn from(value: Pos2) -> Self {
        (value.0, value.1)
    }
}

impl TryFrom<(isize, isize)> for Pos2 {
    type Error = <usize as TryFrom<isize>>::Error;

    fn try_from(value: (isize, isize)) -> Result<Self, Self::Error> {
        Ok(Self(value.0.try_into()?, value.1.try_into()?))
    }
}

impl TryFrom<Dir2> for Pos2 {
    type Error = <usize as TryFrom<isize>>::Error;

    fn try_from(value: Dir2) -> Result<Self, Self::Error> {
        Ok(Self(value.0.try_into()?, value.1.try_into()?))
    }
}

impl Add for Pos2 {
    type Output = Self;

    fn add(self, rhs: Pos2) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Dir2> for Pos2 {
    type Output = Dir2;

    fn add(self, rhs: Dir2) -> Self::Output {
        let lhs: Dir2 = self.into();
        Dir2(lhs.0 + rhs.0, lhs.1 + rhs.1)
    }
}

impl Sub for Pos2 {
    type Output = Dir2;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs: Dir2 = self.into();
        let rhs = rhs.into();
        lhs - rhs
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct Dir2(pub isize, pub isize);

impl Dir2 {
    /// Rotate 90° counter clockwise
    pub fn rotate_90_ccw(self) -> Self {
        Self(-self.1, self.0)
    }

    /// Rotate 90° clockwise
    pub fn rotate_90_cw(self) -> Self {
        Self(self.1, -self.0)
    }
}

impl From<(isize, isize)> for Dir2 {
    fn from(value: (isize, isize)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<(usize, usize)> for Dir2 {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0 as isize, value.1 as isize)
    }
}

impl From<Dir2> for (isize, isize) {
    fn from(value: Dir2) -> Self {
        (value.0, value.1)
    }
}

impl TryFrom<Dir2> for (usize, usize) {
    type Error = <usize as TryFrom<isize>>::Error;

    fn try_from(value: Dir2) -> Result<Self, Self::Error> {
        Ok((value.0.try_into()?, value.1.try_into()?))
    }
}

impl From<Pos2> for Dir2 {
    fn from(value: Pos2) -> Self {
        Dir2(value.0 as isize, value.1 as isize)
    }
}

impl Add for Dir2 {
    type Output = Self;

    fn add(self, rhs: Dir2) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Dir2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Dir2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl AddAssign for Dir2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sum for Dir2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Dir2::default(), |a, b| a + b)
    }
}
