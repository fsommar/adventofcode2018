extern crate failure;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use failure::Error;

type SleepTimes<'a> = HashMap<(u32, &'a str), [bool; 60]>;

fn main() -> Result<(), Error> {
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let mut lines: Vec<_> = input.lines().collect();

    let sleeptimes = calc_sleep(&mut lines[..])?;

    print_sleeptimes(&sleeptimes);

    let m = calc_sleep_per_minute_per_guard(&sleeptimes);

    // Part1
    let (total_minutes, &sleepiest_guard) = m
        .iter()
        .map(|(guard, times)| (times.iter().sum::<u32>(), guard))
        .max()
        .unwrap();
    let (sleepiest_minute, _) = m[&sleepiest_guard]
        .iter()
        .enumerate()
        .max_by_key(|&(_, item)| item)
        .unwrap();
    println!(
        "[part1] #{} for {} minutes, with max on minute {}; {}",
        sleepiest_guard,
        total_minutes,
        sleepiest_minute,
        sleepiest_guard * sleepiest_minute as u32
    );

    // Part2
    let (&guard, minute) = m
        .iter()
        .map(|(guard, times)| {
            (
                guard,
                times
                    .iter()
                    .enumerate()
                    .max_by_key(|&(_, item)| item)
                    .unwrap(),
            )
        }).max_by_key(|&(_, (_minute, amount))| amount)
        .map(|(guard, (minute, _))| (guard, minute))
        .unwrap();

    println!(
        "[part2] #{} on minute {}; {}",
        guard,
        minute,
        guard * minute as u32
    );

    Ok(())
}

fn calc_sleep<'a>(lines: &mut [&'a str]) -> Result<SleepTimes<'a>, Error> {
    // Since input is on ISO-8601 date format,
    // sorting will make the events appear in chronological order.
    lines.sort();

    let mut sleeptimes = HashMap::new();

    let mut current_guard = None;
    let mut last_time = None;

    // Assumptions:
    //   * A guard cannot wake up twice before falling asleep, and vice versa
    //   * A guard always wakes up before they end their shift
    //   * If a guard starts their shift before midnight, those minutes do not count.
    for line in lines {
        println!("{}", line);
        if !line.contains("Guard") && current_guard == None {
            // For some reason, the first lines do not have a guard
            continue;
        }
        // Example data
        //   [1518-09-21 23:54] Guard #1553 begins shift
        //   [1518-05-05 00:48] falls asleep
        //   [1518-08-21 00:54] wakes up
        //         |     |  |    |     |
        //         0     1  2    3     4
        let split = line
            .split(&[' ', ':', '#', '[', ']'][..])
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        let current_minute = if split[1] == "23" {
            // Any time spent before midnight does not count
            0
        } else {
            split[2].parse::<u32>()?
        };

        if split[3] == "Guard" {
            current_guard = Some(split[4].parse::<u32>()?);
        } else if split[3] == "wakes" {
            let mut v = sleeptimes
                .entry((current_guard.unwrap(), split[0]))
                .or_insert([false; 60]);
            for i in last_time.unwrap()..current_minute {
                v[i as usize] = true;
            }
        }

        last_time = Some(current_minute);
    }

    Ok(sleeptimes)
}

fn calc_sleep_per_minute_per_guard(sleeptimes: &SleepTimes) -> HashMap<u32, [u32; 60]> {
    sleeptimes
        .iter()
        .fold(HashMap::new(), |mut m, ((guard, _), times)| {
            for (mut x, &y) in m
                .entry(*guard)
                .or_insert([0; 60])
                .iter_mut()
                .zip(times.iter())
            {
                *x += y as u32;
            }
            m
        })
}

fn print_sleeptimes(sleeptimes: &SleepTimes) {
    let mut x: Vec<_> = sleeptimes
        .iter()
        .map(|(key, value)| {
            (
                key,
                value
                    .iter()
                    .map(|&b| if b { '#' } else { '.' })
                    .collect::<String>(),
            )
        }).collect();
    x.sort();

    for ((guard, date), times) in x.iter() {
        println!("{} #{:4}  {}", &date[5..], guard, times);
    }
}
