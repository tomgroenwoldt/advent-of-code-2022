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

#[derive(Debug, Clone)]
enum Handshape {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Handshape {
    fn from(str: &str) -> Self {
        match str {
            "A" => Handshape::Rock,
            "B" => Handshape::Paper,
            "C" => Handshape::Scissors,
            "X" => Handshape::Rock,
            "Y" => Handshape::Paper,
            "Z" => Handshape::Scissors,
            _ => panic!("Unknown handshape occured."),
        }
    }
}

impl Handshape {
    fn shape_score(&self) -> i32 {
        match self {
            Handshape::Rock => 1,
            Handshape::Paper => 2,
            Handshape::Scissors => 3,
        }
    }
}

#[derive(Debug)]
pub enum RoundOutcome {
    Win,
    Loss,
    Draw,
}

impl From<&str> for RoundOutcome {
    fn from(str: &str) -> Self {
        match str {
            "X" => RoundOutcome::Loss,
            "Y" => RoundOutcome::Draw,
            "Z" => RoundOutcome::Win,
            _ => panic!("Error to parse RoundOutcome from string {:?}", str),
        }
    }
}

#[derive(Debug)]
struct GameRound {
    pub opponent: Handshape,
    pub me: Option<Handshape>,
    pub round_outcome: Option<RoundOutcome>,
}

impl From<Vec<&str>> for GameRound {
    fn from(vec: Vec<&str>) -> Self {
        match PUZZLE_PART {
            PuzzlePart::One => GameRound {
                opponent: vec[0].into(),
                me: Some(vec[1].into()),
                round_outcome: None,
            },
            PuzzlePart::Two => GameRound {
                opponent: vec[0].into(),
                me: None,
                round_outcome: Some(vec[1].into()),
            },
        }
    }
}

impl GameRound {
    pub fn calculate_score(&self) -> i32 {
        let mut shape_score = 0;
        if let Some(hand_shape) = &self.me {
            shape_score = hand_shape.shape_score();
        }

        let round_score = match self {
            GameRound {
                opponent: Handshape::Rock,
                me: Some(Handshape::Paper),
                ..
            } => 6,
            GameRound {
                opponent: Handshape::Paper,
                me: Some(Handshape::Scissors),
                ..
            } => 6,
            GameRound {
                opponent: Handshape::Scissors,
                me: Some(Handshape::Rock),
                ..
            } => 6,
            GameRound {
                opponent: Handshape::Rock,
                me: Some(Handshape::Rock),
                ..
            } => 3,
            GameRound {
                opponent: Handshape::Paper,
                me: Some(Handshape::Paper),
                ..
            } => 3,
            GameRound {
                opponent: Handshape::Scissors,
                me: Some(Handshape::Scissors),
                ..
            } => 3,
            GameRound {
                opponent: Handshape::Rock,
                me: Some(Handshape::Scissors),
                ..
            } => 0,
            GameRound {
                opponent: Handshape::Paper,
                me: Some(Handshape::Rock),
                ..
            } => 0,
            GameRound {
                opponent: Handshape::Scissors,
                me: Some(Handshape::Paper),
                ..
            } => 0,
            _ => 0,
        };

        round_score + shape_score
    }

    fn choose_handshape(&mut self) {
        match self {
            GameRound {
                opponent: Handshape::Rock,
                me: _,
                round_outcome: Some(RoundOutcome::Win),
            } => self.me = Some(Handshape::Paper),
            GameRound {
                opponent: Handshape::Rock,
                me: _,
                round_outcome: Some(RoundOutcome::Loss),
            } => self.me = Some(Handshape::Scissors),
            GameRound {
                opponent: Handshape::Paper,
                me: _,
                round_outcome: Some(RoundOutcome::Win),
            } => self.me = Some(Handshape::Scissors),
            GameRound {
                opponent: Handshape::Paper,
                me: _,
                round_outcome: Some(RoundOutcome::Loss),
            } => self.me = Some(Handshape::Rock),
            GameRound {
                opponent: Handshape::Scissors,
                me: _,
                round_outcome: Some(RoundOutcome::Win),
            } => self.me = Some(Handshape::Rock),
            GameRound {
                opponent: Handshape::Scissors,
                me: _,
                round_outcome: Some(RoundOutcome::Loss),
            } => self.me = Some(Handshape::Paper),
            GameRound {
                opponent: _,
                me: _,
                round_outcome: Some(RoundOutcome::Draw),
            } => self.me = Some(self.opponent.clone()),
            _ => {
                println!("Didn't set my handshape!")
            }
        }
        println!("{:?}", self);
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut game_rounds: Vec<GameRound> = vec![];

    reader.lines().for_each(|line| {
        if let Ok(line) = line {
            if line.is_empty() {
                return;
            }
            game_rounds.push(line.split(' ').collect::<Vec<&str>>().into());
        }
    });

    let total_score: i32 = game_rounds
        .iter_mut()
        .map(|game_round| {
            if PUZZLE_PART == PuzzlePart::Two {
                game_round.choose_handshape();
            }
            game_round.calculate_score()
        })
        .sum();

    println!("The total score would be {}.", total_score);

    Ok(())
}
