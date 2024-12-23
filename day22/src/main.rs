use common::*;
use std::collections::HashMap;

fn part1(input: SS) -> isize {
    input
        .lines()
        .par_bridge()
        .map(to_isize)
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = next_secret(secret);
            }
            secret
        })
        .sum()
}

fn part2(input: SS) -> isize {
    let map = input
        .lines()
        .par_bridge()
        .map(to_isize)
        .map(|secret| {
            let mut map = HashMap::with_capacity(2000);
            std::iter::successors(Some(secret), |&secret| Some(next_secret(secret)))
                .take(2000)
                .map(|n| n % 10)
                .tuple_windows()
                .map(|(a, b, c, d, e)| ((((b - a) * 18 + (c - b)) * 18 + (d - c)) * 18 + e - d, e))
                .for_each(|(k, v)| {
                    map.entry(k).or_insert(v);
                });
            map
        })
        .reduce_with(|mut a, b| {
            for (k, v) in b {
                a.entry(k).and_modify(|w| *w += v).or_insert(v);
            }
            a
        })
        .unwrap();
    map.into_values().max().unwrap()
}

fn next_secret(secret: isize) -> isize {
    let secret = ((secret * 64) ^ secret) % 16777216;
    let secret = ((secret / 32) ^ secret) % 16777216;
    ((secret * 2048) ^ secret) % 16777216
}

boilerplate! {
    part1 => { test1 -> 37327623, real -> 20411980517 }
    part2 => { test2 -> 23, real -> 2362 }
}
