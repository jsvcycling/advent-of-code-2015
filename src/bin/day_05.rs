use std::collections::HashMap;
use std::fs::read_to_string;

const VOWELS: &str = "aeiou";
const NAUGHTY_PAIRS: &str = "ab cd pq xy";

fn part1(buf: &String) -> usize {
    buf.lines()
        .map(|l| {
            let chars: Vec<char> = l.chars().collect();

            let mut vowel_count = 0;
            let mut has_double_letter = false;

            if VOWELS.contains(chars[0]) {
                vowel_count += 1;
            }

            for idx in 0..(chars.len()-1) {
                let c1 = chars[idx];
                let c2 = chars[idx + 1];

                if NAUGHTY_PAIRS.contains(format!("{}{}", c1, c2).as_str()) {
                    return false;
                }

                if VOWELS.contains(c2) {
                    vowel_count += 1;
                }

                if c1 == c2 {
                    has_double_letter = true;
                }
            }

            vowel_count >= 3 && has_double_letter
        })
        .filter(|v| *v == true)
        .count()
}

fn part2(buf: &String) -> usize {
    buf.lines()
        .map(|l| {
            let chars: Vec<char> = l.chars().collect();

            let mut pairs = HashMap::new();

            let mut passed1 = false;
            let mut passed2 = false;

            pairs.insert(format!("{}{}", chars[0], chars[1]), 1);

            // Iterate over every character in the string and test Criteria 2 as
            // well as build a hashmap of pairs. The number of times a pair
            // appears in the string is recorded in the hashmap value.
            for idx in 0..(chars.len() - 2) {
                let c1 = chars[idx];
                let c2 = chars[idx + 1];
                let c3 = chars[idx + 2];

                if c1 == c3 {
                    passed2 = true;
                }

                pairs
                    .entry(format!("{}{}", c2, c3))
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }

            let pairs: HashMap<&String, &usize> = pairs.iter().filter(|(_, v)| **v >= 2).collect();

            if pairs.is_empty() {
                return false;
            }

            // For each pair with 2 or more occurrences, make sure that the
            // first & last occurrance do not overlap (if there are 3
            // occurrences it's okay if the "middle" occurrence overlaps with
            // one of the outer ones; we only need 2 ocurrences).
            for k in pairs.keys() {
                let idx = l.find(k.as_str()).unwrap();
                let ridx = l.rfind(k.as_str()).unwrap();

                if ridx != idx + 1 {
                    passed1 = true;
                    break;
                }
            }

            passed1 && passed2
        })
        .filter(|v| *v == true)
        .count()
}

pub fn main() {
    let buf = read_to_string("inputs/day_05.txt").unwrap();

    println!("Part 1: {}", part1(&buf));
    println!("Part 2: {}", part2(&buf));
}
