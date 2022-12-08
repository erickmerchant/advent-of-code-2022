use std::{collections::HashSet, fs};

fn main() {
    let len = 13;

    if let Ok(input) = fs::read_to_string("input/day6.txt") {
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
