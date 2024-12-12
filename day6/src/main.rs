use common::{Dir2, Grid, Itertools, N, Pos2, SS, boilerplate};
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
            grid[extra] = '#';
            let loops = !walk_grid(grid, pos, dir).all_unique();
            grid[extra] = '.';
            if loops { 1 } else { 0 }
        })
        .sum()
}

fn parse_grid(input: SS) -> (Grid, Pos2, Dir2) {
    let grid: Grid = input.into();
    let pos = grid.positions('^').next().expect("actor not found");
    (grid, pos, N)
}

fn walk_grid(
    grid: &Grid,
    mut pos: Pos2,
    mut dir: Dir2,
) -> impl Iterator<Item = (Pos2, Dir2)> + use<'_> {
    iter::from_fn(move || {
        loop {
            let (next, ch) = grid.step(pos, dir)?;
            if ch != '#' {
                pos = next;
                return Some((pos, dir));
            }
            dir = dir.rotate_90_cw();
        }
    })
}

boilerplate! {
    part1 => { test -> 41, real -> 5067 }
    part2 => { test -> 6, real -> 1793 }
}
