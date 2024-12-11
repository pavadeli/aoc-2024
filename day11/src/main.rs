#![allow(non_upper_case_globals)]

use common::{SS, Solution, boilerplate};
use std::{array, collections::HashMap};

const part1: Solution = go::<25>;
const part2: Solution = go::<75>;

type Mark = u64;
type Count = usize;
type MemoMap = HashMap<Mark, Count>;

const MARK_10: Mark = 10;

fn go<const ROUNDS: usize>(input: SS) -> Count {
    let mut memo: [MemoMap; ROUNDS] = array::from_fn(|_| MemoMap::new());

    input
        .split_whitespace()
        .map(|s| simulate_stone(&mut memo, s.parse().unwrap(), 0))
        .sum()
}

fn simulate_stone<const ROUNDS: usize>(
    memo: &mut [MemoMap; ROUNDS],
    mark: Mark,
    round: usize,
) -> Count {
    if round == ROUNDS {
        return 1;
    }
    if let Some(&r) = memo[round].get(&mark) {
        return r;
    }
    let r = if mark == 0 {
        simulate_stone(memo, 1, round + 1)
    } else {
        let digits = mark.ilog10() + 1;
        if digits % 2 == 0 {
            let split = MARK_10.pow(digits / 2);
            simulate_stone(memo, mark / split, round + 1)
                + simulate_stone(memo, mark % split, round + 1)
        } else {
            simulate_stone(memo, mark * 2024, round + 1)
        }
    };
    memo[round].insert(mark, r);
    r
}

boilerplate! {
    part1 => { test -> 55312, real -> 218956 }
    part2 => { test -> 65601038650482, real -> 259593838049805 }
}
