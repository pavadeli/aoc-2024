use common::{Itertools, SS, boilerplate};
use std::{fmt::Debug, iter, ops::Add};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct P(isize, isize);

impl Add for P {
    type Output = P;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl From<(usize, usize)> for P {
    fn from((a, b): (usize, usize)) -> Self {
        Self(a as isize, b as isize)
    }
}

const E: P = P(1, 0);
const N: P = P(0, -1);
const W: P = P(-1, 0);
const S: P = P(0, 1);
const NE: P = P(1, -1);
const NW: P = P(-1, -1);
const SW: P = P(-1, 1);
const SE: P = P(1, 1);

const DIRECTIONS: &[P] = &[E, N, W, S, NE, NW, SW, SE];

struct Grid {
    bytes: Vec<&'static [u8]>,
}

impl Grid {
    fn new(input: SS) -> Self {
        let bytes = input.lines().map(|line| line.as_bytes()).collect_vec();
        Self { bytes }
    }

    fn get(&self, p: P) -> Option<char> {
        let (x, y): (usize, usize) = (p.0.try_into().ok()?, p.1.try_into().ok()?);
        self.bytes.get(y)?.get(x).map(|&b| b as char)
    }

    fn positions(&self, c: &'static char) -> impl Iterator<Item = P> + '_ {
        self.bytes.iter().enumerate().flat_map(move |(y, row)| {
            row.iter()
                .copied()
                .positions(|b| b == *c as u8)
                .map(move |x| (x, y).into())
        })
    }

    fn vector(&self, mut from: P, dir: P) -> impl Iterator<Item = char> + '_ {
        iter::from_fn(move || {
            let b = self.get(from)?;
            from = from + dir;
            Some(b)
        })
    }
}

fn part1(input: SS) -> usize {
    let grid = Grid::new(input);
    grid.positions(&'X')
        .map(|from| {
            DIRECTIONS
                .iter()
                .filter(|&&dir| grid.vector(from, dir).take(4).eq(['X', 'M', 'A', 'S']))
                .count()
        })
        .sum()
}

fn part2(input: SS) -> usize {
    let grid = Grid::new(input);
    grid.positions(&'A')
        .filter(|&from| {
            matches!((
                grid.get(from + NE),
                grid.get(from + NW),
                grid.get(from + SW),
                grid.get(from + SE)
            ), (
                Some(ne @ ('M'|'S')),
                Some(nw @ ('M'|'S')),
                Some(sw @ ('M'|'S')),
                Some(se @ ('M'|'S'))
            ) if ne != sw && nw != se)
        })
        .count()
}

boilerplate! {
    part1 => { test -> 18, real -> 2468 }
    part2 => { test -> 9, real -> 1864 }
}
