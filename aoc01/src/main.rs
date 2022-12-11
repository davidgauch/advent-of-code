use std::fs::File;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut top = [0; 3];
    let mut curr = 0;
    for line in lines {
        let line = line?;
        if line.is_empty() {
            if top[0] < curr {
                top[0] = curr;
                top.sort();
            }
            curr = 0;
            continue;
        }

        curr += line.parse::<u32>()?;
    }

    println!("top 3: {top:?}");
    println!("sum : {}", top.iter().sum::<u32>());

    Ok(())
}
