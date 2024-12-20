use common::*;
use std::collections::HashSet;

fn go(input: SS, cheat_length: usize, target: usize) -> usize {
    let mut cheat_space = HashSet::from(DIRECTIONS_4);
    for _ in 1..cheat_length {
        cheat_space.extend(
            cheat_space
                .iter()
                .flat_map(|&a| DIRECTIONS_4.map(|b| a + b))
                .collect_vec(),
        );
    }
    let cheat_space = &cheat_space.into_iter().collect_vec();
    let (start, end, ref grid) = parse(input);
    let mut track = Grid::new(grid.rows(), grid.columns(), 0);
    let path = grid
        .astar_flat(start, end, Neighbourhood::Manhattan)
        .unwrap()
        .0
        .enumerate()
        .collect_vec();
    for &(cost, pos) in path.iter() {
        track[pos] = cost;
    }

    path.into_par_iter()
        .map(|(cost_start, cheat_start)| {
            cheat_space
                .iter()
                .flat_map(|&d| {
                    let cheat_length = d.0.unsigned_abs() + d.1.unsigned_abs();
                    let (cheat_end, cost_end) = track.step(cheat_start, d)?;
                    (cost_end >= cost_start + cheat_length + target).then_some(cheat_end)
                })
                .unique()
                .count()
        })
        .sum()
}

fn parse(input: SS) -> (Pos2, Pos2, Grid<bool>) {
    let grid = Grid::from(input);
    let start = grid.positions('S').exactly_one().ok().unwrap();
    let end = grid.positions('E').exactly_one().ok().unwrap();
    (start, end, grid.map(|ch| *ch != '#'))
}

boilerplate! {
    // part 1
    go => { test(2, 20) -> 5, real(2, 100) -> 1415 }
    // part 2
    go => { test(20, 50) -> 285, real(20, 100) -> 1022577 }
}
