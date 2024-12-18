use common::{Itertools, SS, boilerplate, to_usize};
use pathfinding::{grid::Grid, prelude::*};

fn part1(input: SS, size: usize, fallen_bytes: usize) -> usize {
    let mut grid = Grid::new(size, size);
    grid.fill();
    for pos in parse(input).take(fallen_bytes) {
        grid.remove_vertex(pos);
    }
    let target = (size - 1, size - 1);
    get_route(&grid, target).unwrap().1
}

fn part2(input: SS, size: usize, known_ok_value: usize) -> String {
    let mut grid = Grid::new(size, size);
    grid.fill();
    let mut iter = parse(input);
    for pos in iter.by_ref().take(known_ok_value) {
        grid.remove_vertex(pos);
    }
    let target = (size - 1, size - 1);
    let mut optimal_route = get_route(&grid, target).unwrap().0;
    for pos in iter {
        grid.remove_vertex(pos);
        if optimal_route.contains(&pos) {
            let Some((route, _)) = get_route(&grid, target) else {
                return format!("{},{}", pos.0, pos.1);
            };
            optimal_route = route;
        }
    }
    panic!("no obstruction found")
}

fn get_route(grid: &Grid, target: (usize, usize)) -> Option<(Vec<(usize, usize)>, usize)> {
    astar(
        &(0, 0),
        |p| grid.neighbours(*p).into_iter().map(|p| (p, 1)),
        |p| grid.distance(*p, target),
        |p| *p == target,
    )
}

fn parse(input: SS) -> impl Iterator<Item = (usize, usize)> {
    input
        .lines()
        .map(|line| line.split(',').map(to_usize).collect_tuple().unwrap())
}

boilerplate! {
    part1 => { test(7, 12) -> 22, real(71, 1024) -> 276 }
    part2 => { test(7, 12) -> "6,1", real(71, 1024) -> "60,37" }
}
