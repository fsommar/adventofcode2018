extern crate failure;

use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;

use failure::Error;

fn main() -> Result<(), Error> {
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let (x, y) = input.lines().fold((0, 0), |(x, y), line| {
        let counts: HashSet<_> = count_chars(line).values().map(|&x| x).collect();
        (
            x + counts.contains(&2) as i32,
            y + counts.contains(&3) as i32,
        )
    });
    println!("[part1] Checksum is {}", x * y);
    Ok(())
}

fn count_chars(s: &str) -> HashMap<char, i32> {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort();
    chars.iter().fold(HashMap::new(), |mut m, &x| {
        *m.entry(x).or_insert(0) += 1;
        m
    })
}
