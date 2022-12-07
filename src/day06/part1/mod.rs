use std::collections::VecDeque;
use std::collections::HashSet;

fn find_start_of_packet_marker(data: &str) -> Option<usize> {
    let mut deque: VecDeque<char> = VecDeque::new();

    let mut i = 0;

    for c in data.chars() {
        i += 1;

        deque.push_back(c);

        if deque.len() > 4 {
            deque.pop_front();

            let mut set = HashSet::new();

            for prev_char in deque.iter() {
                set.insert(prev_char);
            }

            // Every previous char was unique
            if set.len() == 4 {
                return Some(i);
            }
        }
    }

    return None;
}

pub fn solution(data: Vec<String>) -> String {
    println!("Successfully loaded {} lines of data.", data.len());

    let index = find_start_of_packet_marker(&data[0])
        .expect("Expected to find a packet marker!");

    return index.to_string();
}
