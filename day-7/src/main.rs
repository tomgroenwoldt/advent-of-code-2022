use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const PUZZLE_PART: PuzzlePart = PuzzlePart::One;
const MAXIMUM_DIRECTORY_SIZE: usize = 100000;
const TOTAL_DISK_SPACE: usize = 70000000;
const SPACE_NEEDED_FOR_UPDATE: usize = 30000000;

#[derive(PartialEq, Eq)]
enum PuzzlePart {
    One,
    Two,
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<MyFile>,
}

#[derive(Debug)]
struct MyFile {
    size: usize,
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut old_directories = vec![];
    let mut active_directories = vec![];

    for line in reader.lines().flatten() {
        if line.starts_with("$ cd") {
            // Exits last added directory, thus remove this from the active directories.
            if line.ends_with("..") {
                old_directories.push(active_directories.pop().unwrap());
            } else {
                // Otherwise push the new directory to active ones.
                active_directories.push(Directory {
                    name: line.split(' ').collect::<Vec<_>>()[2].to_string(),
                    files: vec![],
                })
            }
        }
        // Add found file to all active directories.
        if line.chars().next().unwrap().is_numeric() {
            active_directories.iter_mut().for_each(|directory| {
                let my_file = line.split(' ').collect::<Vec<_>>();
                directory.files.push(MyFile {
                    size: my_file[0].parse::<usize>().unwrap(),
                });
            });
        }
    }

    // Merge all directories together, root and last directory aren't quit by "cd ..".
    old_directories.append(&mut active_directories);

    match PUZZLE_PART {
        PuzzlePart::One => {
            let size_of_small_directories = old_directories
                .into_iter()
                .filter(|directory| {
                    directory.files.iter().map(|file| file.size).sum::<usize>()
                        <= MAXIMUM_DIRECTORY_SIZE
                })
                .map(|directory| directory.files.iter().map(|file| file.size).sum::<usize>())
                .sum::<usize>();

            println!(
                "The sum of the total sizes of small directories is {}",
                size_of_small_directories
            );
        }
        PuzzlePart::Two => {
            let mut sorted_directories = old_directories
                .into_iter()
                .map(|directory| {
                    (
                        directory.name,
                        directory.files.iter().map(|file| file.size).sum::<usize>(),
                    )
                })
                .collect::<Vec<_>>();
            sorted_directories.sort_by(|a, b| a.1.cmp(&b.1));
            let available_space = TOTAL_DISK_SPACE - sorted_directories.last().unwrap().1;
            let update_fix_directory = sorted_directories
                .iter()
                .find(|directory| available_space + directory.1 >= SPACE_NEEDED_FOR_UPDATE)
                .unwrap();
            println!("{:?}", update_fix_directory);
        }
    }
    Ok(())
}
