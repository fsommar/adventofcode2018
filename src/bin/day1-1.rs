extern crate failure;

use std::io;
use std::io::prelude::*;

use failure::Error;

fn main() -> Result<(), Error> {
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let counter = input
        .lines()
        .fold(0, |c, line| c + line.parse::<i64>().unwrap());
    println!("[part1] Counter is {}", counter);
    Ok(())
}
