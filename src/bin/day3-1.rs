extern crate failure;

use std::io;
use std::io::prelude::*;

use failure::Error;

fn main() -> Result<(), Error> {
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let mut table = vec![vec![0; 1000]; 1000];

    for line in input.lines() {
        // Example entry
        //   #31 @ 666,828: 11x21
        //    |     |   |   |   |
        //    0     1   2   3   4
        let data = line
            .split(&[' ', ':', '@', ',', 'x'][..])
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        if let [_, left_edge, top_edge, width, height] = data[..] {
            let left_edge: usize = left_edge.parse()?;
            let top_edge: usize = top_edge.parse()?;
            let width: usize = width.parse()?;
            let height: usize = height.parse()?;

            for i in left_edge..(left_edge + width) {
                for j in top_edge..(top_edge + height) {
                    table[i][j] += 1;
                }
            }
        } else {
            panic!("Invalid input");
        }
    }

    let mut overlaps = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if table[i][j] >= 2 {
                overlaps += 1;
            }
        }
    }
    println!("[part1] There are {} square inches of overlaps", overlaps);
    Ok(())
}
