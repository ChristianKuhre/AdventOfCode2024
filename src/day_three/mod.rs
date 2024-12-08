use std::fs;
use regex::Regex;

fn find_mul_and_calculate (text: &str) -> i32 {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0;
    
    for caps in mul_regex.captures_iter(text) {
        let (_, [first, second]) = caps.extract();
        result += first.parse::<i32>().expect("Should be a number") * second.parse::<i32>().expect("Should be a number")
    }

    return result; 
}

pub fn implementation() {
    println!("Day 3");

    let input = fs::read_to_string("src/day_three/input").expect("Should read full input file");

    let one_star = find_mul_and_calculate(input.as_str());

    println!("One Star: {one_star:?}");

    let mut two_star = 0;

    let mut dont = false;
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap(); 
    let mut current_input = input.as_str();
    loop {
        if !dont {
            let dont_mat = dont_regex.find(current_input);
            match dont_mat {
                None => {
                    two_star += find_mul_and_calculate(current_input);
                    break;
                },
                Some(mat) => {
                    two_star += find_mul_and_calculate(&current_input[..mat.start()]);
                    current_input = &current_input[mat.end()..];
                    dont = true
                }
            }
        } else {
            let do_mat = do_regex.find(current_input);
            match do_mat {
                None => {
                    break;
                },
                Some(mat) => {
                    current_input = &current_input[mat.end()..];
                    dont = false;
                }
            }
        }
    }

    println!("Two Star: {two_star:?}");
}