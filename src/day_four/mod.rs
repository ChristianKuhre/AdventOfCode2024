use std::fs;

fn get_cell (board: &Vec<&str>, x: i32, y: i32) -> Option<char> {
    match usize::try_from(x) {
        Err(_e) => None,
        Ok(new_x)  => {
            match board.get(usize::from(new_x)) {
                None => None,
                Some(row) => {
                    match usize::try_from(y) {
                        Err(_e) => None,
                        Ok(new_y)  => {
                            match row.chars().nth(new_y) {
                                None => None,
                                Some(value) => Some(value)
                            }
                        }
                    }
                }
            }
        }
    }
}

fn find_one_star_patterns (board: &Vec<&str>, x: i32, y: i32) -> Vec<Vec<(i32, i32)>> {
    let pattern = ['X', 'M', 'A', 'S'];

    let mut patterns = Vec::<Vec<(i32, i32)>>::new();

    for dir_x in -1..=1 { 
        for dir_y in -1..=1 {
            let mut position = (x, y);
            let mut new_pattern = Vec::<(i32, i32)>::new();

            for c in pattern {
                match get_cell(board, position.0, position.1) {
                    None => break,
                    Some(value) if value != c => break,
                    _ => {
                        new_pattern.push(position)
                    }
                }

                position = (position.0 + dir_x, position.1 + dir_y)
            }

            if new_pattern.len() == pattern.len() {
                patterns.push(new_pattern);
            }
        }
    }

    return patterns;
}

fn find_two_star_pattern (board: &Vec<&str>, x: i32, y: i32) -> bool {
    match get_cell(board, x, y) {
        None => return false,
        Some(value) if value != 'A' => return false,
        _ => {
            match (get_cell(board, x - 1, y - 1), get_cell(board, x + 1, y + 1)) {
                (None, _) | (_, None) => return false,
                (Some(first_top), Some(first_bot)) if (first_top == 'M' && first_bot == 'S') || (first_top == 'S' && first_bot == 'M') => {
                    match (get_cell(board, x + 1, y - 1), get_cell(board, x - 1, y + 1)) {
                        (None, _) | (_, None) => return false,
                        (Some(second_top), Some(second_bot)) if (second_top == 'M' && second_bot == 'S') || (second_top == 'S' && second_bot == 'M') => {
                            return true
                        },
                        _ => return false
                    }
                },
                _ => return false,
            }
        }
    }
}

pub fn implementation() {
    let input = fs::read_to_string("src/day_four/input").expect("Should read full input file");

    let board: Vec<&str> = input.lines().collect();

    let mut one_star_patterns = Vec::<Vec<(i32, i32)>>::new();

    for (x, row) in board.iter().enumerate() {
        for (y, _cell) in row.chars().enumerate() {
            let mut new_one_star_patterns = find_one_star_patterns(&board, x as i32, y as i32);
            one_star_patterns.append(&mut new_one_star_patterns);
        }
    }

    println!("One Star: {}", one_star_patterns.len());

    let mut two_star = 0;

    for (x, row) in board.iter().enumerate() {
        for (y, _cell) in row.chars().enumerate() {
            if find_two_star_pattern(&board, x as i32, y as i32) {
                two_star += 1;
            }
        }
    }

    println!("Two Star: {}", two_star);
} 