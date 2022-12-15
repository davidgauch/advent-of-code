use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let file = File::open("aoc04/input.txt").unwrap();
    let lines = BufReader::new(file).lines().map(Result::unwrap);

    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut fully_contained = 0u32;
    let mut partially_contained = 0u32;
    for line in lines {
        let caps = regex.captures(&line).unwrap();

        let bounds = [1, 2, 3, 4].map(|i| u8::from_str(&caps[i]).unwrap());
        let range1 = bounds[0]..=bounds[1];
        let range2 = bounds[2]..=bounds[3];

        match (
            range1.contains(range2.start()),
            range1.contains(range2.end()),
            range2.contains(range1.start()),
            range2.contains(range1.end()),
        ) {
            (true, true, _, _) | (_, _, true, true) => {
                fully_contained += 1;
                partially_contained += 1;
            }
            (true, _, _, _) | (_, true, _, _) | (_, _, true, _) | (_, _, _, true) => {
                partially_contained += 1;
            }
            _ => {}
        }
    }
    println!("fully contained: {fully_contained}");
    println!("partially contained: {partially_contained}");
}
