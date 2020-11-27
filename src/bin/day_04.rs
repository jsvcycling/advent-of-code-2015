use md5::{Digest, Md5};

// Just a simple debug flag.
const DEBUG: bool = false;

// The prefix provided by AoC.
const PREFIX: &str = "iwrupvqb";

fn compute(goal: &'static str) -> usize {
    let mut i = 1;

    loop {
        let hash = Md5::digest(format!("{}{}", PREFIX, i).as_bytes());

        let mut result = format!("{:x}", hash);
        result.truncate(goal.len());

        if DEBUG {
            println!("{} = {}", i, result);
        }

        if result == goal {
            return i;
        }

        i += 1;
    }
}

pub fn main() {
    println!("Part 1: {}", compute("00000"));
    println!("Part 2: {}", compute("000000"));
}
