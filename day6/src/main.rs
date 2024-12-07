use common::{Itertools, SS, boilerplate};
use pathfinding::matrix::{Matrix, directions};
use rayon::prelude::*;
use std::iter;

fn part1(input: SS) -> usize {
    let (grid, pos, dir) = parse_grid(input);
    // count all unique positions that are encountered during a walk
    walk_grid(&grid, pos, dir)
        .map(|(pos, _)| pos)
        // including the start position (just to be sure, not needed for my input)
        .chain(iter::once(pos))
        .unique()
        .count()
}

fn part2(input: SS) -> usize {
    let (grid, pos, dir) = parse_grid(input);
    // first collect all unique positions to place an extra obstruction,
    // including the position and direction we came from the first time we
    // encountered this position on the grid
    walk_grid(&grid, pos, dir)
        .tuple_windows()
        .unique_by(|((_, _), (extra, _))| *extra)
        .collect_vec()
        // then process this list in parallel
        .into_par_iter()
        .map_with(grid, |grid, ((pos, dir), (extra, _))| {
            grid[extra] = b'#';
            let loops = !walk_grid(grid, pos, dir).all_unique();
            grid[extra] = b'.';
            if loops { 1 } else { 0 }
        })
        .sum()
}

fn parse_grid(input: SS) -> (Matrix<u8>, (usize, usize), (isize, isize)) {
    let grid: Matrix<u8> = input.lines().map(|line| line.bytes()).collect();
    let (pos, _) = grid
        .items()
        .find(|&(_, &b)| b == b'^')
        .expect("actor not found");
    (grid, pos, directions::N)
}

fn walk_grid(
    grid: &Matrix<u8>,
    mut pos: (usize, usize),
    mut dir: (isize, isize),
) -> impl Iterator<Item = ((usize, usize), (isize, isize))> + use<'_> {
    iter::from_fn(move || {
        loop {
            let next = grid.move_in_direction(pos, dir)?;
            if grid[next] != b'#' {
                pos = next;
                return Some((pos, dir));
            }
            dir = (dir.1, -dir.0); // rotate 90°
        }
    })
}

boilerplate! {
    part1 => { test -> 41, real -> 5067 }
    part2 => { test -> 6, real -> 1793 }
}
