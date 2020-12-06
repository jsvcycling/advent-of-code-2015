fn compute(initial_input: &str, rounds: usize) -> String {
    (0..rounds).fold(initial_input.to_string(), |input, _| {
        let mut result = String::new();
        let chars: Vec<char> = input.chars().collect();
        let mut count = 1;

        for idx in 0..chars.len() {
            if idx + 1 >= chars.len() {
                result.push_str(format!("{}{}", count, chars[idx]).as_str());
                break;
            }

            if chars[idx] == chars[idx + 1] {
                count += 1;
            } else {
                result.push_str(format!("{}{}", count, chars[idx]).as_str());
                count = 1;
            }
        }

        result
    })
}

pub fn main() {
    let input = "1113222113";

    println!("Part 1: {}", compute(&input, 40).len());
    println!("Part 2: {}", compute(&input, 50).len());
}
