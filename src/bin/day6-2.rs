extern crate failure;

use std::io;
use std::io::prelude::*;

use failure::Error;

fn main() -> Result<(), Error> {
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let coords: Vec<(i32, i32)> = input
        .lines()
        .map(|s| {
            let mut x = s.split(", ");
            Ok((x.next().unwrap().parse()?, x.next().unwrap().parse()?))
        }).collect::<Result<_, Error>>()?;

    let &min_x = coords.iter().map(|(x, _)| x).min().unwrap();
    let &max_x = coords.iter().map(|(x, _)| x).max().unwrap();
    let &min_y = coords.iter().map(|(_, y)| y).min().unwrap();
    let &max_y = coords.iter().map(|(_, y)| y).max().unwrap();

    let region_size = (min_x..=max_x)
        .flat_map(|x| (min_y..=max_y).map(move |y| (x, y)))
        .map(|(x, y)| {
            coords
                .iter()
                .map(|(cx, cy)| (x - cx).abs() + (y - cy).abs())
                .sum::<i32>()
        }).filter(|&d| d < 10000)
        .count();

    println!(
        "[part2] Region size where the distance to all locations is < 10000 is {}",
        region_size
    );

    Ok(())
}
