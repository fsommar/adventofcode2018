extern crate failure;

use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use failure::Error;

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;

    let first_dup = find_first_dup(&lines)?;
    println!("[part2] First duplicate counter value is {}", first_dup);
    Ok(())
}

fn find_first_dup(lines: &[String]) -> Result<i64, Error> {
    let mut counter = 0i64;
    let mut set = HashSet::new();

    loop {
        for line in lines {
            counter += line.parse::<i64>()?;
            if set.contains(&counter) {
                return Ok(counter);
            }
            set.insert(counter);
        }
    }
}
