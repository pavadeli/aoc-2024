use common::*;

fn part1(input: &str) -> usize {
    let (mut left, mut right) = get_lists(input);
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn part2(input: &str) -> usize {
    let (left, right) = get_lists(input);
    let right = right.into_iter().counts();
    left.into_iter()
        .map(|l| l * right.get(&l).copied().unwrap_or_default())
        .sum()
}

fn get_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(to_usize)
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

boilerplate! {
    part1 => { test -> 11, real -> 1834060 }
    part2 => { test -> 31, real -> 21607792 }
}
