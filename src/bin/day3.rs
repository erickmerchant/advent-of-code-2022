use std::{collections::HashSet, fs};

fn main() {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    if let Ok(input) = fs::read_to_string("input/day3.txt") {
        let lines = input.trim().split('\n');
        let mut total = 0;

        for line in lines {
            let mut set = HashSet::new();
            let mut needle: Option<char> = None;
            let (first, last) = line.split_at(line.len() / 2);

            for c in first.chars() {
                set.insert(c);
            }

            for c in last.chars() {
                if set.contains(&c) {
                    if needle.is_some() {
                        panic!("needle already exists")
                    }

                    needle = Some(c);

                    break;
                }
            }

            if let Some(needle) = needle {
                if let Some(needle_code) = letters.find(needle) {
                    total += needle_code + 1;
                }
            }
        }

        println!("{total}");

        let lines = input.trim().split('\n').collect::<Vec<&str>>();
        let mut total = 0;
        let mut i = 0;

        if lines.len() % 3 > 0 {
            panic!("group with less than 3 teams")
        }

        while lines.len() > i {
            if let (Some(alpha), Some(beta), Some(gamma)) =
                (lines.get(i), lines.get(i + 1), lines.get(i + 2))
            {
                i += 3;

                for needle in alpha.chars() {
                    if beta.contains(needle) && gamma.contains(needle) {
                        if let Some(needle_code) = letters.find(needle) {
                            total += needle_code + 1;
                        }

                        break;
                    }
                }
            }
        }

        println!("{total}");
    }
}
