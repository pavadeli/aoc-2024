use common::{Grid, Neighbourhood, Pos2, SS, boilerplate};
use pathfinding::prelude::*;

fn part1(input: SS) -> usize {
    let grid: Grid = input.into();
    grid.positions('0')
        .map(|head| {
            dfs_reach((head, '0'), |&(pos, level)| {
                valid_neighbours(&grid, pos, level)
            })
            .filter(|(_, ch)| *ch == '9')
            .count()
        })
        .sum()
}

fn part2(input: SS) -> usize {
    let grid: Grid = input.into();
    grid.positions('0')
        .map(|head| {
            count_paths(
                (head, '0'),
                |&(pos, level)| valid_neighbours(&grid, pos, level),
                |(_, ch)| *ch == '9',
            )
        })
        .sum()
}

fn valid_neighbours(
    grid: &Grid,
    pos: Pos2,
    level: char,
) -> impl Iterator<Item = (Pos2, char)> + use<'_> {
    grid.neighbours(pos, Neighbourhood::Manhattan)
        .filter(move |(_, next_lvl)| *next_lvl as u32 == level as u32 + 1)
}

boilerplate! {
    part1 => { test -> 36, real -> 531 }
    part2 => { test -> 81, real -> 1210 }
}
