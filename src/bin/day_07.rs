use std::io::prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Result};

#[derive(Debug, Clone)]
enum Operation {
    Set(String),
    And(String, String),
    Or(String, String),
    LShift(String, String),
    RShift(String, String),
    Not(String)
}

fn execute(cache: &mut HashMap<String, u16>, graph: &HashMap<String, Operation>, target: &String) -> u16 {
    let op = graph.get(target).unwrap();

    match op {
        Operation::Set(value) => {
            if let Ok(v) = value.parse::<u16>() {
                cache.insert(target.clone(), v);
                return v;
            }

            let out = execute(cache, graph, value);
            cache.insert(target.clone(), out);
            out
        },
        Operation::And(left, right) => {
            let l = {
                if cache.contains_key(left) {
                    *cache.get(left).unwrap()
                } else {
                    if let Ok(l) = left.parse::<u16>() {
                        cache.insert(left.clone(), l);
                        l
                    } else {
                        let out = execute(cache, graph, left);
                        cache.insert(left.clone(), out);
                        out
                    }
                }
            };

            let r = {
                if cache.contains_key(right) {
                    *cache.get(right).unwrap()
                } else {
                    if let Ok(l) = right.parse::<u16>() {
                        cache.insert(right.clone(), l);
                        l
                    } else {
                        let out = execute(cache, graph, right);
                        cache.insert(right.clone(), out);
                        out
                    }
                }
            };

            l & r
        },
        Operation::Or(left, right) => {
            let l = {
                if cache.contains_key(left) {
                    *cache.get(left).unwrap()
                } else {
                    if let Ok(l) = left.parse::<u16>() {
                        cache.insert(left.clone(), l);
                        l
                    } else {
                        let out = execute(cache, graph, left);
                        cache.insert(left.clone(), out);
                        out
                    }
                }
            };

            let r = {
                if cache.contains_key(right) {
                    *cache.get(right).unwrap()
                } else {
                    if let Ok(l) = right.parse::<u16>() {
                        cache.insert(right.clone(), l);
                        l
                    } else {
                        let out = execute(cache, graph, right);
                        cache.insert(right.clone(), out);
                        out
                    }
                }
            };

            l | r
        },
        Operation::LShift(left, right) => {
            let l = {
                if cache.contains_key(left) {
                    *cache.get(left).unwrap()
                } else {
                    if let Ok(l) = left.parse::<u16>() {
                        cache.insert(left.clone(), l);
                        l
                    } else {
                        let out = execute(cache, graph, left);
                        cache.insert(left.clone(), out);
                        out
                    }
                }
            };

            let r = {
                if cache.contains_key(right) {
                    *cache.get(right).unwrap()
                } else {
                    if let Ok(l) = right.parse::<u16>() {
                        cache.insert(right.clone(), l);
                        l
                    } else {
                        let out = execute(cache, graph, right);
                        cache.insert(right.clone(), out);
                        out
                    }
                }
            };

            l << r
        },
        Operation::RShift(left, right) => {
            let l = {
                if cache.contains_key(left) {
                    *cache.get(left).unwrap()
                } else {
                    if let Ok(l) = left.parse::<u16>() {
                        cache.insert(left.clone(), l);
                        l
                    } else {
                        let out = execute(cache, graph, left);
                        cache.insert(left.clone(), out);
                        out
                    }
                }
            };

            let r = {
                if cache.contains_key(right) {
                    *cache.get(right).unwrap()
                } else {
                    if let Ok(l) = right.parse::<u16>() {
                        cache.insert(right.clone(), l);
                        l
                    } else {
                        let out = execute(cache, graph, right);
                        cache.insert(right.clone(), out);
                        out
                    }
                }
            };

            l >> r
        },
        Operation::Not(value) => {

            if let Ok(v) = value.parse::<u16>() {
                cache.insert(target.clone(), v);
                return v;
            }

            let out = execute(cache, graph, value);
            cache.insert(target.clone(), !out);
            !out
        },
    }
}

pub fn main() -> Result<()> {
    let f = BufReader::new(File::open("inputs/day_07.txt")?);

    // We create a cache here so that we don't need to recompute previously-
    // calculated values.
    let mut cache: HashMap<String, u16> = HashMap::new();

    // This is effectively a DAG.
    let mut ops: HashMap<String, Operation> = HashMap::new();

    for l in f.lines() {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts[1] == "AND" {
            let left = String::from(parts[0]);
            let right = String::from(parts[2]);
            let output = String::from(parts[4]);

            ops.insert(output, Operation::And(left, right));
        } else if parts[1] == "OR" {
            let left = String::from(parts[0]);
            let right = String::from(parts[2]);
            let output = String::from(parts[4]);

            ops.insert(output, Operation::Or(left, right));
        } else if parts[1] == "LSHIFT" {
            let left = String::from(parts[0]);
            let right = String::from(parts[2]);
            let output = String::from(parts[4]);

            ops.insert(output, Operation::LShift(left, right));
        } else if parts[1] == "RSHIFT" {
            let left = String::from(parts[0]);
            let right = String::from(parts[2]);
            let output = String::from(parts[4]);

            ops.insert(output, Operation::RShift(left, right));
        } else if parts[0] == "NOT" {
            let input = String::from(parts[1]);
            let output = String::from(parts[3]);

            ops.insert(output, Operation::Not(input));
        } else if parts.len() == 3 {
            let input = String::from(parts[0]);
            let output = String::from(parts[2]);

            ops.insert(output, Operation::Set(input));
        }
    }

    println!("{:?}", ops);
    println!("{:?}", ops.get("a"));

    // Start the recursion!
    let part1 = execute(&mut cache, &ops, &String::from("a"));

    ops.entry(String::from("b")).and_modify(|v| *v = Operation::Set(format!("{}", part1)));
    cache.clear();

    let part2 = execute(&mut cache, &ops, &String::from("a"));

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);

    Ok(())
}
