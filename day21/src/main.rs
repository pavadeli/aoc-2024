#![allow(non_upper_case_globals)]

use common::*;
use std::{array, collections::HashMap, iter, sync::LazyLock};

/// ```text
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
/// ```
static NUM_KEYPAD: LazyLock<Grid> = LazyLock::new(|| Grid::from("789\n456\n123\n 0A"));

/// ```text
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
/// ```
static DIR_KEYPAD: LazyLock<Grid> = LazyLock::new(|| Grid::from(" ^A\n<v>"));

struct Robot {
    keypad: &'static Grid,
    gap: Pos2,
    pos: Pos2,
}

impl Robot {
    fn new(keypad: &'static Grid) -> Self {
        Self {
            keypad,
            pos: keypad.positions('A').exactly_one().ok().unwrap(),
            gap: keypad.positions(' ').exactly_one().ok().unwrap(),
        }
    }

    fn commands_for(&mut self, target: char) -> impl Iterator<Item = char> {
        let new_pos = self.keypad.positions(target).exactly_one().ok().unwrap();
        let Dir2(move_row, move_col) = new_pos - self.pos;
        let vertical = if move_row > 0 {
            (move_row as usize, 'v')
        } else {
            (-move_row as usize, '^')
        };
        let horizontal = if move_col > 0 {
            (move_col as usize, '>')
        } else {
            (-move_col as usize, '<')
        };
        // If we need to "cross" the gap, make sure we need to go the safe way
        // first and also if we need to go right, then it is most efficient to
        // do that last
        let iter = if self.pos.0 == self.gap.0 && new_pos.1 == self.gap.1
            || (self.pos.1 != self.gap.1 || new_pos.0 != self.gap.0) && horizontal.1 == '>'
        {
            iter::repeat_n(vertical.1, vertical.0)
                .chain(iter::repeat_n(horizontal.1, horizontal.0))
                .chain(iter::once('A'))
        } else {
            iter::repeat_n(horizontal.1, horizontal.0)
                .chain(iter::repeat_n(vertical.1, vertical.0))
                .chain(iter::once('A'))
        };
        self.pos = new_pos;
        iter
    }
}

struct Cache<const N: usize> {
    results: [HashMap<String, usize>; N],
}

impl<const N: usize> Cache<N> {
    fn new() -> Self {
        Self {
            results: array::from_fn(|_| HashMap::new()),
        }
    }

    /// A new robot is initialised for every call, so we start at 'A' always.
    fn expansion_len(&mut self, level: usize, s: String) -> usize {
        if let Some(result) = self.results[level].get(&s) {
            return *result;
        }
        let mut robot = Robot::new(&DIR_KEYPAD);
        let result = if level == N - 1 {
            s.chars().flat_map(|c| robot.commands_for(c)).count()
        } else {
            s.chars()
                .map(|c| robot.commands_for(c).collect())
                .map(|s| self.expansion_len(level + 1, s))
                .sum()
        };
        self.results[level].insert(s.to_string(), result);
        result
    }
}

const part1: Solution = go::<2>;
const part2: Solution = go::<25>;

fn go<const N: usize>(input: SS) -> usize {
    let mut cache: Cache<N> = Cache::new();
    input
        .lines()
        .map(|code| -> usize {
            let mut first_robot = Robot::new(&NUM_KEYPAD);
            let count: usize = code
                .chars()
                .map(|c| first_robot.commands_for(c).collect())
                .map(|s| cache.expansion_len(0, s))
                .sum();
            count * to_usize(&code[..3])
        })
        .sum()
}

boilerplate! {
    part1 => { test -> 126384, real -> 128962 }
    part2 => { real -> 159684145150108 }
}
