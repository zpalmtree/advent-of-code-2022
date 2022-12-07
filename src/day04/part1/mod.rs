struct Assignment {
    elf_1_start: u32,
    elf_1_end: u32,

    elf_2_start: u32,
    elf_2_end: u32,
}

fn do_assignments_overlap(assignment: &Assignment) -> bool {
    if assignment.elf_1_start <= assignment.elf_2_start && assignment.elf_1_end >= assignment.elf_2_end {
        return true;
    }

    if assignment.elf_2_start <= assignment.elf_1_start && assignment.elf_2_end >= assignment.elf_1_end {
        return true;
    }

    return false;
}

pub fn solution(items: Vec<String>) -> String {
    let mut assignments = Vec::new();

    for next_line in items.iter() {
        let mut split = next_line.split(",");

        let first_assignment = split.next()
            .expect("Expected first assignment");

        let second_assignment = split.next()
            .expect("Expected second assignment");

        let mut elf_1_assignment = first_assignment.split("-");
        let mut elf_2_assignment = second_assignment.split("-");

        let elf_1_start: u32 = elf_1_assignment.next()
            .expect("Expected first range")
            .parse()
            .expect("Expected first range");

        let elf_1_end: u32 = elf_1_assignment.next()
            .expect("Expected first range")
            .parse()
            .expect("Expected first range");

        let elf_2_start: u32 = elf_2_assignment.next()
            .expect("Expected second range")
            .parse()
            .expect("Expected second range");

        let elf_2_end: u32 = elf_2_assignment.next()
            .expect("Expected second range")
            .parse()
            .expect("Expected second range");

        assignments.push(Assignment{
            elf_1_start: elf_1_start,
            elf_1_end: elf_1_end,
            elf_2_start: elf_2_start,
            elf_2_end: elf_2_end,
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
