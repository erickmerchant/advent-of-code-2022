use std::{fs, str::Split};

fn get_total<F: Fn(u32, u32, u32, u32) -> i32>(lines: Split<char>, get_line_total: F) -> i32 {
    let mut total = 0;

    for line in lines {
        if let Some((first, last)) = line.split_once(',') {
            if let (Some((first_start, first_end)), Some((last_start, last_end))) =
                (first.split_once('-'), last.split_once('-'))
            {
                let first_start = first_start.parse::<u32>().unwrap();
                let first_end = first_end.parse::<u32>().unwrap();
                let last_start = last_start.parse::<u32>().unwrap();
                let last_end = last_end.parse::<u32>().unwrap();

                total += get_line_total(first_start, first_end, last_start, last_end);
            }
        }
    }

    total
}

fn main() {
    if let Ok(input) = fs::read_to_string("input/day4.txt") {
        let lines = input.trim().split('\n');

        let total = get_total(
            lines.clone(),
            |first_start, first_end, last_start, last_end| {
                #[allow(clippy::bool_to_int_with_if)]
                if first_start <= last_start && first_end >= last_end
                    || last_start <= first_start && last_end >= first_end
                {
                    1
                } else {
                    0
                }
            },
        );

        println!("{total}");

        let total = get_total(
            lines.clone(),
            |first_start, first_end, last_start, last_end| {
                #[allow(clippy::bool_to_int_with_if)]
                if (first_start < last_start && first_end >= last_start)
                    || (last_start < first_start && last_end >= first_start)
                    || (first_start == last_start)
                {
                    1
                } else {
                    0
                }
            },
        );

        println!("{total}");
    }
}
