use std::io::prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Result};

fn part1(routes: &HashMap<String, HashMap<String, usize>>) -> (usize, Vec<String>) {
    let mut order: Vec<String> = Vec::new();
    let mut minimum: usize = usize::MAX;

    for start in routes.keys() {
        let mut visited: Vec<String> = vec![start.to_string()];
        let mut total: usize = 0;
        let mut current: String = start.clone();

        // Keep looping until we've visited every node.
        loop {
            if visited.len() == routes.keys().len() {
                break;
            }

            if let Some(targets) = routes.get(&current) {
                let mut next: (String, usize) = (String::new(), usize::MAX);

                // Loop through the list of edge for this specific node to find
                // the shortest.
                for t in targets {
                    if !visited.contains(t.0) && *t.1 < next.1 {
                        next = (t.0.clone(), *t.1);
                    }
                }

                visited.push(next.0.clone());
                total += next.1;
                current = next.0.clone();
            }
        }

        // If this iteration is shorter than the current shortest, save it.
        if total < minimum {
            order = visited.clone();
            minimum = total;
        }
    }

    (minimum, order)
}

fn part2(routes: &HashMap<String, HashMap<String, usize>>) -> (usize, Vec<String>) {
    let mut order: Vec<String> = Vec::new();
    let mut maximum: usize = usize::MIN;

    for start in routes.keys() {
        let mut visited: Vec<String> = vec![start.to_string()];
        let mut total: usize = 0;
        let mut current: String = start.clone();

        // Keep looping until we've visited every node.
        loop {
            if visited.len() == routes.keys().len() {
                break;
            }

            if let Some(targets) = routes.get(&current) {
                let mut next: (String, usize) = (String::new(), usize::MIN);

                // Loop through the list of edge for this specific node to find
                // the shortest.
                for t in targets {
                    if !visited.contains(t.0) && *t.1 > next.1 {
                        next = (t.0.clone(), *t.1);
                    }
                }

                visited.push(next.0.clone());
                total += next.1;
                current = next.0.clone();
            }
        }

        // If this iteration is longer than the current longest, save it.
        if total > maximum {
            order = visited.clone();
            maximum = total;
        }
    }

    (maximum, order)
}

pub fn main() -> Result<()> {
    let f = BufReader::new(File::open("inputs/day_09.txt")?);

    let mut routes: HashMap<String, HashMap<String, usize>> = HashMap::new();

    f.lines().for_each(|l| {
        let line = l.unwrap();
        let parts: Vec<_> = line.split_whitespace().collect();

        let loc1 = String::from(parts[0]);
        let loc2 = String::from(parts[2]);
        let dist = parts[4].parse::<usize>().unwrap();

        // Insert 2 entries because the graph is not directed.
        routes
            .entry(loc1.clone())
            .and_modify(|v| {
                v.insert(loc2.clone(), dist);
            })
            .or_insert({
                let mut h: HashMap<String, usize> = HashMap::new();
                h.insert(loc2.clone(), dist);
                h
            });

        routes
            .entry(loc2.clone())
            .and_modify(|v| {
                v.insert(loc1.clone(), dist);
            })
            .or_insert({
                let mut h: HashMap<String, usize> = HashMap::new();
                h.insert(loc1.clone(), dist);
                h
            });
    });

    let part1 = part1(&routes);
    let part2 = part2(&routes);

    println!("Part 1: {} ({:?})", part1.0, part1.1);
    println!("Part 2: {} ({:?})", part2.0, part2.1);

    Ok(())
}
