use std::fs;

fn check_report(levels: &Vec<i32>, skip: Option<usize>) -> bool {
    let mut direction = 0;

    for (index, level) in levels.iter().enumerate() {
        if skip.is_some() && index == skip.unwrap() {
            continue;
        }

        let next_level_index = index + 1;
        let next_level = levels.get(
            if skip.is_some() && next_level_index == skip.unwrap() { next_level_index + 1 } else { next_level_index });

        match next_level {
            None => {
                return true;
            },
            Some(value) => {
                let difference = level - value;

                if difference == 0 || difference.abs() >= 4 {
                    return false;
                }

                if direction == 0 {
                    direction = difference / difference.abs();
                    continue;
                }

                match difference {
                    ..0 if direction != -1 => return false,
                    0.. if direction != 1 => return false,
                    _ => continue,
                }
            }
        }
    }

    return true;
}

pub fn implementation() {
    let input = fs::read_to_string("src/day_two/input").expect("Should read full input file");

    let reports = input.lines();

    let mut one_star = 0;
    let mut failed_reports: Vec<&str> = Vec::new();

    for report in reports.clone() {
        let levels: Vec<i32> = report
            .split(" ")
            .map(|level| level.parse::<i32>().expect("Level should be number"))
            .collect();

        if check_report(&levels, None) {
            one_star += 1
        } else {
            failed_reports.push(report);
        }
    }

    println!("One Star: {one_star:?}");

    let mut two_star = 0;

    for report in failed_reports {
        let levels: Vec<i32> = report
            .split(" ")
            .map(|level| level.parse::<i32>().expect("Level should be number"))
            .collect();

        let levels_count = levels.len();

        let mut skip = 0;

        while skip < levels_count {
            if check_report(&levels, Some(skip)) {
                two_star += 1;
                break;
            }

            skip += 1
        }  
    }

    println!("Two Star: {:?}", one_star + two_star);
}