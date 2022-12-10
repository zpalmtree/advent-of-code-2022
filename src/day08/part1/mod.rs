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

fn is_visible_from_direction(map: &Vec<Vec<u32>>, tree_height: u32, row: usize, column: usize, row_offset: isize, column_offset: isize) -> bool {
    /* Reached the end without finding a tree blocking the view, it's visible */
    if row == 0 && row_offset < 0 || column == 0 && column_offset < 0 {
        return true;
    }

    let next_row = (row as isize + row_offset) as usize;
    let next_column = (column as isize + column_offset) as usize;

    if map.get(next_row).is_none() || map[next_row].get(next_column).is_none() {
        return true;
    }

    let next_tree_height = map[next_row][next_column];

    /* This tree is equal or taller than the queried tree. So it is not visible. */
    if next_tree_height >= tree_height {
        return false;
    }

    /* This tree does not block the visiblity, lets continue along the path and check the next tree
     * in line */
    return is_visible_from_direction(map, tree_height, next_row, next_column, row_offset, column_offset);
}

fn is_visible_from_top(map: &Vec<Vec<u32>>, tree_height: u32, row: usize, column: usize) -> bool {
    return is_visible_from_direction(map, tree_height, row, column, -1, 0);
}

fn is_visible_from_bottom(map: &Vec<Vec<u32>>, tree_height: u32, row: usize, column: usize) -> bool {
    return is_visible_from_direction(map, tree_height, row, column, 1, 0);
}

fn is_visible_from_left(map: &Vec<Vec<u32>>, tree_height: u32, row: usize, column: usize) -> bool {
    return is_visible_from_direction(map, tree_height, row, column, 0, -1);
}

fn is_visible_from_right(map: &Vec<Vec<u32>>, tree_height: u32, row: usize, column: usize) -> bool {
    return is_visible_from_direction(map, tree_height, row, column, 0, 1);
}

fn is_tree_visible(map: &Vec<Vec<u32>>, row: usize, column: usize) -> bool {
    let tree = map[row][column];

    if is_visible_from_top(map, tree, row, column) {
        return true;
    }

    if is_visible_from_bottom(map, tree, row, column) {
        return true;
    }

    if is_visible_from_left(map, tree, row, column) {
        return true;
    }

    if is_visible_from_right(map, tree, row, column) {
        return true;
    }

    return false;
}

fn build_visible_map(map: Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let mut visible_map = Vec::new();
    let mut i = 0;

    for column in &map {
        match visible_map.get(i) {
            None => {
                visible_map.push(Vec::new());
            },
            _ => (),
        }

        let current_row = visible_map.get_mut(i).unwrap();

        let mut j = 0;

        for _ in column {
            current_row.push(is_tree_visible(&map, i, j));
            j += 1;
        }

        i += 1;
    }

    return visible_map;
}

pub fn solution(data: Vec<String>) -> String {
    println!("Successfully loaded {} lines of data.", data.len());

    let map = parse_map(data);
    let visible_map = build_visible_map(map);

    let mut total_visible = 0;

    for column in &visible_map {
        for tree in column {
            if *tree {
                total_visible += 1;
            }
        }
    }

    return total_visible.to_string();
}
