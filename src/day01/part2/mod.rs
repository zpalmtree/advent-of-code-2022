#[derive(Debug)]
struct Elf {
    total_calories: i32,
}

fn load_calorie_count(input: Vec<String>) -> Vec<Elf> {
    let mut elves = Vec::new();
    let mut elf_items = Vec::new();

    for next_line in input.iter() {
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

    return elves;
}

pub fn solution(input: Vec<String>) -> String {
    let mut elves = load_calorie_count(input);

    elves.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));

    let mut calorie_count = 0;

    for i in 0..3 {
        let item = &elves[i];
        calorie_count += item.total_calories;
    }

    return calorie_count.to_string();
}
