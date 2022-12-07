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

fn all_distinct(marker: &mut Vec<char>) -> bool {
    // Sort and check if there exist duplicates.
    marker.sort();
    marker
        .iter()
        .enumerate()
        .filter(|(index, _)| {
            if *index < marker.len() - 1 {
                marker[*index] == marker[index + 1]
            } else {
                false
            }
        })
        .count()
        == 0
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut buffer = vec![];

    for line in reader.lines().flatten() {
        buffer = line.chars().collect::<Vec<_>>();
    }

    let distinct_character_count = match PUZZLE_PART {
        PuzzlePart::One => 4,
        PuzzlePart::Two => 14,
    };

    for index in 0..buffer.len() {
        // Break if we reach the end of the buffer with the tail
        // of our current marker.
        if index + distinct_character_count > buffer.len() {
            break;
        }

        let mut markers = buffer
            .get(index..index + distinct_character_count)
            .unwrap()
            .to_vec();
        if all_distinct(&mut markers) {
            println!(
                "{} characters had to be processed.",
                index + distinct_character_count
            );
            break;
        }
    }

    Ok(())
}
