use std::fs;

fn main() {
    if let Ok(input) = fs::read_to_string("input/day6.txt") {
        let mut pos = 3;
        let chars: Vec<char> = input.chars().collect();

        while pos < input.len() {
            if chars[pos] != chars[pos - 1]
                && chars[pos] != chars[pos - 2]
                && chars[pos] != chars[pos - 3]
                && chars[pos - 1] != chars[pos - 2]
                && chars[pos - 1] != chars[pos - 3]
                && chars[pos - 2] != chars[pos - 3]
            {
                break;
            }

            pos += 1;
        }

        println!("{}", pos + 1);
    }
}
