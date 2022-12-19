pub struct Monkey {
    name: String,

    starting_items: Vec<Item>,

    calc_worry_level_after_inspection: Box<dyn Fn(i32) -> i32>,

    monkey_worry_test: Box<dyn Fn(i32) -> bool>,

    monkey_destination_on_test_fail: usize,

    monkey_destination_on_test_success: usize,

    items_seen: i32,
}

#[derive(Debug)]
struct Item {
    worry_level: i32,
}

#[derive(PartialEq)]
enum Operations {
    Multiply,
    Add,
}

fn parse_operation_func(operation_str: &str, operand_str: &str) -> Box<dyn Fn(i32) -> i32> {
    let operation = match operation_str {
        "+" => Operations::Add,
        "*" => Operations::Multiply,
        _ => panic!("Unsupported operation!"),
    };

    if operation == Operations::Multiply {
        if operand_str == "old" {
            return Box::new(move |x| x * x);
        }

        let operand: i32 = operand_str.parse().unwrap();

        return Box::new(move |x| x * operand);
    } else {
        if operand_str == "old" {
            return Box::new(move |x| x + x);
        }

        let operand: i32 = operand_str.parse().unwrap();

        return Box::new(move |x| x + operand);
    }
}

fn parse_monkeys(data: Vec<String>) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    let mut i = 0;

    let starting_items_prefix = "  Starting items: ";
    let operation_prefix = "  Operation: new = old ";
    let test_prefix = "  Test: divisible by ";
    let test_success_prefix = "    If true: throw to monkey ";
    let test_fail_prefix = "    If false: throw to monkey ";

    while i < data.len() {
        let starting_items_str = data.get(i+1).unwrap().strip_prefix(starting_items_prefix).unwrap();
        let operation = data.get(i+2).unwrap().strip_prefix(operation_prefix).unwrap();

        let mut starting_items = Vec::new();

        for item in starting_items_str.split(", ") {
            starting_items.push(Item{
                worry_level: item.parse().unwrap(),
            });
        }

        let test: i32 = data.get(i+3).unwrap()
            .strip_prefix(test_prefix).unwrap()
            .parse().unwrap();

        let mut func_split = operation.split(" ");
        let operation = func_split.next().unwrap();
        let operand = func_split.next().unwrap();

        let test_success = data.get(i+4).unwrap()
            .strip_prefix(test_success_prefix).unwrap()
            .parse().unwrap();

        let test_fail = data.get(i+5).unwrap()
            .strip_prefix(test_fail_prefix).unwrap()
            .parse().unwrap();

        let operation_func = parse_operation_func(operation, operand);

        let monkey = Monkey{
            name: format!("Monkey {}", i / 7),
            starting_items: starting_items,
            calc_worry_level_after_inspection: operation_func,
            monkey_worry_test: Box::new(move |x| x % test == 0),
            monkey_destination_on_test_fail: test_fail,
            monkey_destination_on_test_success: test_success,
            items_seen: 0,
        };

        monkeys.push(monkey);

        i += 7;
    }

    return monkeys;
}

pub fn print_monkeys(monkeys: &Vec<Monkey>) {
    for monkey in monkeys.iter() {
        println!("{}", monkey.name);
        println!("Items seen: {}", monkey.items_seen);

        print!("    Items: ");

        for item in &monkey.starting_items {
            print!("{}, ", item.worry_level);
        }

        println!("");
        println!("");
    }
}

pub fn solution(data: Vec<String>) -> String {
    println!("Successfully loaded {} lines of data.", data.len());

    let mut monkeys = parse_monkeys(data);

    let round_count = 20;

    for i in 0..round_count {
        println!("Round {}", i+1);

        for j in 0..monkeys.len() {
            let monkey = &mut monkeys[j];

            println!("{}:", monkey.name);

            let mut destinations = Vec::new();

            for item in &monkey.starting_items {
                let new_worry_level = (monkey.calc_worry_level_after_inspection)(item.worry_level);
                let final_worry_level = new_worry_level / 3;

                let test_result = (monkey.monkey_worry_test)(final_worry_level);

                let monkey_destination_index = match test_result {
                    true => monkey.monkey_destination_on_test_success,
                    false => monkey.monkey_destination_on_test_fail,
                };

                destinations.push((
                    monkey_destination_index,
                    Item{ worry_level: final_worry_level },
                ));

                println!("  Monkey inspect an item with a worry level of {}.", item.worry_level);
                println!("      Worry level -> {}", new_worry_level);
                println!("      Monkey gets bored with item. Worry level is divided by 3 to {}", final_worry_level);
                
                if test_result {
                    println!("      Current worry level is divisible by ?.");
                    println!("      Item with worry level {} is thrown to monkey {}.", final_worry_level, monkey.monkey_destination_on_test_success);
                } else {
                    println!("      Current worry level is not divisible by ?.");
                    println!("      Item with worry level {} is thrown to monkey {}.", final_worry_level, monkey.monkey_destination_on_test_fail);
                }
            }

            println!("");

            monkey.items_seen += monkey.starting_items.len() as i32;
            monkey.starting_items = Vec::new();

            for (destination, item) in destinations.iter() {
                let monkey_destination = &mut monkeys[*destination];
                monkey_destination.starting_items.push(Item{ worry_level: item.worry_level });
                println!("Pushing item {} to {}", item.worry_level, destination);
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_seen.cmp(&a.items_seen));

    let number_one = &monkeys[0];
    let number_two = &monkeys[1];

    let monkey_business = number_one.items_seen * number_two.items_seen;

    print_monkeys(&monkeys);

    return monkey_business.to_string();
}
