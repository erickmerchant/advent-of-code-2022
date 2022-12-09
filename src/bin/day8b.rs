use std::fs;

struct Tree {
    height: u8,
}

fn main() {
    if let Ok(input) = fs::read_to_string("input/day8.txt") {
        let lines = input.split('\n');
        let mut grid: Vec<Vec<Tree>> = Vec::new();

        for line in lines {
            if line.is_empty() {
                break;
            }

            let mut g: Vec<Tree> = Vec::new();

            g.extend(line.chars().map(|c| Tree {
                height: c.to_string().parse::<u8>().unwrap(),
            }));

            grid.push(g);
        }

        let row_length: usize = grid[0].len();
        let column_length: usize = grid.len();
        let mut highest = 0;

        for y in 0..column_length {
            for x in 0..row_length {
                let mut score = 1;
                let mut direction = 0;

                for east in x + 1..row_length {
                    if grid[y][east].height < grid[y][x].height {
                        direction += 1;
                    } else {
                        direction += 1;
                        break;
                    }
                }

                score *= direction;

                let mut direction = 0;

                for west in (0..x).rev() {
                    if grid[y][west].height < grid[y][x].height {
                        direction += 1;
                    } else {
                        direction += 1;
                        break;
                    }
                }

                score *= direction;

                let mut direction = 0;

                for south in y + 1..column_length {
                    if grid[south][x].height < grid[y][x].height {
                        direction += 1;
                    } else {
                        direction += 1;
                        break;
                    }
                }

                score *= direction;

                let mut direction = 0;

                for north in (0..y).rev() {
                    if grid[north][x].height < grid[y][x].height {
                        direction += 1;
                    } else {
                        direction += 1;
                        break;
                    }
                }

                score *= direction;

                if score > highest {
                    highest = score;
                }
            }
        }

        println!("{highest}");
    }
}
