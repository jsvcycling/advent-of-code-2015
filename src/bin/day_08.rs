use std::fs::read_to_string;

pub fn main() {
    let buf = read_to_string("inputs/day_08.txt").unwrap();

    let mut code_total = 0;

    let part1: usize = buf.lines().fold(0, |acc, line| {
        code_total += line.len();

        let mut line = line
            .trim_matches('"')
            .replace("\\\\", ";")
            .replace("\\\"", ":");

        while let Some(i) = line.find(r"\x") {
            line = line.replace(line.get(i..(i + 4)).unwrap(), "?");
        }

        acc + line.len()
    });

    let part2: usize = buf.lines().fold(0, |acc, line| {
        // Two extra characters for the outer quotes.
        acc + line.replace("\\", ";;").replace("\"", "::").len() + 2
    });

    println!("{}", code_total - part1);
    println!("{}", part2 - code_total);
}
