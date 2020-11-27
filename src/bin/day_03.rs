use std::io::prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Result, SeekFrom};

fn compute(reader: &mut BufReader<File>, with_robot: bool) -> Result<usize> {
    let mut buffer = reader.fill_buf()?;

    let mut santa_pos = (0, 0);
    let mut robot_pos = (0, 0);

    let mut houses = HashMap::new();
    houses.insert((0, 0), 1); // The starting house will get a gift.

    let mut is_robot = false;

    while !buffer.is_empty() {
        for ch in buffer {
            let movement = match *ch as char {
                '^' => (0, 1),
                'v' => (0, -1),
                '<' => (-1, 0),
                '>' => (1, 0),
                _ => (0, 0),
            };

            // If we have a robot and it's the robot's turn, update it's
            // position. Otherwise, update Santa's position.
            if with_robot && is_robot {
                robot_pos.0 += movement.0;
                robot_pos.1 += movement.1;

                houses.insert(robot_pos, 1);
            } else {
                santa_pos.0 += movement.0;
                santa_pos.1 += movement.1;

                houses.insert(santa_pos, 1);
            }

            is_robot = !is_robot;
        }

        let length = buffer.len();
        reader.consume(length);
        buffer = reader.fill_buf()?;
    }

    Ok(houses.iter().count())
}

pub fn main() -> Result<()> {
    let file = File::open("inputs/day_03.txt")?;
    let mut reader = BufReader::new(file);

    println!("Part 1: {}", compute(&mut reader, false)?);

    reader.seek(SeekFrom::Start(0))?;

    println!("Part 2: {}", compute(&mut reader, true)?);

    Ok(())
}
