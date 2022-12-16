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

#[derive(Debug, Clone)]
struct Square {
    x: usize,
    y: usize,
    value: char,
}

fn get_maximum_smaller_neighbor(current_square: &mut Square, squares: Vec<Vec<Square>>) -> Square {
    let mut neighbors = vec![];

    if current_square.y > 0 {
        neighbors.push(&squares[current_square.y][current_square.x]);
    }
    if current_square.y < squares.len() - 2 {
        neighbors.push(&squares[current_square.y][current_square.x]);
    }
    if current_square.x > 0 {
        neighbors.push(&squares[current_square.y][current_square.x]);
    }
    if current_square.x < squares[current_square.y].len() - 2 {
        neighbors.push(&squares[current_square.y][current_square.x]);
    }

    neighbors
        .iter()
        .filter(|square| (square.value as u32) > (current_square.value as u32 - 2))
}

fn main() -> std::io::Result<()> {
    let file = File::open("input2.txt")?;
    let reader = BufReader::new(file);

    let mut squares = vec![];

    for (y, line) in reader.lines().flatten().enumerate() {
        if line.is_empty() {
            continue;
        }
        squares.push(
            line.chars()
                .enumerate()
                .map(|(x, value)| Square { x, y, value })
                .collect::<Vec<_>>(),
        );
    }

    let squares_clone = squares.clone();

    let mut start = (0, 0);
    let mut destination = (0, 0);

    // let mut path = vec![];

    println!("path: {:?}", squares);

    Ok(())
}
