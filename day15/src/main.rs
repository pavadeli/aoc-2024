use common::{CLEAR_TERM, Dir2, E, Grid, Itertools, N, Pos2, S, SS, W, boilerplate};
use std::{mem, thread::sleep, time::Duration};

fn part1(input: SS) -> usize {
    let (mut grid, moves) = parse(input, |c| [c]);
    let mut robot = grid.positions('@').exactly_one().ok().unwrap();
    grid[robot] = '.';
    for m in moves {
        let (next_pos, found_there) = grid.step(robot, m).unwrap();
        if found_there == '#' {
            // impossible move
            continue;
        }
        if found_there == 'O' {
            // see if we can move some blocks;
            let Some((move_to, _)) = grid
                .walk(robot, m)
                .skip(1)
                .take_while(|(_, c)| *c != '#')
                .find(|(_, c)| *c == '.')
            else {
                continue;
            };
            grid.swap(next_pos, move_to);
        }
        robot = next_pos;

        #[cfg(not(test))]
        dump_grid(&mut grid, robot);
    }

    grid.positions('O').map(|p| p.0 * 100 + p.1).sum()
}

fn part2(input: SS) -> usize {
    let (mut grid, moves) = parse(input, |c| match c {
        c @ ('#' | '.') => [c; 2],
        '@' => ['@', '.'],
        'O' => ['[', ']'],
        _ => panic!("unexpected character in grid: {c}"),
    });
    let mut robot = grid.positions('@').exactly_one().ok().unwrap();
    grid[robot] = '.';
    let mut sandbox = grid.clone();
    for dir in moves {
        let (next_pos, found_there) = grid.step(robot, dir).unwrap();
        match found_there {
            // impossible move
            '#' => (),
            // easy move
            '.' => robot = next_pos,
            // Ok, we need to move some boxes... Let's try so in our sandbox.
            '[' | ']' => {
                sandbox.clone_from(&grid);
                if move_box(&mut sandbox, next_pos, found_there, dir).is_ok() {
                    robot = next_pos;
                    mem::swap(&mut grid, &mut sandbox);
                }
            }
            _ => unreachable!(),
        }
    }

    #[cfg(not(test))]
    dump_grid(&mut grid, robot);

    grid.positions('[').map(|p| p.0 * 100 + p.1).sum()
}

fn parse<F, I>(input: SS, grid_mapper: F) -> (Grid, impl Iterator<Item = Dir2>)
where
    F: Fn(char) -> I,
    I: IntoIterator<Item = char>,
{
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let grid: Grid = grid
        .lines()
        .map(|line| line.chars().flat_map(&grid_mapper))
        .collect();
    let moves = moves.chars().filter_map(|c| match c {
        '>' => Some(E),
        '^' => Some(N),
        '<' => Some(W),
        'v' => Some(S),
        '\n' => None,
        _ => panic!("unexpected character: {c:?}"),
    });
    (grid, moves)
}

#[allow(dead_code, reason = "used to animate the story of the robot on run")]
fn dump_grid(grid: &mut Grid, robot: Pos2) {
    grid[robot] = '@';
    println!("{CLEAR_TERM}{grid:?}");
    sleep(Duration::from_millis(200));
    grid[robot] = '.';
}

fn move_box(grid: &mut Grid, pos: Pos2, ch: char, dir: Dir2) -> Result<(), ()> {
    let pos = if ch == '[' {
        pos
    } else {
        pos.saturating_add_dir(W)
    };
    match dir {
        E | W => {
            let check_pos;
            let new_left_pos;
            let new_right_pos;
            let clear_pos;
            if dir == E {
                new_left_pos = pos.saturating_add_dir(E);
                new_right_pos = new_left_pos.saturating_add_dir(E);
                check_pos = new_right_pos;
                clear_pos = pos;
            } else {
                new_left_pos = pos.saturating_add_dir(W);
                new_right_pos = pos;
                check_pos = new_left_pos;
                clear_pos = pos.saturating_add_dir(E);
            }
            match grid[check_pos] {
                c @ ('[' | ']') => move_box(grid, check_pos, c, dir)?,
                '#' => return Err(()),
                c => assert_eq!(c, '.'),
            }
            grid[new_left_pos] = '[';
            grid[new_right_pos] = ']';
            grid[clear_pos] = '.';
        }
        N | S => {
            let new_pos = pos.saturating_add_dir(dir);
            let other_pos = new_pos.saturating_add_dir(E);
            for p in [new_pos, other_pos] {
                match grid[p] {
                    c @ ('[' | ']') => move_box(grid, p, c, dir)?,
                    '#' => return Err(()),
                    c => assert_eq!(c, '.'),
                }
            }
            grid[new_pos] = '[';
            grid[other_pos] = ']';
            grid[pos] = '.';
            grid[pos.saturating_add_dir(E)] = '.';
        }
        _ => unreachable!(),
    }
    Ok(())
}

boilerplate! {
    part1 => { kid -> 1433 }
    part1 => { test1 -> 10092, test2 -> 2028, real -> 1413675 }
    part2 => { test1 -> 9021, real -> 1399772 }
}
