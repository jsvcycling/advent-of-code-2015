use std::io::prelude::*;

use std::fs::File;
use std::io::{BufReader, Result};

pub fn main() -> Result<()> {
    let mut f = BufReader::new(File::open("inputs/day_01.txt")?);

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let mut first_basement = -1;

    let result: i64 = buf
        .char_indices()
        .scan(0, |state, c| {
            *state += match c.1 {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

            if *state < 0 && first_basement < 0 {
                first_basement = c.0 as i64 + 1;
            }

            Some(*state)
        })
        .last()
        .unwrap();

    println!("Part 1: {}", result);
    println!("Part 2: {}", first_basement);

    Ok(())
}
