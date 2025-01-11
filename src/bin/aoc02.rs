use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    ops::RangeInclusive,
};

use itertools::Itertools;

fn main() -> Result<()> {
    let mut count = 0;
    for line in BufReader::new(File::open("data/aoc02.data")?).lines() {
        let line = line?;

        let mut pairs = line
            .split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .tuple_windows();

        let Some((a, b)) = pairs.next() else {
            continue;
        };

        const INCREASING: RangeInclusive<i32> = -3..=-1;
        const DECREASING: RangeInclusive<i32> = 1..=3;

        let diff = a -b;
        let legal_diff = if INCREASING.contains(&diff) {
            INCREASING
        } else if DECREASING.contains(&diff) {
            DECREASING
        } else {
            continue;
        };

        if pairs.all(|(a, b)| legal_diff.contains(&(a - b))) {
            count += 1;
        }
    }

    println!("{count}");

    Ok(())
}
