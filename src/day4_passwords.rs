use std::fs;
//use crate::helper::print_if_sample;

pub fn exec_part1() -> () {
    //change per day
    let day_number = 4;

    //probably no need to change again
    let file_name = format!("src/input/day{}.txt", day_number);
    run_part1(file_name);
}

pub fn exec_part2() -> () {
    //change per day
    let day_number = 4;

    //probably no need to change again
    let file_name = format!("src/input/day{}.txt", day_number);
    run_part2(file_name);
}

fn run_part1(file_name: String) {
    //probably leave this unchanged
    let content = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");
    let values: Vec<i32> = content.split('-').map(|x| x.parse().unwrap()).collect();
    let lower = values[0];
    let higher = values[1];
    let mut possible_passwords = 0;
    for i in lower..higher {
        if is_password_compliant(i) {
            possible_passwords += 1;
        }
    }
    println!("{}", possible_passwords)
}

fn is_password_compliant(number: i32) -> bool {
    let mut current_number = number;
    let mut last_number = 10;
    let mut double_number_found = false;
    while current_number > 0 {
        let remainder = current_number % 10;
        if remainder > last_number {
            return false;
        }
        if remainder == last_number {
            double_number_found = true;
        }
        last_number = remainder;

        current_number = current_number / 10;
    }
    return double_number_found;
}

fn run_part2(file_name: String) {
    //probably leave this unchanged
    let content = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");
    let values: Vec<i32> = content.split('-').map(|x| x.parse().unwrap()).collect();
    let lower = values[0];
    let higher = values[1];
    let mut possible_passwords = 0;
    for i in lower..higher {
        if is_password_compliant_part2(i) {
            possible_passwords += 1;
        }
    }
    println!("{}", possible_passwords)
}

fn is_password_compliant_part2(number: i32) -> bool {
    let mut current_number = number;
    let mut last_number = 10;
    let mut last_number_count = 0;
    let mut min_last_number_count = 10;

    while current_number > 0 {
        let remainder = current_number % 10;
        if remainder > last_number {
            return false;
        }
        if remainder == last_number {
            last_number_count += 1;
        } else {
            if last_number_count >= 2 && last_number_count < min_last_number_count {
                min_last_number_count = last_number_count;
            }
            last_number_count = 1;
        }
        last_number = remainder;

        current_number = current_number / 10;
    }

    if last_number_count >= 2 && last_number_count < min_last_number_count {
        min_last_number_count = last_number_count;
    }

    return min_last_number_count == 2;
}