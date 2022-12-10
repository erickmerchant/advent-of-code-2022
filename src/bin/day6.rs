use std::{collections::HashSet, fs};

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

        let len = 13;

        let mut pos = len;
        let chars: Vec<char> = input.chars().collect();

        'outer: while pos < input.len() {
            let mut set: HashSet<char> = HashSet::new();
            let mut pos2 = pos - 1;

            set.insert(chars[pos]);

            while pos2 >= pos - len {
                if set.contains(&chars[pos2]) {
                    pos += 1;

                    continue 'outer;
                }

                set.insert(chars[pos2]);

                pos2 -= 1;
            }

            break;
        }

        println!("{}", pos + 1);
    }
}
