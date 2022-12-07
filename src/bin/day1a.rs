use std::fs;

fn main() {
    if let Ok(input) = fs::read_to_string("input/day1.txt") {
        let mut total = 0;
        let mut max = 0;
        let lines = input.split('\n');

        for line in lines {
            if !line.is_empty() {
                let number: i32 = line.parse().unwrap();

                total += number;
            } else {
                if total > max {
                    max = total
                }

                total = 0;
            }
        }

        println!("{max}");
    }
}
