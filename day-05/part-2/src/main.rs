extern crate regex;
extern crate num;

use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
use regex::Regex;
use std::cmp;
use num::integer::div_ceil;

struct Stack {
    stack_index: usize,

    stack_items: Vec<Crate>,
}

struct Crate {
    crate_item: char,
}

struct CraneInstruction {
    start_stack_index: usize,

    end_stack_index: usize,

    crate_count: usize,
}

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

fn parse_crane_instruction(instruction: String) -> CraneInstruction {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let captures = re.captures(&instruction).unwrap();

    let crate_count = captures.get(1).unwrap().as_str().parse().unwrap();
    let start_stack_index = captures.get(2).unwrap().as_str().parse().unwrap();
    let end_stack_index = captures.get(3).unwrap().as_str().parse().unwrap();

    return CraneInstruction {
        start_stack_index: start_stack_index,
        end_stack_index: end_stack_index,
        crate_count: crate_count,
    }
}

fn print_stack(stacks: &Vec<Stack>) {
    for (i, stack) in stacks.iter().enumerate() {
        print!("Stack {}: ", i + 1);

        for item in stack.stack_items.iter() {
            print!("[{}] ", item.crate_item);
        }

        println!("");
    }
}

fn print_top_of_stack(stacks: &Vec<Stack>) {
    for stack in stacks {
        print!("{}", stack.stack_items.last().unwrap().crate_item);
    }

    println!("");
}

fn parse_starting_stack(starting_stack: &[String]) -> Vec<Stack> {
    let mut stacks = Vec::new();

    let mut stack_count = 0;

    for stack_slice in starting_stack {
        stack_count = cmp::max(stack_count, div_ceil(stack_slice.len(), 4));
    }

    for i in 0..stack_count {
        let mut stack_items = Vec::new();

        for stack_slice in starting_stack.iter().rev() {
            let crate_item = stack_slice.as_bytes()[i*4 + 1] as char;

            if crate_item != ' ' {
                stack_items.push(Crate{
                    crate_item: crate_item,
                });
            }
        }
        
        stacks.push(Stack{
            stack_index: i + 1,
            stack_items: stack_items,
        });
    }
    
    return stacks;
}

fn execute_crane_instruction(instruction: &CraneInstruction, mut stack: Vec<Stack>) -> Vec<Stack> {
    let left_index = cmp::min(instruction.start_stack_index - 1, instruction.end_stack_index - 1);
    let right_index = cmp::max(instruction.start_stack_index - 1, instruction.end_stack_index - 1);

    let (left_stacks, right_stacks) = stack.split_at_mut(right_index);

    let left_stack = &mut left_stacks[left_index];
    let right_stack = &mut right_stacks[0];

    let mut remove_stack = &mut left_stack.stack_items;
    let mut add_stack = &mut right_stack.stack_items;

    if instruction.start_stack_index - 1 == right_index {
        remove_stack = &mut right_stack.stack_items;
        add_stack = &mut left_stack.stack_items;
    }

    let mut crate_stack = remove_stack.drain(remove_stack.len() - instruction.crate_count..).collect();
    add_stack.append(&mut crate_stack);

    return stack;
}

fn main() {
    let data = load_data("../../data.txt".to_string())
        .expect("Failed to load data!");

    println!("Successfully loaded {} lines of data.", data.len());

    let mut starting_stacks = Vec::new();
    let mut crane_instructions = Vec::new();

    let mut load_starting_stacks = true;

    for line in data {
        if line == "" {
            load_starting_stacks = false;
            continue;
        }

        if load_starting_stacks {
            starting_stacks.push(line);
        } else {
            crane_instructions.push(parse_crane_instruction(line));
        }
    }

    println!("Instructions: {}", crane_instructions.len());

    let starting_stack = parse_starting_stack(&starting_stacks[0..starting_stacks.len() - 1]);

    println!("Starting stack:");
    print_stack(&starting_stack);

    let mut intermediate_stack = starting_stack;

    for instruction in crane_instructions.iter() {
        intermediate_stack = execute_crane_instruction(instruction, intermediate_stack);
    }

    println!("\nEnding stack:");
    print_stack(&intermediate_stack);

    println!("\nTop of stacks:");
    print_top_of_stack(&intermediate_stack);
}
