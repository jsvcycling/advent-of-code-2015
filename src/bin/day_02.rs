use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn main() -> Result<()> {
    let file = File::open("inputs/day_02.txt")?;
    let mut reader = BufReader::new(file);

    let mut total_paper = 0;
    let mut total_ribbon = 0;

    loop {
        let mut buffer = String::new();
        let result = reader.read_line(&mut buffer);

        if result.is_err() || result.unwrap() == 0 {
            break;
        }

        // Parse the line into a vector of u32's.
        let vals: Vec<u32> = buffer
            .split("x")
            .map(|v| v.trim().parse::<u32>().unwrap())
            .collect();

        // Wrapping paper
        {
            // Compute the area of each side type, find the smallest side (as
            // slack) and double each side (front & back, left & right, top &
            // bottom).
            let sides = vec![vals[0] * vals[1], vals[0] * vals[2], vals[1] * vals[2]];
            let slack = sides.iter().min().unwrap();
            let sides: u32 = sides.iter().map(|v| 2 * v).sum();

            total_paper += sides + slack;
        }

        // Ribbon
        {
            // Find the longest side, the total sum of sides, and the amount of
            // ribbon needed for the now.
            let longest_side = vals.iter().max().unwrap();
            let sides: u32 = vals.iter().sum();
            let bow = vals.iter().fold(1, |acc, v| acc * v);

            total_ribbon += (sides - longest_side) * 2 + bow;
        }
    }

    println!("Part 1: {}", total_paper);
    println!("Part 2: {}", total_ribbon);

    Ok(())
}
