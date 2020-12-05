use std::io::prelude::*;

use std::fs::File;
use std::io::{BufReader, Result};

pub fn main() -> Result<()> {
    let f = BufReader::new(File::open("inputs/day_08.txt")?);
    let lines: Vec<_> = f.lines().map(|l| l.unwrap()).collect();

    let mut code_total = 0;

    let part1: usize = lines
        .iter()
        .map(|line| {
            code_total += line.len();

            let mut line = line
                .trim_matches('"')
                .replace("\\\\", ";")
                .replace("\\\"", ":");

            while let Some(i) = line.find(r"\x") {
                let hex = String::from_utf8(line.as_bytes()[i..(i + 4)].to_vec()).unwrap();
                line = line.replace(&hex, "?");
            }
            line.len()
        })
        .sum();

    let part2: usize = lines
        .iter()
        .map(|line| {
            let line = line.replace("\\", ";;").replace("\"", "::");
            line.len() + 2 // Two extra characters for the outer quotes.
        })
        .sum();

    println!("{:?}", code_total - part1);
    println!("{:?}", part2 - code_total);

    Ok(())
}
