use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

const PUZZLE_PART: PuzzlePart = PuzzlePart::Two;

#[derive(PartialEq, Eq)]
enum PuzzlePart {
    One,
    Two,
}

use num::integer::gcd;

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<usize>,
    inspection_count: usize,
    operator: String,
    operator_value: Result<usize, ParseIntError>,
    test_value: usize,
    test_true_monkey: usize,
    test_false_monkey: usize,
}

impl From<Vec<String>> for Monkey {
    fn from(value: Vec<String>) -> Self {
        let id = value[0].split(' ').collect::<Vec<_>>()[1]
            .replace(':', "")
            .parse::<usize>()
            .unwrap();
        let item_split_index = value[1].find(':').unwrap() + 1;
        let items = value[1]
            .split_at(item_split_index)
            .1
            .trim()
            .replace(',', "")
            .split(' ')
            .collect::<Vec<_>>()
            .iter()
            .map(|value| value.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let operation = value[2].trim().replace("Operation: new = old ", "");
        let operation = operation.split(' ').collect::<Vec<_>>();

        let test_value = value[3]
            .trim()
            .replace("Test: divisible by ", "")
            .parse::<usize>()
            .unwrap();
        let test_true_monkey = value[4]
            .trim()
            .replace("If true: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();
        let test_false_monkey = value[5]
            .trim()
            .replace("If false: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();

        Monkey {
            id,
            items,
            inspection_count: 0,
            operator: operation[0].to_string(),
            operator_value: operation[1].parse::<usize>(),
            test_value,
            test_true_monkey,
            test_false_monkey,
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut monkey_lines = vec![];
    let mut monkeys: Vec<Monkey> = vec![];
    let (worry_level_divisor, round_count) = match PUZZLE_PART {
        PuzzlePart::One => (3, 20),
        PuzzlePart::Two => (1, 10000),
    };

    for line in reader.lines().flatten() {
        if line.is_empty() {
            continue;
        }
        if line.trim().starts_with("If false") {
            monkey_lines.push(line);
            monkeys.push(monkey_lines.clone().into());
            monkey_lines.clear();
        } else {
            monkey_lines.push(line);
        }
    }

    let test_values = monkeys
        .iter()
        .map(|monkey| monkey.test_value)
        .collect::<Vec<_>>();

    let product = test_values.iter().product::<usize>();
    let gcd = test_values
        .iter()
        .fold(test_values[0], |acc, &x| gcd(acc, x));
    let lcm = product / gcd;

    for _ in 0..round_count {
        for monkey in &mut monkeys.clone() {
            let mut item = monkeys[monkey.id].items.pop();
            while item.is_some() {
                let old_item = item.unwrap();
                let new_item = match (monkey.operator_value.clone(), monkey.operator.as_str()) {
                    (Ok(value), "*") => (old_item * value) / worry_level_divisor,
                    (Ok(value), "+") => (old_item + value) / worry_level_divisor,
                    (Err(_), "*") => (old_item * old_item) / worry_level_divisor,
                    (Err(_), "+") => (old_item + old_item) / worry_level_divisor,
                    _ => panic!("Error executing monkey operation."),
                };
                if new_item % monkey.test_value == 0 {
                    monkeys[monkey.test_true_monkey].items.push(new_item % lcm);
                } else {
                    monkeys[monkey.test_false_monkey].items.push(new_item % lcm);
                }
                monkeys[monkey.id].inspection_count += 1;

                item = monkeys[monkey.id].items.pop();
            }
        }
    }

    let mut inspection_counts = monkeys
        .into_iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<_>>();
    inspection_counts.sort();
    inspection_counts.reverse();

    let monkey_business = inspection_counts[0] * inspection_counts[1];
    println!("The level of monkey business is {}.", monkey_business);

    Ok(())
}
