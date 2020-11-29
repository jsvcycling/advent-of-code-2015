use std::io::prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Result, SeekFrom};

type Coord = (i32, i32);

#[derive(Debug)]
enum Action {
    TurnOn { start: Coord, end: Coord },
    TurnOff { start: Coord, end: Coord },
    Toggle { start: Coord, end: Coord },
    None,
}

fn parse_coord(coord: &str) -> Coord {
    let vals: Vec<i32> = coord
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    (vals[0], vals[1])
}

fn parse_action(input: String) -> Action {
    let parts: Vec<&str> = input.split(' ').collect();

    if parts[1] == "on" {
        return Action::TurnOn {
            start: parse_coord(parts[2]),
            end: parse_coord(parts[4]),
        };
    } else if parts[1] == "off" {
        return Action::TurnOff {
            start: parse_coord(parts[2]),
            end: parse_coord(parts[4]),
        };
    } else if parts[0] == "toggle" {
        return Action::Toggle {
            start: parse_coord(parts[1]),
            end: parse_coord(parts[3]),
        };
    }

    Action::None
}

pub fn part1(reader: &mut BufReader<File>) -> Result<usize> {
    // Although we're working on a "grid", store the light states in a HashMap
    // so that they can be handled sparsely (i.e. we don't care about lights
    // that we never interact with).
    let mut lights = HashMap::new();

    loop {
        let mut buffer = String::new();
        let result = reader.read_line(&mut buffer);

        if result.is_err() || result.unwrap() == 0 {
            break;
        }

        let action = parse_action(buffer);

        match action {
            Action::TurnOn { start, end } => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        lights
                            .entry((i, j))
                            .and_modify(|l| *l = true)
                            .or_insert(true);
                    }
                }
            }
            Action::TurnOff { start, end } => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        lights
                            .entry((i, j))
                            .and_modify(|l| *l = false)
                            .or_insert(false);
                    }
                }
            }
            Action::Toggle { start, end } => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        lights
                            .entry((i, j))
                            .and_modify(|l| *l = !*l)
                            .or_insert(true);
                    }
                }
            }
            _ => (),
        }
    }

    let turned_on = lights.values().filter(|v| **v == true).count();

    Ok(turned_on)
}

pub fn part2(reader: &mut BufReader<File>) -> Result<i32> {
    // Although we're working on a "grid", store the light states in a HashMap
    // so that they can be handled sparsely (i.e. we don't care about lights
    // that we never interact with).
    let mut lights = HashMap::new();

    loop {
        let mut buffer = String::new();
        let result = reader.read_line(&mut buffer);

        if result.is_err() || result.unwrap() == 0 {
            break;
        }

        let action = parse_action(buffer);

        match action {
            Action::TurnOn { start, end } => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        lights.entry((i, j)).and_modify(|l| *l += 1).or_insert(1);
                    }
                }
            }
            Action::TurnOff { start, end } => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        lights
                            .entry((i, j))
                            .and_modify(|l| *l = std::cmp::max(*l - 1, 0));
                    }
                }
            }
            Action::Toggle { start, end } => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        lights.entry((i, j)).and_modify(|l| *l += 2).or_insert(2);
                    }
                }
            }
            _ => (),
        }
    }

    let total_brightness = lights.values().sum();

    Ok(total_brightness)
}

pub fn main() -> Result<()> {
    let file = File::open("inputs/day_06.txt")?;
    let mut reader = BufReader::new(file);

    println!("Part 1: {}", part1(&mut reader)?);

    reader.seek(SeekFrom::Start(0))?;

    println!("Part 2: {}", part2(&mut reader)?);

    Ok(())
}
