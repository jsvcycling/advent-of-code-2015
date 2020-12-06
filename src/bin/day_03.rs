use std::collections::HashMap;
use std::fs::read_to_string;

fn compute(buf: &String, with_robot: bool) -> usize {
    let mut santa_pos = (0, 0);
    let mut robot_pos = (0, 0);

    let mut houses = HashMap::new();
    houses.insert((0, 0), 1); // The starting house will get a gift.

    let mut is_robot = false;

    buf.chars().for_each(|c| {
        let movement = match c {
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
    });

    houses.iter().count()
}

pub fn main() {
    let buf = read_to_string("inputs/day_03.txt").unwrap();

    println!("Part 1: {}", compute(&buf, false));
    println!("Part 2: {}", compute(&buf, true));
}
