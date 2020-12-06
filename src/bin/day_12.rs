use std::io::prelude::*;

use std::fs::File;
use std::io::{BufReader, Result};

use json::JsonValue;

fn traverse_and_sum(value: &JsonValue, skip_red: bool) -> i64 {
    match value {
        JsonValue::Number(_) => value.as_i64().unwrap(),
        JsonValue::Object(obj) => {
            let mut sum: i64 = 0;

            for v in obj.iter() {
                // If skip_red is enabled and one of the values in the object is
                // "red" then immediately stop and return 0.
                if skip_red && v.1.is_string() && v.1.as_str().unwrap() == "red".to_string() {
                    return 0;
                }

                sum += traverse_and_sum(v.1, skip_red);
            }

            sum
        }
        JsonValue::Array(vals) => vals.iter().map(|v| traverse_and_sum(v, skip_red)).sum(),
        _ => 0,
    }
}

pub fn main() -> Result<()> {
    let mut f = BufReader::new(File::open("inputs/day_12.txt")?);

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let root = json::parse(buf.as_str()).unwrap();

    let part1 = traverse_and_sum(&root, false);
    let part2 = traverse_and_sum(&root, true);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
