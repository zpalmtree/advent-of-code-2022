use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
use std::collections::VecDeque;
use std::collections::HashSet;

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

fn find_start_of_packet_marker(data: &str) -> Option<usize> {
    let mut deque: VecDeque<char> = VecDeque::new();

    let mut i = 0;

    for c in data.chars() {
        i += 1;

        deque.push_back(c);

        if deque.len() > 14 {
            deque.pop_front();

            let mut set = HashSet::new();

            for prev_char in deque.iter() {
                set.insert(prev_char);
            }

            // Every previous char was unique
            if set.len() == 14 {
                return Some(i);
            }
        }
    }

    return None;
}

fn main() {
    let data = load_data("../data.txt".to_string())
        .expect("Failed to load data!");

    println!("Successfully loaded {} lines of data.", data.len());

    let index = find_start_of_packet_marker(&data[0])
        .expect("Expected to find a packet marker!");

    println!("Start of packet marker index: {}", index);
}
