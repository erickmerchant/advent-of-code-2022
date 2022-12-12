use regex::Regex;
use std::{fs, str::Split};

#[derive(PartialEq)]
enum Operation {
    Add,
    Multiply,
}

struct Monkey {
    items: Vec<usize>,
    operation_amount: Option<usize>,
    operation: Option<Operation>,
    test: usize,
    winner: usize,
    loser: usize,
    inspection_count: usize,
}

fn get_monkey_business(blocks: Split<&str>, decreaser: usize, rounds: usize) -> usize {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut test_total: usize = 1;

    for block in blocks {
        let mut items: Vec<usize> = vec![];
        let mut operation_amount = None;
        let operation = if block.contains("Operation: new = old *") {
            Some(Operation::Multiply)
        } else if block.contains("Operation: new = old +") {
            Some(Operation::Add)
        } else {
            None
        };
        let mut test = 0;
        let mut winner = 0;
        let mut loser = 0;

        let lines = block.split('\n');
        for line in lines {
            let number_regex = Regex::new(r"(\d+)").unwrap();
            for cap in number_regex.captures_iter(line) {
                let num = cap[1].parse::<usize>().unwrap();

                if line.contains("Starting items:") {
                    items.push(num);
                }

                if line.contains("Operation:") {
                    operation_amount = Some(num);
                }

                if line.contains("Test: divisible by") {
                    test = num;

                    test_total *= num;
                }

                if line.contains("If true: throw to monkey") {
                    winner = num;
                }

                if line.contains("If false: throw to monkey") {
                    loser = num;
                }
            }
        }

        monkeys.push(Monkey {
            items,
            operation_amount,
            operation,
            test,
            winner,
            loser,
            inspection_count: 0,
        })
    }

    for _ in 1..=rounds {
        for monkey_index in 0..monkeys.len() {
            let mut changes: Vec<(usize, usize)> = vec![];

            if let Some(monkey) = monkeys.get_mut(monkey_index) {
                for index in 0..monkey.items.len() {
                    let old = monkey.items[index];

                    let mut worry_level = match monkey.operation {
                        Some(Operation::Multiply) => match monkey.operation_amount {
                            Some(amount) => old * amount,
                            None => old * old,
                        },
                        Some(Operation::Add) => match monkey.operation_amount {
                            Some(amount) => old + amount,
                            None => old + old,
                        },
                        None => old,
                    };

                    worry_level = (worry_level - (worry_level % decreaser)) / decreaser;

                    let new_worry_level = worry_level % test_total;

                    if new_worry_level % monkey.test == 0 {
                        changes.push((monkey.winner, new_worry_level))
                    } else {
                        changes.push((monkey.loser, new_worry_level))
                    }
                }

                monkey.inspection_count += monkey.items.len();

                monkey.items = vec![];
            }

            for change in changes {
                let (monkey_index, old) = change;

                if let Some(monkey) = monkeys.get_mut(monkey_index) {
                    monkey.items.push(old)
                }
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspection_count.partial_cmp(&a.inspection_count).unwrap());

    monkeys[0].inspection_count * monkeys[1].inspection_count
}

fn main() {
    if let Ok(input) = fs::read_to_string("input/day11.txt") {
        let blocks = input.split("\n\n");

        let result = get_monkey_business(blocks.clone(), 3, 20);

        println!("{result}");

        let result = get_monkey_business(blocks, 1, 10_000);

        println!("{result}");
    }
}
