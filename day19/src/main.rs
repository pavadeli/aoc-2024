use common::*;
use regex::{Regex, RegexSet};

fn part1(input: SS) -> usize {
    let (patterns, designs) = parse(input);
    let regex = Regex::new(&format!("^({})+$", patterns.join("|"))).unwrap();
    designs.filter(|d| regex.is_match(d)).count()
}

fn part2(input: SS) -> usize {
    let (ref patterns, designs) = parse(input);
    let regexes = RegexSet::new(patterns.iter().map(|p| format!("^{p}"))).unwrap();
    designs
        .map(|design| {
            pathfinding::count_paths(
                0,
                |&i| {
                    regexes
                        .matches(&design[i..])
                        .into_iter()
                        .map(move |p_idx| i + patterns[p_idx].len())
                        .filter(|&i| i <= design.len())
                },
                |&i| i == design.len(),
            )
        })
        .sum()
}

fn parse(input: SS) -> (Vec<SS>, impl Iterator<Item = SS>) {
    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").collect();
    lines.next().unwrap();
    (patterns, lines)
}

boilerplate! {
    part1 => { test -> 6, real -> 255 }
    part2 => { test -> 16, real -> 621820080273474 }
}
