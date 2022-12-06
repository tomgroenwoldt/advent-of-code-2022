use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const PUZZLE_PART: PuzzlePart = PuzzlePart::One;
const NUMBER_OF_STACKS: usize = 9;

#[derive(PartialEq, Eq)]
enum PuzzlePart {
    One,
    Two,
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut stacks = vec![vec![]; NUMBER_OF_STACKS];
    let mut movements: Vec<Move> = vec![];
    let mut read_stack = true;

    for line in reader.lines().flatten() {
        if line.is_empty() {
            read_stack = false;
            continue;
        }
        if read_stack {
            line.chars()
                .enumerate()
                .filter(|(index, char)| (*index as i32 - 1) % 4 == 0 && char.is_alphabetic())
                .map(|(index, char)| ((index - 1) / 4, char))
                .for_each(|(index, char)| stacks[index].insert(0, char));
        } else {
            let movement = line.split(' ').collect::<Vec<_>>();
            movements.push(Move {
                count: movement[1].parse::<usize>().unwrap(),
                from: movement[3].parse::<usize>().unwrap() - 1,
                to: movement[5].parse::<usize>().unwrap() - 1,
            })
        }
    }

    match PUZZLE_PART {
        PuzzlePart::One => {
            movements.iter().for_each(|movement| {
                for _ in 1..=movement.count {
                    let item = stacks[movement.from].pop().unwrap();
                    stacks[movement.to].push(item);
                }
            });
        }
        PuzzlePart::Two => {
            movements.iter().for_each(|movement| {
                let stack_length = stacks[movement.from].len();
                let items: Vec<char> =
                    stacks[movement.from][stack_length - movement.count..stack_length].to_vec();
                stacks[movement.from] =
                    stacks[movement.from][..stack_length - movement.count].to_vec();
                stacks[movement.to].append(&mut items.to_vec());
            });
        }
    };

    let top_items = stacks
        .iter_mut()
        .filter_map(|stack| stack.pop())
        .collect::<Vec<_>>();

    println!("{:?}", top_items);

    Ok(())
}
