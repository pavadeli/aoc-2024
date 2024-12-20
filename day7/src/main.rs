use common::*;

fn part1(input: SS) -> usize {
    go(input, &[|a, b| a + b, |a, b| a * b])
}

fn part2(input: SS) -> usize {
    go(input, &[
        |a, b| a + b,
        |a, b| a * b,
        |a, b| a * 10_usize.pow(b.ilog10() + 1) + b,
    ])
}

fn go(input: SS, ops: &[fn(usize, usize) -> usize]) -> usize {
    input
        .lines()
        .par_bridge()
        .filter_map(|line| {
            let (target, nrs) = line.split_once(": ").unwrap();
            let target = to_usize(target);
            let nrs = nrs.split_whitespace().map(to_usize).collect_vec();
            let (&first, nrs) = nrs.split_first().unwrap();
            try_operators(first, target, nrs, ops).then_some(target)
        })
        .sum()
}

fn try_operators(
    cur: usize,
    target: usize,
    nrs: &[usize],
    ops: &[fn(usize, usize) -> usize],
) -> bool {
    let Some((&next, nrs)) = nrs.split_first() else {
        return cur == target;
    };
    // I've observed no `0` in the operands in the input, so all intermediate
    // results must always be equal or greater than `cur`, therefore we can stop
    // if it exceeds `target`.
    ops.iter()
        .map(|op| op(cur, next))
        .filter(|&v| v <= target)
        .any(|val| try_operators(val, target, nrs, ops))
}

boilerplate! {
    part1 => { test -> 3749, real -> 975671981569 }
    part2 => { test -> 11387, real -> 223472064194845 }
}
