struct Assignment {
    elf_1_start: u32,
    elf_1_end: u32,

    elf_2_start: u32,
    elf_2_end: u32,
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

pub fn solution(input: Vec<String>) -> String {
    let mut assignments = Vec::new();

    for next_line in input.iter() {
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

    let mut contained = Vec::new();

    for assignment in assignments.iter() {
        if do_assignments_overlap(assignment) {
            contained.push(assignment);
        }
    }

    return contained.len().to_string();
}
