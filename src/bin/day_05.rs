#[macro_use]
extern crate lazy_static;

use std::io::prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Result, SeekFrom};

lazy_static! {
    static ref VOWELS: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    static ref NAUGHTY_PAIRS: Vec<&'static str> = vec!["ab", "cd", "pq", "xy"];
}

fn check_vowel(c: &char) -> bool {
    VOWELS.contains(c)
}

fn part1(reader: &mut BufReader<File>) -> Result<usize> {
    let mut nice_count = 0;

    loop {
        let mut buffer = String::new();
        let result = reader.read_line(&mut buffer);

        if result.is_err() || result.unwrap() == 0 {
            break;
        }

        let mut vowel_count = 0;
        let mut has_double_letter = false;
        let mut has_naughty_pair = false;

        buffer = buffer.trim().to_string();
        let mut chars = buffer.chars();

        // We need to remember the previous character.
        let mut prev_char = chars.next().unwrap();

        if check_vowel(&prev_char) {
            vowel_count += 1;
        }

        for c in chars {
            if check_vowel(&c) {
                vowel_count += 1;
            }

            if c == prev_char {
                has_double_letter = true;
            }

            let char_pair = format!("{}{}", prev_char, c);

            if NAUGHTY_PAIRS.contains(&char_pair.as_str()) {
                has_naughty_pair = true;
                break;
            }

            prev_char = c;
        }

        if vowel_count >= 3 && has_double_letter && !has_naughty_pair {
            nice_count += 1;
        }
    }

    Ok(nice_count)
}

fn part2(reader: &mut BufReader<File>) -> Result<usize> {
    let mut nice_count = 0;

    loop {
        let mut buffer = String::new();
        let result = reader.read_line(&mut buffer);

        if result.is_err() || result.unwrap() == 0 {
            break;
        }

        buffer = buffer.trim().to_string();

        // We need to remember the two previous characters.
        let mut chars = buffer.chars();
        let mut prev_char1 = chars.next().unwrap();
        let mut prev_char2 = chars.next().unwrap();

        let mut pairs = HashMap::new();
        pairs.insert(format!("{}{}", prev_char1, prev_char2), 1);

        let mut passed1 = false;
        let mut passed2 = false;

        // Iterate over every character in the string and test Criteria 2 as
        // well as build a hashmap of pairs. The number of times a pair
        // appears in the string is recorded in the hashmap value.
        for c in chars {
            if prev_char1 == c {
                passed2 = true;
            }

            let pair = format!("{}{}", prev_char2, c);

            if let Some(v) = pairs.get_mut(&pair) {
                *v += 1;
            } else {
                pairs.insert(pair, 1);
            }

            // Prepare to shift over 1 character.
            prev_char1 = prev_char2;
            prev_char2 = c;
        }

        let pairs: HashMap<&String, &usize> = pairs.iter().filter(|(_, v)| *v >= &2).collect();

        if pairs.is_empty() {
            continue;
        }

        // For each pair with 2 or more occurrences, make sure that the first &
        // last occurrance do not overlap (if there are 3 occurrences it's okay
        // if the "middle" occurrence overlaps with one of the outer ones; we
        // only need 2 ocurrences).
        for (k, _) in pairs.iter() {
            let find_idx = buffer.find(k.as_str()).unwrap();
            let rfind_idx = buffer.rfind(k.as_str()).unwrap();

            if rfind_idx != find_idx + 1 {
                passed1 = true;
                break;
            }
        }

        if passed1 && passed2 {
            nice_count += 1;
        }
    }

    Ok(nice_count)
}

pub fn main() -> Result<()> {
    let file = File::open("inputs/day_05.txt")?;
    let mut reader = BufReader::new(file);

    println!("Part 1: {}", part1(&mut reader)?);

    reader.seek(SeekFrom::Start(0))?;

    println!("Part 2: {}", part2(&mut reader)?);

    Ok(())
}
