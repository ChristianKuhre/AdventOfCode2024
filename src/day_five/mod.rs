use std::{cmp::Ordering, collections::HashSet, fs};

pub fn implementation() {
    println!("Day 5");

    let input = fs::read_to_string("src/day_five/input").expect("Should read full input file");

    let mut input_split = input.split("\n\n");

    let mut order_set = HashSet::<&str>::new();

    for line in input_split.nth(0).unwrap().lines() {
        order_set.insert(line);
    }

    let mut one_star = 0;
    let mut two_star = 0;

    'update_loop: for line in input_split.nth(0).unwrap().lines() {
        let mut update: Vec<&str> = line.split(",").collect();

        for (i, instruction) in update.iter().enumerate() {
            let is_not_ordered = update[i..].iter().any(|value| order_set.contains(&format!("{}|{}", value, instruction).as_str()));

            if is_not_ordered {
                update.sort_by(|a, b| if order_set.contains(&format!("{}|{}", a, b).as_str()) { Ordering::Less } else { Ordering::Greater });

                two_star += update.get(update.len() / 2).unwrap().parse::<i32>().unwrap();

                continue 'update_loop;
            }
        }

        one_star += update.get(update.len() / 2).unwrap().parse::<i32>().unwrap();
    }

    println!("One Star: {one_star:?}");
    println!("Two Star: {two_star:?}");
}