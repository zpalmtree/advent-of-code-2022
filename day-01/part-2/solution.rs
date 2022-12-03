use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

#[derive(Debug)]
struct Elf {
    total_calories: i32,
}

fn load_calorie_count(file_path: String) -> Result<Vec<Elf>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut elves = Vec::new();
    let mut elf_items = Vec::new();

    for line in reader.lines() {
        let next_line = line.expect("Expected line!");

        if next_line == "" {
            elves.push(Elf{
                total_calories: elf_items.iter().sum(),
            });

            elf_items = Vec::new();

            continue;
        }

        elf_items.push(next_line.parse::<i32>().unwrap());
    }

    if elf_items.len() != 0 {
        elves.push(Elf{
            total_calories: elf_items.iter().sum(),
        });
    }

    Ok(elves)
}

fn main() {
    let mut elves = load_calorie_count("../calorie_counts.txt".to_string())
        .expect("Failed to load strategy guide");

    elves.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));

    let mut calorie_count = 0;

    for i in 0..3 {
        let item = &elves[i];
        calorie_count += item.total_calories;
    }

    println!("{:?}", calorie_count);
}
