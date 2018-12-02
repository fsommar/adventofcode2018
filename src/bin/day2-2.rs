extern crate failure;

use std::io;
use std::io::prelude::*;

use failure::Error;

fn main() -> Result<(), Error> {
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let lines: Vec<_> = input.lines().collect();

    for (i, line1) in lines.iter().enumerate() {
        for line2 in lines[i..].iter() {
            if distance(line1, line2) == 1 {
                let similar = line1
                    .chars()
                    .zip(line2.chars())
                    .filter(|(c1, c2)| c1 == c2)
                    .map(|(c, _)| c)
                    .collect::<String>();

                println!(
                    "[part2] The similar characters for the two boxes are {}",
                    similar
                );
                return Ok(());
            }
        }
    }
    Ok(())
}

fn distance(s1: &str, s2: &str) -> i32 {
    s1.chars()
        .zip(s2.chars())
        .fold(0, |acc, (c1, c2)| acc + (c1 != c2) as i32)
}
