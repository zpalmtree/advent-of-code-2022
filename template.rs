use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

fn load_data(file_path: String) -> Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        let next_line = line.expect("Expected line!");
        lines.push(next_line);
    }

    Ok(lines)
}

fn main() {
    let data = load_data("../data.txt".to_string())
        .expect("Failed to load data!");

    println!("Successfully loaded {} lines of data.", data.len());
}
