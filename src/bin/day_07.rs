use std::collections::HashMap;
use std::fs::read_to_string;

type Cache = HashMap<String, u16>;
type Graph = HashMap<String, Op>;

enum Op {
    Set(String),
    And(String, String),
    Or(String, String),
    LShift(String, String),
    RShift(String, String),
    Not(String),
}

// Check if the target exists in the cache. If not, check if it's a pure value
// (i.e. 16-bit number). If neither of these are true, execute it.
fn get_value(cache: &mut Cache, graph: &Graph, target: &String) -> u16 {
    if cache.contains_key(target) {
        return *cache.get(target).unwrap();
    }

    if let Ok(l) = target.parse::<u16>() {
        cache.insert(target.clone(), l);
        return l;
    }

    let value = execute(cache, graph, target);
    cache.insert(target.clone(), value);
    value
}

fn execute(cache: &mut Cache, graph: &Graph, target: &String) -> u16 {
    match graph.get(target).unwrap() {
        Op::Set(v) => get_value(cache, graph, v),
        Op::And(l, r) => get_value(cache, graph, l) & get_value(cache, graph, r),
        Op::Or(l, r) => get_value(cache, graph, l) | get_value(cache, graph, r),
        Op::LShift(l, r) => get_value(cache, graph, l) << get_value(cache, graph, r),
        Op::RShift(l, r) => get_value(cache, graph, l) >> get_value(cache, graph, r),
        Op::Not(v) => !get_value(cache, graph, v),
    }
}

pub fn main() {
    let buf = read_to_string("inputs/day_07.txt").unwrap();

    // This is effectively a DAG.
    let mut ops: Graph = Graph::new();

    buf.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts[1] == "AND" {
            ops.insert(
                parts[4].to_string(),
                Op::And(parts[0].to_string(), parts[2].to_string()),
            );
        } else if parts[1] == "OR" {
            ops.insert(
                parts[4].to_string(),
                Op::Or(parts[0].to_string(), parts[2].to_string()),
            );
        } else if parts[1] == "LSHIFT" {
            ops.insert(
                parts[4].to_string(),
                Op::LShift(parts[0].to_string(), parts[2].to_string()),
            );
        } else if parts[1] == "RSHIFT" {
            ops.insert(
                parts[4].to_string(),
                Op::RShift(parts[0].to_string(), parts[2].to_string()),
            );
        } else if parts[0] == "NOT" {
            ops.insert(parts[3].to_string(), Op::Not(parts[1].to_string()));
        } else if parts.len() == 3 {
            ops.insert(parts[2].to_string(), Op::Set(parts[0].to_string()));
        }
    });

    // We create a cache here so that we don't need to recompute previously-
    // calculated values.
    let mut cache: Cache = Cache::new();

    // Start the recursion!
    let part1 = execute(&mut cache, &ops, &"a".to_string());

    ops.entry("b".to_string())
        .and_modify(|v| *v = Op::Set(part1.to_string()));

    cache.clear();

    let part2 = execute(&mut cache, &ops, &"a".to_string());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
