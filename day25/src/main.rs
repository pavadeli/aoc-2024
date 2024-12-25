use common::*;

fn part1(input: SS) -> usize {
    let (keys, locks): (Vec<_>, Vec<_>) = input
        .lines()
        .chunks(8)
        .into_iter()
        .partition_map(parse_pattern);
    locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|(lock, key)| lock.iter().zip_eq(key).all(|(a, b)| a + b < 6))
        .count()
}

fn parse_pattern(mut c: impl Iterator<Item = SS>) -> Either<[u8; 5], [u8; 5]> {
    assert!(matches!(c.next(), Some("#####" | ".....")));
    let mut r = [0; 5];
    for line in c.by_ref().take(5) {
        r.iter_mut()
            .zip_eq(line.chars())
            .filter(|(_, c)| *c == '#')
            .for_each(|(n, _)| *n += 1);
    }
    let result = match c.next().unwrap() {
        "#####" => Either::Left(r),
        "....." => Either::Right(r),
        c => panic!("unexpected last line: {c:?}"),
    };
    assert!(c.all(|line| line.is_empty()));
    result
}

boilerplate! {
    part1 => { test -> 3, real -> 3320 }
}
