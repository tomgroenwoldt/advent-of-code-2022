#![feature(slice_as_chunks)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const PUZZLE_PART: PuzzlePart = PuzzlePart::Two;

#[derive(PartialEq, Eq)]
enum PuzzlePart {
    One,
    Two,
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut backpacks = vec![];

    for line in reader.lines().flatten() {
        if line.is_empty() {
            continue;
        }
        backpacks.push(line);
    }

    let mut sum_priorities = vec![];

    match PUZZLE_PART {
        PuzzlePart::One => {
            backpacks.iter().for_each(|backpack| {
                let (first_compartment, second_compartment) = backpack.split_at(backpack.len() / 2);

                let scoring = first_compartment
                    .chars()
                    .find(|item| second_compartment.contains(*item))
                    .map(|item| {
                        println!("{:?}", item);
                        if item.is_uppercase() {
                            item as u32 - 38
                        } else {
                            item as u32 - 96
                        }
                    })
                    .unwrap_or_default();

                sum_priorities.push(scoring);
            });
        }
        PuzzlePart::Two => {
            backpacks.chunks(3).for_each(|chunk| {
                let backpack_group = chunk.iter().collect::<Vec<_>>();

                let scoring = backpack_group[0]
                    .chars()
                    .find(|item| {
                        backpack_group[1].contains(*item) && backpack_group[2].contains(*item)
                    })
                    .map(|item| {
                        if item.is_uppercase() {
                            item as u32 - 38
                        } else {
                            item as u32 - 96
                        }
                    })
                    .unwrap_or_default();

                sum_priorities.push(scoring);
            });
        }
    }

    println!(
        "The sum of priorities is {:?}",
        sum_priorities.iter().sum::<u32>()
    );

    Ok(())
}
