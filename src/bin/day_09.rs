use std::collections::HashMap;
use std::fs::read_to_string;

fn part1(routes: &HashMap<String, HashMap<String, usize>>) -> Option<usize> {
    routes
        .keys()
        .scan(usize::MAX, |state, node| {
            let mut visited: Vec<_> = vec![node.to_string()];
            let mut total: usize = 0;

            // Keep looping until we've visited every node.
            while visited.len() != routes.len() {
                if let Some(targets) = routes.get(visited.last().unwrap()) {
                    // Select closest target node that hasn't been visited yet.
                    let next = targets
                        .iter()
                        .filter(|t| !visited.contains(t.0))
                        .min_by_key(|t| t.1)
                        .unwrap();

                    visited.push(next.0.clone());
                    total += next.1;
                }
            }

            *state = usize::min(total, *state);
            Some(*state)
        })
        .last()
}

fn part2(routes: &HashMap<String, HashMap<String, usize>>) -> Option<usize> {
    routes
        .keys()
        .scan(usize::MIN, |state, node| {
            let mut visited: Vec<_> = vec![node.to_string()];
            let mut total: usize = 0;

            // Keep looping until we've visited every node.
            while visited.len() != routes.len() {
                if let Some(targets) = routes.get(visited.last().unwrap()) {
                    // Select furthest target node that hasn't been visited yet.
                    let next = targets
                        .iter()
                        .filter(|t| !visited.contains(t.0))
                        .max_by_key(|t| t.1)
                        .unwrap();

                    visited.push(next.0.clone());
                    total += next.1;
                }
            }

            *state = usize::max(total, *state);
            Some(*state)
        })
        .last()
}

pub fn main() {
    let buf = read_to_string("inputs/day_09.txt").unwrap();
    let mut routes: HashMap<String, HashMap<String, usize>> = HashMap::new();

    buf.lines().for_each(|line| {
        let parts: Vec<_> = line.split_whitespace().collect();

        let loc1 = parts[0].to_string();
        let loc2 = parts[2].to_string();
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

    println!("Part 1: {}", part1(&routes).unwrap());
    println!("Part 2: {}", part2(&routes).unwrap());
}
