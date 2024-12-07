use common::{SS, boilerplate};
use std::{cmp, collections::HashSet};

type Page = u8;
type Rule = [Page; 2];
type Rules = HashSet<Rule>;

fn part1(input: SS) -> usize {
    let (rules, updates) = parse(input);
    updates
        .filter(|pages| {
            pages.is_sorted_by(|&a, &b| {
                let result = rules.contains(&[a, b]);
                // Apparently we have a total ordering. Thank you, that makes it
                // much easier! I've removed more complicated code that could
                // handle partial orderings.
                assert_eq!(result, !rules.contains(&[b, a]));
                result
            })
        })
        .map(|pages| pages[pages.len() / 2] as usize)
        .sum()
}

fn part2(input: SS) -> usize {
    let (rules, updates) = parse(input);
    updates
        .filter_map(|mut pages| {
            if pages.is_sorted_by(|&a, &b| rules.contains(&[a, b])) {
                return None;
            }
            pages.sort_by(|&a, &b| {
                if rules.contains(&[a, b]) {
                    cmp::Ordering::Less
                } else {
                    cmp::Ordering::Greater
                }
            });
            Some(pages)
        })
        .map(|pages| pages[pages.len() / 2] as usize)
        .sum()
}

fn parse(input: SS) -> (Rules, impl Iterator<Item = Vec<Page>>) {
    let (rule_lines, update_lines) = input.split_once("\n\n").unwrap();
    let rules = rule_lines
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('|').unwrap();
            [left.parse().unwrap(), right.parse().unwrap()]
        })
        .collect();
    let updates = update_lines
        .lines()
        .map(|line| line.split(',').map(|nr| nr.parse().unwrap()).collect());
    (rules, updates)
}

boilerplate! {
    part1 => { test -> 143, real -> 5639 }
    part2 => { test -> 123, real -> 5273 }
}
