use std::{fs, str::Split};

fn get_total<F: Fn(&str, &str) -> i32>(lines: Split<char>, get_line_total: F) -> i32 {
    let mut total = 0;

    for line in lines {
        if let Some((them, me)) = line.split_once(' ') {
            total += get_line_total(them, me);
        }
    }

    total
}

fn main() {
    if let Ok(input) = fs::read_to_string("input/day2.txt") {
        let lines = input.split('\n');

        let total = get_total(lines.clone(), |them, me| match them {
            "A" => match me {
                "X" => 1 + 3,
                "Y" => 2 + 6,
                "Z" => 3,
                _ => 0,
            },
            "B" => match me {
                "X" => 1,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                _ => 0,
            },
            "C" => match me {
                "X" => 1 + 6,
                "Y" => 2,
                "Z" => 3 + 3,
                _ => 0,
            },
            _ => 0,
        });

        println!("{total}");

        let total = get_total(lines.clone(), |them, me| match them {
            "A" => match me {
                "X" => 3,
                "Y" => 3 + 1,
                "Z" => 6 + 2,
                _ => 0,
            },
            "B" => match me {
                "X" => 1,
                "Y" => 3 + 2,
                "Z" => 6 + 3,
                _ => 0,
            },
            "C" => match me {
                "X" => 2,
                "Y" => 3 + 3,
                "Z" => 6 + 1,
                _ => 0,
            },
            _ => 0,
        });

        println!("{total}");
    }
}
