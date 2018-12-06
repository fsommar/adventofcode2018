extern crate failure;

use std::collections::{HashMap, HashSet};
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

    let mut areas = HashMap::new();
    let mut infinites: HashSet<usize> = HashSet::new();

    let &min_x = coords.iter().map(|(x, _)| x).min().unwrap();
    let &max_x = coords.iter().map(|(x, _)| x).max().unwrap();
    let &min_y = coords.iter().map(|(_, y)| y).min().unwrap();
    let &max_y = coords.iter().map(|(_, y)| y).max().unwrap();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let dists: Vec<_> = coords
                .iter()
                .map(|(cx, cy)| (x - cx).abs() + (y - cy).abs())
                .enumerate()
                .collect();
            let &(closest, min_dist) = dists.iter().min_by_key(|&(_, dist)| dist).unwrap();
            // Count the area iff it's the only one closest to this point.
            if dists.iter().filter(|(_, d)| *d == min_dist).count() == 1 {
                *areas.entry(closest).or_insert(0) += 1;
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    infinites.insert(closest);
                }
            }
        }
    }

    println!(
        "[part1] Maximum non-infinite area is {}",
        areas
            .iter()
            .filter(|(i, _)| !infinites.contains(i))
            .map(|(_, area)| area)
            .max()
            .unwrap()
    );

    Ok(())
}
