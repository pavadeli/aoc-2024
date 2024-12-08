use common::{DIRECTIONS_8, Grid, NE, NW, SE, SS, SW, boilerplate, second};

fn part1(input: SS) -> usize {
    let grid: &Grid = &input.into();
    grid.positions('X')
        .flat_map(|from| {
            DIRECTIONS_8.iter().filter(move |&&dir| {
                grid.walk(from, dir)
                    .take(4)
                    .map(second)
                    .eq(['X', 'M', 'A', 'S'])
            })
        })
        .count()
}

fn part2(input: SS) -> usize {
    let grid: Grid = input.into();
    grid.positions('A')
        .filter(|&from| {
            matches!((
                grid.get(from + NE),
                grid.get(from + NW),
                grid.get(from + SW),
                grid.get(from + SE)
            ), (
                Some(ne @ ('M'|'S')),
                Some(nw @ ('M'|'S')),
                Some(sw @ ('M'|'S')),
                Some(se @ ('M'|'S'))
            ) if ne != sw && nw != se)
        })
        .count()
}

boilerplate! {
    part1 => { test -> 18, real -> 2468 }
    part2 => { test -> 9, real -> 1864 }
}
