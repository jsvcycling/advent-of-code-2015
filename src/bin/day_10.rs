fn compute(initial_input: &str, rounds: usize) -> usize {
    let mut input = String::from(initial_input);

    for _ in 0..rounds {
        let mut result = String::new();
        let mut iter = input.chars();

        let mut ch = iter.next().unwrap();
        let mut count = 1;

        loop {
            if let Some(c) = iter.next() {
                if c == ch {
                    count += 1;
                    continue;
                }

                result.push_str(format!("{}{}", count, ch).as_str());
                ch = c;
                count = 1;
                continue;
            }

            result.push_str(format!("{}{}", count, ch).as_str());
            break;
        }

        input = result;
    }

    input.len()
}

pub fn main() {
    let input = "1113222113";

    println!("Part 1: {}", compute(&input, 40));
    println!("Part 2: {}", compute(&input, 50));
}
