use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_calories_per_elf = vec![];
    let mut calorie_count = 0;

    for line in reader.lines() {
        if let Ok(calorie_value) = line?.parse::<i32>() {
            calorie_count += calorie_value;
        } else {
            // "parse::<i32> throws an error on an empty line,
            // thus signaling the end of an elfs calorie block.
            total_calories_per_elf.push(calorie_count);
            calorie_count = 0;
        }
    }

    // Sort the calorie collection in descending order.
    total_calories_per_elf.sort_by(|a, b| b.cmp(a));

    let top_calorie_count = total_calories_per_elf[0];
    let mut top_three_calorie_count_sum = 0;

    for calorie_value in total_calories_per_elf[0..3].iter() {
        top_three_calorie_count_sum += calorie_value;
    }

    println!(
        "The elf with the most calories carries {} calories.",
        top_calorie_count
    );
    println!(
        "The three elfs with the most calories carry {} calories together.",
        top_three_calorie_count_sum
    );

    Ok(())
}
