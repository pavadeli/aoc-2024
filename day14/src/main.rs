use common::{Itertools, SS, boilerplate, to_isize};
use pathfinding::grid::Grid;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone, Copy)]
struct Robot {
    pos: (isize, isize),
    dir: (isize, isize),
}

impl Robot {
    fn go(&self, secs: isize, width: isize, height: isize) -> Self {
        Self {
            pos: (
                (self.pos.0 + self.dir.0 * secs).rem_euclid(width),
                (self.pos.1 + self.dir.1 * secs).rem_euclid(height),
            ),
            ..*self
        }
    }
}

fn part1(input: SS, width: isize, height: isize) -> usize {
    let robots = parse(input).map(|r| r.go(100, width, height));
    calc_chaos(robots, width, height)
}

fn part2(input: SS, width: isize, height: isize) -> isize {
    // I have observed the tree by looking at an animation of `dump_grid` to be
    // clustered in one of the quadrants, so we should be able to use the method
    // of part1 to observe an unusual low "safety factor" when the tree appears.
    let robots = parse(input).collect_vec();
    let baseline_chaos = (0..100)
        .map(|s| {
            let robots = robots.iter().map(|r| r.go(s, width, height));
            calc_chaos(robots, width, height)
        })
        .min()
        .unwrap();
    // Taking 80% of the lowest value of the first 100 secs.
    let baseline_chaos = baseline_chaos * 4 / 5;
    let result = (101..width * height)
        .into_par_iter()
        .find_map_first(|s| {
            let robots = robots.iter().map(|r| r.go(s, width, height));
            (calc_chaos(robots, width, height) < baseline_chaos).then_some(s)
        })
        .expect("christmas tree not found");
    dump_grid(robots.into_iter().map(|r| r.go(result, width, height)));
    result
}

fn dump_grid(robots: impl IntoIterator<Item = Robot>) {
    let grid = Grid::from_coordinates(&robots.into_iter().map(|r| r.pos).collect_vec()).unwrap();
    eprintln!("{grid:?}");
}

fn parse(input: SS) -> impl Iterator<Item = Robot> {
    input.lines().map(|line| {
        let (p, v) = line.strip_prefix("p=").unwrap().split_once(" v=").unwrap();
        Robot {
            pos: p.split(',').map(to_isize).collect_tuple().unwrap(),
            dir: v.split(',').map(to_isize).collect_tuple().unwrap(),
        }
    })
}

fn calc_chaos(robots: impl IntoIterator<Item = Robot>, width: isize, height: isize) -> usize {
    let mut qs = [0; 4];
    let half_w = width / 2;
    let half_h = height / 2;
    for r in robots {
        let (x, y) = r.pos;
        if x != half_w && y != half_h {
            let idx = if x < half_w { 0 } else { 1 } + if y < half_h { 0 } else { 2 };
            qs[idx] += 1
        }
    }
    qs.into_iter().product()
}

boilerplate! {
    part1 => { test(11, 7) -> 12, real(101, 103) -> 220971520 }
    part2 => { real(101, 103) -> 6355 }
}
