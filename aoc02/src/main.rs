use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> anyhow::Result<()> {
    part1()?;
    part2()
}

fn part1() -> anyhow::Result<()> {
    let file = File::open("aoc02/input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut my_total: u32 = 0;
    for line in lines {
        let line = line?;
        let bytes = line.as_bytes();

        if bytes.len() != 3 {
            continue;
        }

        assert!(b"ABC".contains(&bytes[0]));
        assert!(b"XYZ".contains(&bytes[2]));

        let op_throw = bytes[0] - b'A';
        let my_throw = bytes[2] - b'X';

        let mut my_points = my_throw + 1;

        match ((op_throw as i8) - (my_throw as i8)).rem_euclid(3) {
            0 => {
                // draw
                my_points += 3;
            }
            1 => {}
            2 => {
                // my win
                my_points += 6;
            }
            _ => unreachable!(),
        }

        my_total += my_points as u32;
    }

    println!("{my_total}");

    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let file = File::open("aoc02/input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut my_total: u32 = 0;
    for line in lines {
        my_total += match line?.as_str() {
            // Trailing whitespace
            "" => 0,
            // Opponent Rock
            "A X" => 0 + 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,

            // Opponent Paper
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,

            // Opponent Scissors
            "C X" => 0 + 2,
            "C Y" => 3 + 3,
            "C Z" => 6 + 1,
            _ => unreachable!(),
        };
    }

    println!("{my_total}");

    Ok(())
}
