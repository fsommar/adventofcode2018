extern crate failure;

use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use failure::Error;

fn main() -> Result<(), Error> {
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    // Idea: Store current ID in all the cells it encompasses, and add it to the ID set.
    // If an ID already exists in that cell, both IDs are overlapping; remove them from the ID set.
    let mut table = vec![vec![0; 1000]; 1000];
    let mut set = HashSet::new();

    for line in input.lines() {
        // Example entry
        //   #31 @ 666,828: 11x21
        //    |     |   |   |   |
        //    0     1   2   3   4
        let data = line
            .split(&['#', ' ', ':', '@', ',', 'x'][..])
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        if let [id, left_edge, top_edge, width, height] = data[..] {
            let id: usize = id.parse()?;
            let left_edge: usize = left_edge.parse()?;
            let top_edge: usize = top_edge.parse()?;
            let width: usize = width.parse()?;
            let height: usize = height.parse()?;

            let mut has_overlap = false;

            for i in left_edge..(left_edge + width) {
                for j in top_edge..(top_edge + height) {
                    if table[i][j] != 0 {
                        // We have an overlap, remove IDs.
                        set.remove(&table[i][j]);
                        set.remove(&id);
                        has_overlap = true;
                    } else {
                        table[i][j] = id;
                        if !has_overlap {
                            set.insert(id);
                        }
                    }
                }
            }
        } else {
            panic!("Invalid input");
        }
    }

    if set.len() != 1 {
        panic!(format!(
            "Expected 1 element left in the set, was {}",
            set.len()
        ));
    }
    println!(
        "[part2] The overlapping ID is {}",
        set.iter().next().unwrap()
    );
    Ok(())
}
