use std::fs;

fn main() {
    if let Ok(input) = fs::read_to_string("input/day2.txt") {
        let lines = input.split('\n');
        let mut total = 0;

        for line in lines {
            if let Some((them, me)) = line.split_once(' ') {
                total += match them {
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
                }
            }
        }

        println!("{total}");
    }
}
