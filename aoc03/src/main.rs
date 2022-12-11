use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("aoc03/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines().map(Result::unwrap);

    let mut total = 0;
    for line in lines {
        let (container1, container2) = line.split_at(line.len() / 2);

        let contents1: HashSet<_> = container1.chars().collect();
        let contents2: HashSet<_> = container2.chars().collect();

        let intersection = contents1.intersection(&contents2);

        if let Some(priority) = intersection
            .into_iter()
            .map(|character| priority(*character))
            .max()
        {
            total += u32::from(priority)
        }
    }

    println!("{total}");
}

fn priority(character: char) -> u8 {
    match character {
        'a'..='z' => u8::try_from(character).unwrap() - b'a' + 1,
        'A'..='Z' => u8::try_from(character).unwrap() - b'A' + 27,
        _ => panic!("illegal character"),
    }
}

fn part2() {
    let file = File::open("aoc03/input.txt").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let mut total = 0;
    for chunk in lines.chunks(3) {
        assert_eq!(chunk.len(), 3);

        let intersection = chunk
            .into_iter()
            .map(|sack| sack.chars().collect::<HashSet<_>>())
            .reduce(|a, b| a.intersection(&b).cloned().collect())
            .unwrap();

        if let Some(priority) = intersection
            .into_iter()
            .map(|character| priority(character))
            .max()
        {
            total += u32::from(priority)
        }
    }

    println!("{total}");
}
