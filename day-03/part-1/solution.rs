use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack {
    compartment_1: String,

    compartment_2: String,

    duplicated_item: char,
}

fn get_duplicated_item_priority(duplicated_item: &char) -> i32 {
    let ascii_value = *duplicated_item as i32;

    match ascii_value {
        // Lowercase
        65..=90  => ascii_value - 38,

        // Uppercase
        97..=122 => ascii_value - 96,

        _       => panic!("Unexpected ascii code!"),
    }
}

fn load_rucksack_data(file_path: String) -> Result<Vec<Rucksack>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut rucksacks = Vec::new();

    for line in reader.lines() {
        let next_line = line.expect("Expected line!");

        let mut compartment_1_set = HashSet::new();
        let mut compartment_2_set = HashSet::new();

        let (compartment_1, compartment_2) = next_line.split_at(next_line.len() / 2);

        for c in compartment_1.chars() {
            compartment_1_set.insert(c);
        }

        for c in compartment_2.chars() {
            compartment_2_set.insert(c);
        }

        let duplicated_item = compartment_1_set.intersection(&compartment_2_set).next()
            .expect("No duplicated item!");

        rucksacks.push(Rucksack{
            compartment_1: compartment_1.to_string(),
            compartment_2: compartment_2.to_string(),
            duplicated_item: duplicated_item.clone(),
        });
    }

    Ok(rucksacks)
}

fn main() {
    let rucksacks = load_rucksack_data("../rucksack_data.txt".to_string())
        .expect("Failed to load rucksack data!");

    let mut priorities = Vec::new();

    for rucksack in rucksacks.iter() {
        let duplicated_score = get_duplicated_item_priority(&rucksack.duplicated_item);
        priorities.push(duplicated_score);
    }

    let sum: i32 = priorities.iter().sum();

    println!("Sum of priorities: {}", sum);
}
