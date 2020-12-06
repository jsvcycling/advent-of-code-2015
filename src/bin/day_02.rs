use std::fs::read_to_string;

pub fn main() {
    let buf = read_to_string("inputs/day_02.txt").unwrap();

    let results: (u32, u32) = buf
        .lines()
        .scan((0, 0), |state, l| {
            // Parse the line into a vector of u32's.
            let vals: Vec<u32> = l
                .split('x')
                .map(|v| v.trim().parse::<u32>().unwrap())
                .collect();

            // Compute the area of each side type, find the smallest side (as
            // slack) and double each side (front & back, left & right, top &
            // bottom).
            let sides = vec![vals[0] * vals[1], vals[0] * vals[2], vals[1] * vals[2]];
            let slack = sides.iter().min().unwrap();
            let sides = sides.iter().sum::<u32>();

            state.0 += sides * 2 + slack;

            // Find the longest side, the total sum of sides, and the amount of
            // ribbon needed for the now.
            let longest_side = vals.iter().max().unwrap();
            let sides = vals.iter().sum::<u32>();
            let bow = vals.iter().product::<u32>();

            state.1 += (sides - longest_side) * 2 + bow;

            Some(*state)
        })
        .last()
        .unwrap();

    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);
}
