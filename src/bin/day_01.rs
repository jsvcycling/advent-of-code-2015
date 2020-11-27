use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn main() -> Result<()> {
    let file = File::open("inputs/day_01.txt")?;
    let mut reader = BufReader::new(file);

    let mut first_basement = -1;
    let mut position = 0;
    let mut result = 0;

    let mut buffer = reader.fill_buf()?;

    while !buffer.is_empty() {
        for ch in buffer {
            result += match *ch {
                0x28 => 1,  // The ASCII hexcode for '('.
                0x29 => -1, // The ASCII hexcode for ')'.
                _ => 0,
            };

            position += 1;

            if result < 0 && first_basement < 0 {
                first_basement = position;
            }
        }

        let length = buffer.len();
        reader.consume(length);
        buffer = reader.fill_buf()?;
    }

    println!("Part 1: {}", result);
    println!("Part 2: {}", first_basement);

    Ok(())
}
