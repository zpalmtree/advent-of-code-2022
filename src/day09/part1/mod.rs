use std::collections::HashSet;

#[derive(Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn parse_direction(direction: char) -> Direction {
    match direction {
        'U' => Direction::North,
        'D' => Direction::South,
        'R' => Direction::East,
        'L' => Direction::West,
        _   => panic!("Unexpected direction!"),
    }
}

fn parse_directions(data: Vec<String>) -> Vec<Direction> {
    let mut directions = Vec::new();

    for line in data {
        let mut split = line.split(" ");

        let direction = parse_direction(split.next().unwrap().as_bytes()[0] as char);
        let count = split.next().unwrap().parse().unwrap();

        for _ in 0..count {
            directions.push(direction.clone());
        }
    }

    return directions;
}

fn move_x(x: i32, direction: &Direction) -> i32 {
    match direction {
        Direction::East => x + 1,
        Direction::West => x - 1,
        _ => x,
    }
}

fn move_y(y: i32, direction: &Direction) -> i32 {
    match direction {
        Direction::North => y + 1,
        Direction::South => y - 1,
        _ => y,
    }
}

fn move_head(head: &Coordinate, direction: &Direction) -> Coordinate {
    let new_x = move_x(head.x, direction);
    let new_y = move_y(head.y, direction);

    return Coordinate{
        x: new_x,
        y: new_y,
    }
}

fn move_tail(head: &Coordinate, tail: &Coordinate) -> Coordinate {
    let x_diff = (head.x - tail.x).abs();
    let y_diff = (head.y - tail.y).abs();

    let out_of_phase = x_diff + y_diff > 2;

    let new_x = if x_diff > 1 || out_of_phase {
        let direction = if head.x > tail.x {
            Direction::East
        } else {
            Direction::West
        };

        move_x(tail.x, &direction)
    } else {
        tail.x
    };

    let new_y = if y_diff > 1 || out_of_phase {
        let direction = if head.y > tail.y {
            Direction::North
        } else {
            Direction::South
        };

        move_y(tail.y, &direction)
    } else {
        tail.y
    };

    return Coordinate{
        x: new_x,
        y: new_y,
    }
}

pub fn solution(data: Vec<String>) -> String {
    println!("Successfully loaded {} lines of data.", data.len());

    let directions = parse_directions(data);

    let mut head_positions = Vec::new();
    let mut tail_positions = Vec::new();

    let mut head = Coordinate{
        x: 0,
        y: 0,
    };

    let mut tail = Coordinate{
        x: 0,
        y: 0,
    };
    
    head_positions.push(head.clone());
    tail_positions.push(tail.clone());

    for direction in directions {
        head = move_head(&head, &direction);
        tail = move_tail(&head, &tail);

        //println!("Head: {:?}, tail: {:?}", head, tail);

        head_positions.push(head.clone());
        tail_positions.push(tail.clone());
    }

    let mut unique_positions = HashSet::new();

    for position in tail_positions {
        unique_positions.insert(position);
    }

    return unique_positions.len().to_string();
}
