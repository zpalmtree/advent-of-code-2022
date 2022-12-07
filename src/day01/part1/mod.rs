#[derive(Debug)]
struct Elf {
    items: Vec<i32>,
}

pub fn solution(input: Vec<String>) -> String {
    let elves = load_calorie_count(input);

    let mut max_count = 0;

    for elf in elves.iter() {
        let calories = elf.items.iter().sum();

        if calories > max_count {
            max_count = calories;
        }
    }

    return max_count.to_string();
}

fn load_calorie_count(input: Vec<String>) -> Vec<Elf> {
    let mut elves = Vec::new();
    let mut elf_items = Vec::new();
    
    for next_line in input.iter() {
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

    return elves;
}
