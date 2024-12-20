use common::*;

#[derive(Debug)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

fn part1(input: SS) -> usize {
    parse(input).filter_map(eval_machine).sum()
}

fn part2(input: SS) -> usize {
    const OFFSET: usize = 10_000_000_000_000;

    parse(input)
        .update(|m| {
            m.prize.0 += OFFSET;
            m.prize.1 += OFFSET;
        })
        .filter_map(eval_machine)
        .sum()
}

fn parse(input: SS) -> impl Iterator<Item = Machine> {
    input
        .split("\n\n")
        .map(|s| {
            let mut lines = s.lines();
            let (a0, a1) = lines
                .next()?
                .strip_prefix("Button A: X+")?
                .split_once(", Y+")?;
            let (b0, b1) = lines
                .next()?
                .strip_prefix("Button B: X+")?
                .split_once(", Y+")?;
            let (p0, p1) = lines
                .next()?
                .strip_prefix("Prize: X=")?
                .split_once(", Y=")?;
            Some(Machine {
                a: (to_usize(a0), to_usize(a1)),
                b: (to_usize(b0), to_usize(b1)),
                prize: (to_usize(p0), to_usize(p1)),
            })
        })
        .map(Option::unwrap)
}

// a_presses * a0 + b_presses * b0 = p0
// a_presses * a1 + b_presses * b1 = p1
// cost = 3 * a_presses + b_presses

// a_presses = (p0 - b_presses * b0) / a0
// (p0 - b_presses * b0) / a0 * a1 + b_presses * b1 = p1
// p1 = p0 / a0 * a1 - b_presses * b0 / a0 * a1 + b_presses * b1
// p1 = p0 / a0 * a1 - b_presses * (b0 / a0 * a1 - b1)
// p1 - p0 / a0 * a1 = -b_presses * (b0 / a0 * a1 - b1)
// b_presses = (p0 / a0 * a1 - p1) / (b0 / a0 * a1 - b1)

// ergo:
// a_presses = (p0 - b_presses * b0) / a0
// b_presses = (p0 / a0 * a1 - p1) / (b0 / a0 * a1 - b1)
// cost = 3 * a_presses + b_presses
fn eval_machine(Machine { a, b, prize: p }: Machine) -> Option<usize> {
    // approx then check
    let b_presses = ((p.0 as f64 / a.0 as f64 * a.1 as f64 - p.1 as f64)
        / (b.0 as f64 / a.0 as f64 * a.1 as f64 - b.1 as f64))
        .round() as isize;
    let a_presses = (p.0 as isize - b_presses * b.0 as isize) / a.0 as isize;
    // Now check that the number of presses are non-negative, would be a
    // difficult play otherwise. 🤷🏼‍♂️
    let a_presses: usize = a_presses.try_into().ok()?;
    let b_presses: usize = b_presses.try_into().ok()?;
    let correct =
        a_presses * a.0 + b_presses * b.0 == p.0 && a_presses * a.1 + b_presses * b.1 == p.1;
    correct.then(|| a_presses * 3 + b_presses)
}

boilerplate! {
    part1 => { test -> 480, real -> 33209 }
    part2 => { test -> 875318608908,  real -> 83102355665474 }
}
