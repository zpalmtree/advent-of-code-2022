use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

struct Assignment {
    elf_1_start: u32,
    elf_1_end: u32,

    elf_2_start: u32,
    elf_2_end: u32,
}

fn load_section_assignments(file_path: String) -> Result<Vec<Assignment>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut assignments = Vec::new();

    for line in reader.lines() {
        let next_line = line.expect("Expected line!");

        let mut split = next_line.split(",");

        let mut elf_1_assignment = split.next().unwrap().split("-");
        let mut elf_2_assignment = split.next().unwrap().split("-");

        assignments.push(Assignment{
            elf_1_start: elf_1_assignment.next().unwrap().parse().unwrap(),
            elf_1_end: elf_1_assignment.next().unwrap().parse().unwrap(),
            elf_2_start: elf_2_assignment.next().unwrap().parse().unwrap(),
            elf_2_end: elf_2_assignment.next().unwrap().parse().unwrap(),
        });
    }

    Ok(assignments)
}

fn do_assignments_overlap(assignment: &Assignment) -> bool {
    if assignment.elf_1_start < assignment.elf_2_start && assignment.elf_1_end < assignment.elf_2_start {
        return false;
    }

    if assignment.elf_2_start < assignment.elf_1_start && assignment.elf_2_end < assignment.elf_1_start {
        return false;
    }

    return true;
}

fn main() {
    let assignments = load_section_assignments("../section_assignments.txt".to_string())
        .expect("Failed to load assignment data!");

    let mut contained = Vec::new();

    for assignment in assignments.iter() {
        if do_assignments_overlap(assignment) {
            contained.push(assignment);
        }
    }

    println!("Assignments where ranges overlap: {}", contained.len());
}
