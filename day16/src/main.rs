use common::*;

const ROTATE_COST: usize = 1000;
const STEP_COST: usize = 1;

fn part1(input: SS) -> usize {
    let (grid, start, end) = parse(input);
    let (_, cost) = pathfinding::astar(
        &(start, E),
        |(pos, dir)| successors(&grid, *pos, *dir),
        |(pos, _)| pos.abs_diff(end),
        |(p, _)| *p == end,
    )
    .unwrap();
    cost
}

fn part2(input: SS) -> usize {
    let (grid, start, end) = parse(input);
    let (paths, _) = pathfinding::astar_bag(
        &(start, E),
        |&(pos, dir)| successors(&grid, pos, dir),
        |(pos, _)| pos.abs_diff(end),
        |(p, _)| *p == end,
    )
    .unwrap();
    paths.flatten().map(first).unique().count()
}

fn parse(input: &str) -> (Grid, Pos2, Pos2) {
    let grid = Grid::from(input);
    let start = grid.positions('S').exactly_one().ok().unwrap();
    let end = grid.positions('E').exactly_one().ok().unwrap();
    (grid, start, end)
}

fn successors(grid: &Grid, pos: Pos2, dir: Dir2) -> impl Iterator<Item = ((Pos2, Dir2), usize)> {
    let (step, found) = grid.step(pos, dir).unwrap();
    (found != '#')
        .then_some(((step, dir), STEP_COST))
        .into_iter()
        .chain([
            ((pos, dir.rotate_90_cw()), ROTATE_COST),
            ((pos, dir.rotate_90_ccw()), ROTATE_COST),
        ])
}

boilerplate! {
    part1 => { test1 -> 7036, test2 -> 11048, real -> 143564 }
    part2 => { test1 -> 45, test2 -> 64, real -> 593 }
}
