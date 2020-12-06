const VALID_LETTERS: &str = "abcdefghjkmnpqrstuvwxyz";

fn inc_letter(c: char) -> char {
    if c == 'z' {
        return 'a';
    }

    let idx = VALID_LETTERS.find(c).unwrap();

    VALID_LETTERS.as_bytes()[idx + 1] as char
}

// Check to make sure the password has a three character run (e.g. 'abc').
fn check_run(chars: &Vec<char>) -> bool {
    for idx in 0..(chars.len() - 2) {
        let c1 = chars[idx] as u8;
        let c2 = chars[idx + 1] as u8;
        let c3 = chars[idx + 2] as u8;

        if c1 == c2 - 1 && c1 == c3 - 2 {
            return true;
        }
    }

    false
}

// Check to make sure the password has at least two non-overlapping character
// pairs (e.g. 'aa' or 'bb'; 'aaa' would count as one pair).
fn check_pairs(chars: &Vec<char>) -> bool {
    let mut seen_pair = false;
    let mut skip_next = false;

    for idx in 0..(chars.len() - 1) {
        if skip_next {
            skip_next = false;
            continue;
        }

        let c1 = chars[idx];
        let c2 = chars[idx + 1];

        if c1 == c2 {
            if seen_pair {
                return true;
            }

            seen_pair = true;
            skip_next = true;
        }
    }

    false
}

fn compute(curr_passwd: &str) -> String {
    let mut chars: Vec<char> = curr_passwd.chars().collect();

    loop {
        for idx in (0..chars.len()).rev() {
            if idx == chars.len() - 1 || chars[idx + 1] == 'a' {
                chars[idx] = inc_letter(chars[idx]);
                continue;
            }

            break;
        }

        if check_run(&chars) && check_pairs(&chars) {
            break;
        }
    }

    chars.iter().collect()
}

pub fn main() {
    let input = "hxbxwxba";

    let part1 = compute(&input);
    let part2 = compute(&part1.as_str());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
