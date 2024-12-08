use common::{Grid, Itertools, Pos2, SS, boilerplate, first, swap};
use std::collections::{HashMap, HashSet};

fn part1(input: SS) -> usize {
    let (grid, perms) = parse(input);
    perms
        .filter_map(|[a, b]| grid.step(b, b - a))
        .unique()
        .count()
}

fn part2(input: SS) -> usize {
    let (grid, perms) = parse(input);
    perms
        .flat_map(|[a, b]| grid.walk(a, b - a))
        .map(first)
        .unique()
        .count()
}

fn parse(input: SS) -> (Grid, impl Iterator<Item = [Pos2; 2]>) {
    let grid: Grid = input.into();
    let antennas: HashMap<char, HashSet<_>> = grid
        .items()
        .filter(|(_, ch)| *ch != '.')
        .map(swap)
        .into_grouping_map()
        .collect();
    let perms = antennas.into_values().flat_map(|set| {
        set.into_iter()
            .permutations(2)
            .map(|perm| perm.try_into().unwrap())
    });
    (grid, perms)
}

boilerplate! {
    part1 => { test -> 14, real -> 400 }
    part2 => { test -> 34, real -> 1280 }
}
