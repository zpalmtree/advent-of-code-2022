use std::collections::HashSet;
use std::collections::HashMap;

struct ElfGroup {
    group_badge: char,
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

pub fn solution(input: Vec<String>) -> String {
    let mut groups = Vec::new();
    let mut elf_group = Vec::new();
    let mut parsed_group = Vec::new();

    for next_line in input.iter() {
        elf_group.push(next_line);

        if elf_group.len() == 3 {
            groups.push(elf_group.clone());
            elf_group = Vec::new();
        }
    }

    'outer : for group in groups.iter() {
        /* Convert to set so there is only one of each char in the sack and
         * we don't count an item in the same rucksack twice */
        let rucksacks: [HashSet<char>; 3] = [
            group[0].chars().into_iter().collect(),
            group[1].chars().into_iter().collect(),
            group[2].chars().into_iter().collect(),
        ];

        let mut seen_map = HashMap::new();

        for rucksack in rucksacks {
            for c in rucksack {
                match seen_map.get(&c) {
                    Some(&number) => seen_map.insert(c, number + 1),
                    _ => seen_map.insert(c, 1),
                };
            }
        }

        for (item, seen_count) in seen_map.iter() {
            if *seen_count == 3 {
                parsed_group.push(ElfGroup{
                    group_badge: *item,
                });

                continue 'outer;
            }
        }

        panic!("Failed to find group badge!");
    }

    let mut priorities = Vec::new();

    for group in parsed_group.iter() {
        let duplicated_score = get_duplicated_item_priority(&group.group_badge);
        priorities.push(duplicated_score);
    }

    let sum: i32 = priorities.iter().sum();

    return sum.to_string();
}
