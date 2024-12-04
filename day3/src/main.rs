use common::{boilerplate, to_usize};
use regex::Regex;
use std::sync::LazyLock;

static MULS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

fn part1(input: &str) -> usize {
    MULS.captures_iter(input)
        .map(|m| m.extract())
        .map(|(_, [a, b])| to_usize(a) * to_usize(b))
        .sum()
}

fn part2(input: &str) -> usize {
    Regex::new(r"(?:^|do\(\))((?s).*?)(?:$|don't\(\))")
        .unwrap()
        .captures_iter(input)
        .map(|m| m.extract())
        .map(|(_, [segment])| part1(segment))
        .sum()
}

boilerplate! {
    part1 => { test1 -> 161, test2 -> 161, real -> 164730528 }
    part2 => { test1 -> 161, test2 -> 48, real -> 70478672 }
}
