use std::collections::HashMap;
use std::fs::read_to_string;

type Coord = (i32, i32);

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

fn parse_action(input: &str) -> Action {
    let parts: Vec<&str> = input.split_whitespace().collect();

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

pub fn part1(buf: &String) -> usize {
    // Although we're working on a "grid", store the light states in a HashMap
    // so that they can be handled sparsely (i.e. we don't care about lights
    // that we never interact with).
    let mut lights = HashMap::new();

    buf.lines().for_each(|line| {
        let action = parse_action(line);

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
        };
    });

    lights.values().filter(|v| **v == true).count()
}

pub fn part2(buf: &String) -> i32 {
    // Although we're working on a "grid", store the light states in a HashMap
    // so that they can be handled sparsely (i.e. we don't care about lights
    // that we never interact with).
    let mut lights = HashMap::new();

    buf.lines().for_each(|line| {
        let action = parse_action(line);

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
        };
    });

    lights.values().sum()
}

pub fn main() {
    let buf = read_to_string("inputs/day_06.txt").unwrap();

    println!("Part 1: {}", part1(&buf));
    println!("Part 2: {}", part2(&buf));
}
