use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const PUZZLE_PART: PuzzlePart = PuzzlePart::One;

#[derive(PartialEq, Eq)]
enum PuzzlePart {
    One,
    Two,
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut pairs = vec![];

    for line in reader.lines().flatten() {
        if line.is_empty() {
            continue;
        }
        pairs.push(line);
    }

    let count = pairs
        .into_iter()
        .map(|pair| {
            pair.split(',')
                .into_iter()
                .map(|range| {
                    range
                        .split("-")
                        .map(|number| number.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|range| range[0]..=range[1])
                .collect::<Vec<_>>()
        })
        .filter(|ranges| match PUZZLE_PART {
            PuzzlePart::One => {
                ranges[0].clone().all(|number| ranges[1].contains(&number))
                    || ranges[1].clone().all(|number| ranges[0].contains(&number))
            }
            PuzzlePart::Two => {
                ranges[0].clone().any(|number| ranges[1].contains(&number))
                    || ranges[1].clone().any(|number| ranges[0].contains(&number))
            }
        })
        .count();

    println!("The number of overlaps is {}.", count);

    Ok(())
}
