use std::fs;

pub fn implementation() {
    println!("Day 1");

    let input = fs::read_to_string("src/day_one/input").expect("Should read full input file");

    let lines: Vec<&str> =  input.lines().collect();

    let mut left_numbers: Vec<i32> = Vec::with_capacity(lines.len());
    let mut right_numbers: Vec<i32> = Vec::with_capacity(lines.len());

    for line in lines {
        let mut line_split = line.split("   ");

        let first_number = line_split.next().expect("Should be first number in line split");
        let second_number = line_split.next().expect("Should be second number in line split");

        left_numbers.push(first_number.parse::<i32>().expect("Should be an integer"));
        right_numbers.push(second_number.parse::<i32>().expect("Should be an integer"));
    }

    left_numbers.sort();
    right_numbers.sort();

    let one_star = left_numbers
        .clone()
        .into_iter()
        .enumerate()
        .fold(0, |state,(index, number)| state + (number - right_numbers.get(index).unwrap()).abs());

    println!("One Star: {one_star:?}");

    let two_star = left_numbers
        .into_iter()
        .fold(0, |state,number| state + right_numbers.clone().into_iter().filter(|v| number == *v).count() as i32 * number);

    println!("Two Star: {two_star:?}");
}