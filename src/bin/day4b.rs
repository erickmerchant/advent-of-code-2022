use std::fs;

fn main() {
    if let Ok(input) = fs::read_to_string("input/day4.txt") {
        let lines = input.trim().split('\n');
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

                    if (first_start < last_start && first_end >= last_start)
                        || (last_start < first_start && last_end >= first_start)
                        || (first_start == last_start)
                    {
                        total += 1;
                    }
                }
            }
        }

        println!("{total}");
    }
}
