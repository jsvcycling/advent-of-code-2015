use md5::{Digest, Md5};

fn compute(prefix: &'static str, goal: &'static str) -> usize {
    for i in 1.. {
        let hash = Md5::digest(format!("{}{}", prefix, i).as_bytes());

        if format!("{:x}", hash).starts_with(goal) {
            return i;
        }
    }

    return 0;
}

pub fn main() {
    let input = "iwrupvqb";

    println!("Part 1: {}", compute(input, "00000"));
    println!("Part 2: {}", compute(input, "000000"));
}
