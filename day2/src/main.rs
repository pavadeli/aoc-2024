use common::{boilerplate, to_isize, Itertools};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
}

fn part1(input: &str) -> usize {
    reports(input)
        .filter(|report| matches!(directions(report).all_equal_value(), Ok(Some(_))))
        .count()
}

fn part2(input: &str) -> usize {
    reports(input)
        .filter(|report| try_trend(report, Direction::Up) || try_trend(report, Direction::Down))
        .count()
}

fn try_trend(report: &[isize], trend: Direction) -> bool {
    let Some(bad_idx) = directions(report).position(|dir| dir != Some(trend)) else {
        return true;
    };
    eval_without_idx(report, bad_idx, trend) || eval_without_idx(report, bad_idx + 1, trend)
}

fn eval_without_idx(levels: &[isize], idx: usize, trend: Direction) -> bool {
    let (left, right) = levels.split_at(idx);
    let right = &right[1..]; // remove the bad apple
    let glue = match (left.last(), right.first()) {
        (Some(&l), Some(&r)) => Some((l, r)),
        _ => None,
    };

    let eval_trend =
        |levels: &[isize]| levels.len() < 2 || directions(levels).all(|d| d == Some(trend));

    glue.is_none_or(|t| direction_tuple(t) == Some(trend)) && eval_trend(left) && eval_trend(right)
}

fn reports(input: &str) -> impl Iterator<Item = Vec<isize>> + '_ {
    input
        .lines()
        .map(|line| line.split_whitespace().map(to_isize).collect())
}

fn directions(levels: &[isize]) -> impl Iterator<Item = Option<Direction>> + '_ {
    levels.iter().copied().tuple_windows().map(direction_tuple)
}

fn direction_tuple((a, b): (isize, isize)) -> Option<Direction> {
    match b - a {
        1..=3 => Some(Direction::Up),
        -3..=-1 => Some(Direction::Down),
        _ => None,
    }
}

boilerplate! {
    part1 => { test -> 2, real -> 299 }
    part2 => { test -> 4, real -> 364 }
}
