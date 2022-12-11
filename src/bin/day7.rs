use std::{collections::HashMap, fs, str::Split};

fn get_total<F: Fn(HashMap<String, u32>) -> u32>(lines: Split<char>, process_sizes: F) -> u32 {
    let mut path: Vec<String> = vec![];
    let mut sizes: HashMap<String, u32> = HashMap::new();
    let mut listing = false;

    for line in lines {
        if listing && !line.starts_with('$') {
            if !line.starts_with("dir") {
                if let Some((file_size, _)) = line.split_once(' ') {
                    let file_size = file_size.parse::<u32>().unwrap();

                    for dir in &path {
                        sizes
                            .entry(dir.clone())
                            .and_modify(|file_size_total| *file_size_total += file_size)
                            .or_insert(file_size);
                    }
                }
            }

            continue;
        }

        listing = false;

        if line == "$ ls" {
            listing = true;

            continue;
        }

        if line.starts_with("$ cd ") {
            if let Some((_, mut dir)) = line.split_once("$ cd ") {
                if dir == ".." {
                    path.pop();

                    continue;
                }

                if dir == "/" {
                    dir = "";
                }

                if let Some(last) = path.last() {
                    path.push(format!("{}/{}", last, dir));
                } else {
                    path.push(dir.to_string());
                }
            }
        }
    }

    process_sizes(sizes)
}

fn main() {
    if let Ok(input) = fs::read_to_string("input/day7.txt") {
        let lines = input.split('\n');

        let total = get_total(lines.clone(), |sizes| {
            let mut total = 0;

            for size in sizes.values() {
                if size <= &100000 {
                    total += size;
                }
            }

            total
        });

        println!("{total}");

        let total = get_total(lines, |sizes| {
            let mut total = 0;

            if let Some(root_size) = sizes.get("") {
                let goal = 30000000 - (70000000 - root_size);

                for size in sizes.values() {
                    if size >= &goal && (size < &total || total == 0) {
                        total = *size;
                    }
                }
            }

            total
        });

        println!("{total}");
    }
}
