use std::{fs, str::Split};

fn get_total(lines: Split<char>, mut cycles: Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    let mut x_total: i32 = 1;
    let mut current_cycle: i32 = 0;

    let mut step = |x_total: i32| {
        current_cycle += 1;

        if let Some(next_cycle) = cycles.last() {
            if next_cycle == &current_cycle {
                total += x_total * current_cycle;

                cycles.pop();
            }
        }
    };

    for line in lines {
        if line.starts_with("addx") {
            if let Some((_, x)) = line.split_once(' ') {
                let x = x.parse::<i32>().unwrap();

                step(x_total);

                step(x_total);

                x_total += x;
            }
        }

        if line == "noop" {
            step(x_total);
        }
    }

    total
}

fn main() {
    if let Ok(input) = fs::read_to_string("input/day10.txt") {
        let lines = input.split('\n');
        let cycles: Vec<i32> = vec![220, 180, 140, 100, 60, 20];
        let total = get_total(lines.clone(), cycles);

        println!("{total}");
    }
}
