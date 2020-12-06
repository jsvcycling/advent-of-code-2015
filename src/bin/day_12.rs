use std::fs::read_to_string;

use json::JsonValue;

fn compute(value: &JsonValue, skip_red: bool) -> i64 {
    match value {
        JsonValue::Number(_) => value.as_i64().unwrap(),
        JsonValue::Object(obj) => {
            let mut sum: i64 = 0;

            for (_, v) in obj.iter() {
                // Skip the object if it contains the value "red".
                if skip_red && v.is_string() && v.as_str().unwrap() == "red".to_string() {
                    return 0;
                }

                sum += compute(v, skip_red);
            }

            sum
        }
        JsonValue::Array(vals) => vals.iter().fold(0, |acc, v| acc + compute(v, skip_red)),
        _ => 0,
    }
}

pub fn main() {
    let buf = read_to_string("inputs/day_12.txt").unwrap();
    let root = json::parse(buf.as_str()).unwrap();

    let part1 = compute(&root, false);
    let part2 = compute(&root, true);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
