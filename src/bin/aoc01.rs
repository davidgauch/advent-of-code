use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

fn main() -> Result<()> {
    let (a, b): (Vec<_>, Vec<_>) = BufReader::new(File::open("data/aoc01.data")?)
        .lines()
        .filter_map(|l| {
            let l = l.ok()?;
            let (a, b) = l.split_once(" ")?;

            Some((a.parse::<usize>().ok()?, b.parse::<usize>().ok()?))
        })
        .collect();

    //let sum: u32 = a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum();
    //println!("{sum}");

    let counts = b.into_iter().counts();
    let sum: usize = a
        .into_iter()
        .filter_map(|a| Some(a * counts.get(&a)?))
        .sum();

    println!("{sum}");
    Ok(())
}
