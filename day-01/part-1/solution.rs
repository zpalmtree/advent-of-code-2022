use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

#[derive(Debug)]
struct Elf {
    items: Vec<i32>,
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
                items: elf_items.clone(),
            });

            elf_items = Vec::new();

            continue;
        }

        elf_items.push(next_line.parse::<i32>().unwrap());
    }

    if elf_items.len() != 0 {
        elves.push(Elf{
            items: elf_items,
        });
    }

    Ok(elves)
}

fn main() {
    let elves = load_calorie_count("../calorie_counts.txt".to_string())
        .expect("Failed to load strategy guide");

    let mut max_count = 0;

    for elf in elves.iter() {
        let calories = elf.items.iter().sum();

        if calories > max_count {
            max_count = calories;
        }
    }

    println!("{:?}", max_count);
}
