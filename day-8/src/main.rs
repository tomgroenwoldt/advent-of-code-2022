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

#[derive(Debug)]
struct TreeVisibility {
    left: bool,
    right: bool,
    top: bool,
    down: bool,
}

impl TreeVisibility {
    fn is_visible(&self) -> bool {
        matches!(
            self,
            TreeVisibility {
                left: false,
                right: false,
                top: false,
                down: false,
            }
        )
    }
}

#[derive(Debug)]
struct ScenicScore {
    left: usize,
    right: usize,
    top: usize,
    down: usize,
}

impl ScenicScore {
    fn get_result(&self) -> usize {
        self.left * self.right * self.top * self.down
    }
}

fn is_visible(i: usize, j: usize, tree_height: &Vec<Vec<u32>>) -> bool {
    // Check right visibility.
    let mut tree_visibility = TreeVisibility {
        left: true,
        right: true,
        top: true,
        down: true,
    };
    for k in j + 1..tree_height.len() {
        if tree_height[i][j] <= tree_height[i][k] {
            tree_visibility.right = false;
            break;
        }
    }
    for k in 0..j {
        if tree_height[i][j] <= tree_height[i][k] {
            tree_visibility.left = false;
            break;
        }
    }
    for k in i + 1..tree_height.len() {
        if tree_height[i][j] <= tree_height[k][j] {
            tree_visibility.down = false;
            break;
        }
    }
    for k in 0..i {
        if tree_height[i][j] <= tree_height[k][j] {
            tree_visibility.top = false;
            break;
        }
    }
    tree_visibility.is_visible()
}

fn get_scenic_score(i: usize, j: usize, tree_height: &Vec<Vec<u32>>) -> usize {
    // Check right visibility.
    let mut scenic_score = ScenicScore {
        left: 0,
        right: 0,
        top: 0,
        down: 0,
    };
    for k in j + 1..tree_height.len() {
        scenic_score.right += 1;
        if tree_height[i][j] <= tree_height[i][k] {
            break;
        }
    }
    for k in (0..j).rev() {
        scenic_score.left += 1;
        if tree_height[i][j] <= tree_height[i][k] {
            break;
        }
    }
    for k in i + 1..tree_height.len() {
        scenic_score.down += 1;
        if tree_height[i][j] <= tree_height[k][j] {
            break;
        }
    }
    for k in (0..i).rev() {
        scenic_score.top += 1;
        if tree_height[i][j] <= tree_height[k][j] {
            break;
        }
    }
    scenic_score.get_result()
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut tree_heights = vec![];

    for line in reader.lines().flatten() {
        if line.is_empty() {
            continue;
        }
        let mut row = vec![];
        for char in line.chars() {
            row.push(char.to_digit(10).unwrap());
        }
        tree_heights.push(row);
    }

    let mut visible_trees = vec![];
    let mut scenic_scores = vec![];

    for i in 1..tree_heights.len() - 1 {
        for j in 1..tree_heights.len() - 1 {
            match PUZZLE_PART {
                PuzzlePart::One => {
                    if is_visible(i, j, &tree_heights) {
                        visible_trees.push(tree_heights[i][j]);
                    }
                }
                PuzzlePart::Two => {
                    scenic_scores.push(get_scenic_score(i, j, &tree_heights));
                }
            }
        }
    }

    let number_of_visible_trees = visible_trees.len() + tree_heights.len() * 4 - 4;
    println!(
        "The number of visible trees is {}.",
        number_of_visible_trees
    );

    scenic_scores.sort();
    println!(
        "The maximum scenic score is {}",
        scenic_scores[scenic_scores.len() - 1]
    );

    Ok(())
}
