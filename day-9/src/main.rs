use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const PUZZLE_PART: PuzzlePart = PuzzlePart::Two;
const ROPE_LENGTH: usize = match PUZZLE_PART {
    PuzzlePart::One => 2,
    PuzzlePart::Two => 10,
};

#[derive(PartialEq, Eq)]
enum PuzzlePart {
    One,
    Two,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Unknown direction found."),
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    count: usize,
}

#[derive(PartialEq, Eq, Clone)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new() -> Self {
        Knot { x: 0, y: 0 }
    }
    fn move_as_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
    }

    fn move_as_tail(
        &mut self,
        head: &Knot,
        index: usize,
        tail_positions: &mut Vec<(usize, usize)>,
    ) {
        let (x_offset, y_offset) = match (head.x < self.x, head.y < self.y) {
            (true, true) => (1, 1),
            (true, false) => (1, -1),
            (false, true) => (-1, 1),
            (false, false) => (-1, -1),
        };
        // Tail touches head.
        if self.x.abs_diff(head.x) < 2 && self.y.abs_diff(head.y) < 2 {
        } else if self.x == head.x {
            self.y = head.y + y_offset;
        } else if self.y == head.y {
            self.x = head.x + x_offset;
        } else {
            // Diagonal move.
            if self.x.abs_diff(head.x) == 2 {
                self.x = head.x + x_offset;
                self.y = head.y;
            } else {
                self.x = head.x;
                self.y = head.y + y_offset;
            }
        }
        if index == ROPE_LENGTH - 1 {
            tail_positions.push((self.y as usize, self.x as usize));
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut head_moves = vec![];
    let mut tail_positions = vec![];

    for line in reader.lines().flatten() {
        if line.is_empty() {
            continue;
        }
        let head_move = line.split(' ').collect::<Vec<_>>();
        head_moves.push(Move {
            direction: head_move[0].into(),
            count: head_move[1].parse::<usize>().unwrap(),
        });
    }

    let mut rope = vec![Knot::new(); ROPE_LENGTH];

    for head_move in &head_moves {
        for _i in 0..head_move.count {
            for index in 0..rope.len() {
                if index == 0 {
                    rope[index].move_as_head(&head_move.direction);
                    continue;
                }
                let current_head = rope[index - 1].clone();
                rope[index].move_as_tail(&current_head, index, &mut tail_positions);
            }
        }
    }

    tail_positions.sort();
    tail_positions.dedup();

    println!("The tail had {} positions.", tail_positions.len());

    Ok(())
}
