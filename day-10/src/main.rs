use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Duration,
};

#[derive(PartialEq, Eq, Debug)]
struct Operation {
    addx: Option<i32>,
}

struct Crt {
    display: Vec<Vec<char>>,
}

impl Crt {
    fn new() -> Self {
        Crt {
            display: vec![vec!['.'; 40]; 6],
        }
    }
    fn print(&self, register_value: i32, cycle_count: i32) {
        // Clear terminal.
        print!("{}[2J", 27 as char);
        (0..40).for_each(|index| {
            if index == (cycle_count - 1) % 40 {
                print!("v");
            } else {
                print!(" ");
            }
        });
        println!();
        (0..40).for_each(|index| {
            if ((register_value - 1)..=(register_value + 1)).contains(&index) {
                print!("#");
            } else {
                print!(".");
            }
        });
        (0..2).for_each(|_| println!());
        self.display.iter().for_each(|row| {
            row.iter().for_each(|pixel| print!("{}", pixel));
            println!();
        });
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut operations = vec![];
    let mut register_value: i32 = 1;
    let mut cycle_count = 0;

    for line in reader.lines().flatten() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("noop") {
            operations.push(Operation { addx: None });
        } else {
            operations.push(Operation {
                addx: Some(
                    line.split(' ').collect::<Vec<_>>()[1]
                        .parse::<i32>()
                        .unwrap(),
                ),
            })
        }
    }

    let mut signal_strenghs = vec![];
    let mut crt = Crt::new();
    crt.print(register_value, cycle_count);

    for operation in operations {
        match operation {
            Operation { addx: None } => {
                cycle_count += 1;
                if (cycle_count % 40) - 20 == 0 {
                    signal_strenghs.push(cycle_count * register_value);
                }
                if (register_value - 1..=register_value + 1).contains(&((cycle_count - 1) % 40)) {
                    crt.display[(cycle_count / 40) as usize][((cycle_count - 1) % 40) as usize] =
                        '#';
                }
                crt.print(register_value, cycle_count);
            }
            Operation { addx: Some(value) } => {
                for _ in 0..2 {
                    cycle_count += 1;
                    if (register_value - 1..=register_value + 1).contains(&((cycle_count - 1) % 40))
                    {
                        crt.display[(cycle_count / 40) as usize]
                            [((cycle_count - 1) % 40) as usize] = '#';
                    }
                    if (cycle_count % 40) - 20 == 0 {
                        signal_strenghs.push(cycle_count * register_value);
                    }
                    crt.print(register_value, cycle_count);
                }
                register_value += value;
            }
        }
    }

    println!(
        "The sum of the signal strenghts is {}.",
        signal_strenghs.iter().sum::<i32>()
    );
    Ok(())
}
