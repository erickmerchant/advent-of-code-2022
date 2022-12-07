use std::fs;

fn main() {
    if let Ok(input) = fs::read_to_string("input/day1.txt") {
        let mut totals: Vec<i32> = vec![];
        let mut total = 0;
        let lines = input.split('\n');

        for line in lines {
            if !line.is_empty() {
                let number: i32 = line.parse().unwrap();

                total += number;
            } else {
                totals.push(total);

                total = 0;
            }
        }

        totals.sort();
        totals.reverse();

        let maxes = &totals[0..3];

        let maxes: i32 = maxes.iter().sum();

        println!("{maxes}");
    }
}
