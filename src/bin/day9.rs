use std::{collections::HashSet, fs, str::Split};

#[derive(Copy, Clone)]
struct Knot {
    x: i32,
    y: i32,
}

fn get_positions(lines: Split<char>, knots: &mut Vec<Knot>) -> HashSet<(i32, i32)> {
    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    positions.insert((0, 0));

    let do_moves = move |xy: (i32, i32), head: &mut Knot, tail_x: i32, tail_y: i32| -> (i32, i32) {
        let (x, y) = xy;

        head.x += x;
        head.y += y;

        let x_high = (tail_x - head.x).abs() >= 2;
        let y_high = (tail_y - head.y).abs() >= 2;

        let mut result = (0, 0);

        if x_high || y_high {
            if tail_x == head.x {
                result = (0, y);
            } else if tail_y == head.y {
                result = (x, 0);
            } else if head.x > tail_x && head.y > tail_y {
                result = (1, 1);
            } else if head.x > tail_x && head.y < tail_y {
                result = (1, -1);
            } else if head.x < tail_x && head.y > tail_y {
                result = (-1, 1);
            } else if head.x < tail_x && head.y < tail_y {
                result = (-1, -1);
            }
        }

        result
    };

    for line in lines {
        if let Some((direction, steps)) = line.split_once(' ') {
            let steps = steps.parse::<i32>().unwrap();

            for _ in 0..steps {
                let mut next_move = match direction {
                    "U" => (0, -1),
                    "D" => (0, 1),
                    "L" => (-1, 0),
                    "R" => (1, 0),
                    _ => (0, 0),
                };

                for i in 0..knots.len() {
                    let next_x = if let Some(knot) = knots.get(i + 1) {
                        knot.x
                    } else {
                        0
                    };
                    let next_y = if let Some(knot) = knots.get(i + 1) {
                        knot.y
                    } else {
                        0
                    };

                    next_move = do_moves(next_move, &mut knots[i], next_x, next_y);
                }

                if let Some(last_knot) = knots.last() {
                    positions.insert((last_knot.x, last_knot.y));
                }
            }
        }
    }

    positions
}

fn main() {
    if let Ok(input) = fs::read_to_string("input/day9.txt") {
        let lines = input.split('\n');

        let mut knots: Vec<Knot> = vec![Knot { x: 0, y: 0 }, Knot { x: 0, y: 0 }];

        let positions = get_positions(lines.clone(), &mut knots);

        println!("{}", positions.len());

        let mut knots: Vec<Knot> = vec![
            Knot { x: 0, y: 0 },
            Knot { x: 0, y: 0 },
            Knot { x: 0, y: 0 },
            Knot { x: 0, y: 0 },
            Knot { x: 0, y: 0 },
            Knot { x: 0, y: 0 },
            Knot { x: 0, y: 0 },
            Knot { x: 0, y: 0 },
            Knot { x: 0, y: 0 },
            Knot { x: 0, y: 0 },
        ];

        let positions = get_positions(lines, &mut knots);

        println!("{}", positions.len());
    }
}
