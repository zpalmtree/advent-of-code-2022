fn parse_map(data: Vec<String>) -> Vec<Vec<u32>> {
    let mut map = Vec::new();
    let mut i = 0;

    for line in &data {
        match map.get(i) {
            None => {
                map.push(Vec::new());
            },
            _ => (),
        }

        let current_row = map.get_mut(i).unwrap();

        for c in line.chars() {
            current_row.push(c.to_digit(10).unwrap());
        }

        i += 1;
    }

    return map;
}

fn trees_visible_in_direction(map: &Vec<Vec<u32>>, tree_height: u32, row: usize, column: usize, row_offset: isize, column_offset: isize) -> u32 {
    /* Reached the end without finding a tree blocking the view, it's visible */
    if row == 0 && row_offset < 0 || column == 0 && column_offset < 0 {
        return 0;
    }

    let next_row = (row as isize + row_offset) as usize;
    let next_column = (column as isize + column_offset) as usize;

    if map.get(next_row).is_none() || map[next_row].get(next_column).is_none() {
        return 0;
    }

    let next_tree_height = map[next_row][next_column];

    /* This tree is equal or taller than the queried tree. So it is not visible. */
    if next_tree_height >= tree_height {
        return 1;
    }

    /* This tree does not block the visiblity, lets continue along the path and check the next tree
     * in line */
    return 1 + trees_visible_in_direction(map, tree_height, next_row, next_column, row_offset, column_offset);
}

fn trees_visible_in_top(map: &Vec<Vec<u32>>, tree_height: u32, row: usize, column: usize) -> u32 {
    return trees_visible_in_direction(map, tree_height, row, column, -1, 0);
}

fn trees_visible_in_bottom(map: &Vec<Vec<u32>>, tree_height: u32, row: usize, column: usize) -> u32 {
    return trees_visible_in_direction(map, tree_height, row, column, 1, 0);
}

fn trees_visible_in_left(map: &Vec<Vec<u32>>, tree_height: u32, row: usize, column: usize) -> u32 {
    return trees_visible_in_direction(map, tree_height, row, column, 0, -1);
}

fn trees_visible_in_right(map: &Vec<Vec<u32>>, tree_height: u32, row: usize, column: usize) -> u32 {
    return trees_visible_in_direction(map, tree_height, row, column, 0, 1);
}

fn get_scenic_score(map: &Vec<Vec<u32>>, row: usize, column: usize) -> u32 {
    let tree = map[row][column];

    let top_score = trees_visible_in_top(map, tree, row, column);
    let bottom_score = trees_visible_in_bottom(map, tree, row, column);
    let left_score = trees_visible_in_left(map, tree, row, column);
    let right_score = trees_visible_in_right(map, tree, row, column);

    return top_score * bottom_score * left_score * right_score;
}

fn build_scenic_map(map: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut scenic_map = Vec::new();
    let mut i = 0;

    for column in &map {
        match scenic_map.get(i) {
            None => {
                scenic_map.push(Vec::new());
            },
            _ => (),
        }

        let current_row = scenic_map.get_mut(i).unwrap();

        let mut j = 0;

        for _ in column {
            current_row.push(get_scenic_score(&map, i, j));
            j += 1;
        }

        i += 1;
    }

    return scenic_map;
}

pub fn solution(data: Vec<String>) -> String {
    println!("Successfully loaded {} lines of data.", data.len());

    let map = parse_map(data);
    let scenic_map = build_scenic_map(map);

    let mut best_score = 0;

    for column in &scenic_map {
        for scenic_score in column {
            if *scenic_score > best_score {
                best_score = *scenic_score;
            }
        }
    }

    return best_score.to_string();
}
