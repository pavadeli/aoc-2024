use common::*;
use std::{collections::HashSet, iter};

type RegionSetImpl = HashSet<Pos2>;

// plan:
// - obviously a grid
// - take a node, floodfill using dfs or bfs:
//   - make sure to not process those nodes again (set them to 0)
//   - amount of fences per block is 4 - nr of neighbours of same region

fn part1(input: SS) -> usize {
    regions(input)
        .map(|region| {
            let fences: usize = region
                .iter()
                .copied()
                .map(|block| {
                    DIRECTIONS_4
                        .iter()
                        .copied()
                        .filter(|&dir| !region_contains_neighbour(&region, block, dir))
                        .count()
                })
                .sum();

            fences * region.len()
        })
        .sum()
}

fn part2(input: SS) -> usize {
    // plan:
    // - get the grid
    // - also floodfill to get the regions
    // - count the starts en stops of fences, divided by two will be the actual
    //   fences, I guess

    // Examples: (`X` is the current region, `O` is another region)

    // The middle X has score 0, no fences start near that block.
    // O   O   O
    // - - - - -
    // X   X   X
    // - - - - -
    // O   O   O
    //

    // The middle X has score 4, one complete fence, and two "starts".
    // O   O   O
    //   +-- - -
    // O | X   X
    //   +-- - -
    // O   O   O
    //

    // The middle X here has score 2 (4 fence-ends, but only the upper-left two
    // actually touch the middle block)
    // O   O   O
    //   +-- - -
    // O | X   X
    //       +--
    // O | X | O

    // The middle X here has score 3 (one complete corner, and one corner that
    // is shared with the X in the top-right)
    // O   O | X
    //   +---+
    // O | X   X
    //       +--
    // O   X | O

    // So, the pattern is that you get 2 fence-ends for every diagonal where
    // both orthogonal neighbours are of a different region.
    regions(input)
        .map(|region| {
            let fences: usize = region
                .iter()
                .copied()
                .map(|block| -> usize {
                    DIRECTIONS_4
                        .iter()
                        .copied()
                        .circular_tuple_windows()
                        .map(|(a, b)| {
                            let n_a = region_contains_neighbour(&region, block, a);
                            let n_b = region_contains_neighbour(&region, block, b);
                            // If both are neighbours from our region then we
                            // don't have any fence heads to account for. If
                            // both are not neighbours from our region then we
                            // know that we have 2 fence heads. Otherwise we
                            // have to look at the diagonal.
                            if n_a && n_b {
                                0
                            } else if !n_a && !n_b {
                                2
                            } else if region_contains_neighbour(&region, block, a + b) {
                                1
                            } else {
                                0
                            }
                        })
                        .sum()
                })
                .sum();

            fences * region.len() / 2
        })
        .sum()
}

fn regions(input: SS) -> impl Iterator<Item = RegionSetImpl> {
    let mut grid = Grid::from(input);
    let mut keys = grid.keys();
    iter::from_fn(move || {
        for pos in &mut keys {
            let name = grid[pos];
            if name == 0 as char {
                continue;
            }
            let region: RegionSetImpl = grid
                .bfs_reachable(pos, Neighbourhood::Manhattan, |_, n| n == name)
                .collect();
            for &r_pos in &region {
                grid[r_pos] = 0 as char;
            }
            return Some(region);
        }
        None
    })
}

fn region_contains_neighbour(region: &RegionSetImpl, pos: Pos2, dir: Dir2) -> bool {
    Pos2::try_from(pos + dir).is_ok_and(|p| region.contains(&p))
}

boilerplate! {
    part1 => { test -> 1930, real -> 1431440 }
    part2 => { test -> 1206, real -> 869070 }
}
