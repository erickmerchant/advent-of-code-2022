use std::{fs, str::Split};

fn get_letters<F: Fn(&mut Vec<char>, &mut Vec<char>)>(lines: Split<char>, do_moves: F) -> String {
    let mut stacks: Vec<Vec<char>> = vec![];

    for _ in 0..9 {
        stacks.push(vec![]);
    }

    for line in lines.clone() {
        if line == " 1   2   3   4   5   6   7   8   9 " {
            break;
        }

        let mut pos = 1;

        for stack in stacks.iter_mut() {
            if let Some(c) = line.chars().nth(pos) {
                if c != ' ' {
                    stack.insert(0, c);
                }
            };

            pos += 4;
        }
    }

    for line in lines {
        if line.starts_with("move") {
            let parts = line.split_terminator(' ').collect::<Vec<&str>>();
            let mut quantity = parts[1].parse::<usize>().unwrap();
            let origin = parts[3].parse::<usize>().unwrap();
            let destination = parts[5].parse::<usize>().unwrap();

            let stack_origin = stacks.get_mut(origin - 1).unwrap();
            let mut moves: Vec<char> = vec![];

            while quantity > 0 {
                if let Some(m) = stack_origin.pop() {
                    moves.insert(0, m);
                }

                quantity -= 1;
            }

            let stack_destination = stacks.get_mut(destination - 1).unwrap();

            do_moves(&mut moves, stack_destination);
        }
    }

    let mut message: String = String::from("");

    for stack in stacks.iter_mut() {
        message.push(*stack.last().unwrap());
    }

    message
}

fn main() {
    if let Ok(input) = fs::read_to_string("input/day5.txt") {
        let lines = input.split('\n');

        let message = get_letters(lines.clone(), |moves, stack_destination| {
            for m in moves.iter_mut().rev() {
                stack_destination.push(*m);
            }
        });

        println!("{message}");

        let message = get_letters(lines.clone(), |moves, stack_destination| {
            for m in moves.iter_mut() {
                stack_destination.push(*m);
            }
        });

        println!("{message}");
    }
}
