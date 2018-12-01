extern crate failure;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env;
use std::io;
use std::io::prelude::*;

use failure::Error;

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;

    if env::args().skip(1).next().unwrap_or("part1".into()) == "part1" {
        let counter = lines
            .iter()
            .fold(0, |c, line| apply_change(c, line).unwrap());
        println!("[part1] Counter is {}", counter);
        Ok(())
    } else {
        let first_dup = find_first_dup(&lines)?;
        println!("[part2] First duplicate counter value is {}", first_dup);
        Ok(())
    }
}

fn apply_change<'a>(counter: i64, line: &'a str) -> Result<i64, Error> {
    let num: i64 = line.chars().skip(1).collect::<String>().parse()?;
    Ok(match line.as_bytes()[0] {
        b'+' => counter + num,
        b'-' => counter - num,
        _ => panic!("Invalid input format, expected line to start with + or -"),
    })
}

fn find_first_dup(lines: &[String]) -> Result<i64, Error> {
    let mut counter = 0i64;
    let mut map = HashMap::new();

    loop {
        for line in lines {
            counter = apply_change(counter, line)?;
            match map.entry(counter) {
                Entry::Occupied(_) => {
                    return Ok(counter);
                }
                Entry::Vacant(e) => {
                    e.insert(1);
                }
            }
        }
    }
}
