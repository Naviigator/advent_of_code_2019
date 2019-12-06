use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

pub fn exec_part1() -> () {
    //change per day
    let day_number = 6;
    //probably no need to change
    let sample_file_name = format!("src/input/day{}_sample.txt", day_number);
    run_part1(sample_file_name);

    //probably no need to change again
    let file_name = format!("src/input/day{}.txt", day_number);
    run_part1(file_name);
}

pub fn exec_part2() -> () {
    //change per day
    let day_number = 6;
    //probably no need to change
    let sample_file_name = format!("src/input/day{}_sample2.txt", day_number);
    run_part2(sample_file_name);

    //probably no need to change again
    let file_name = format!("src/input/day{}.txt", day_number);
    run_part2(file_name);
}

fn run_part1(file_name: String) {
    //probably leave this unchanged
    let file = File::open(&file_name).expect("unable to open file");
    let reader = BufReader::new(file);
    let mut orbits: HashMap<String, String> = HashMap::new();
    for line in reader.lines() {
        let line_string = line.unwrap();
        let split: Vec<&str> = line_string.split(")").collect();
        orbits.insert(String::from(split[1]), String::from(split[0]));
    }
    do_part1(&orbits)
}

fn do_part1(orbits: &HashMap<String, String>) -> () {
    let mut total_count = 0;
    for (k, _v) in orbits.iter() {
        let mut current_count = 0;
        let mut current_key = k;
        while current_key != "COM" {
            current_count += 1;
            current_key = orbits.get(current_key).unwrap();
        }
        total_count += current_count;
    }

    println!("{}", total_count);
}

fn run_part2(file_name: String) {
    //probably leave this unchanged
    let file = File::open(&file_name).expect("unable to open file");
    let reader = BufReader::new(file);
    let mut orbits: HashMap<String, String> = HashMap::new();
    for line in reader.lines() {
        let line_string = line.unwrap();
        let split: Vec<&str> = line_string.split(")").collect();
        orbits.insert(String::from(split[1]), String::from(split[0]));
    }
    do_part2(orbits)
}

fn do_part2(orbits: HashMap<String, String>) -> () {
    let mut my_path: HashMap<String, i32> = HashMap::new();
    let mut current_key = &String::from("YOU");
    let mut count = 0;
    let com = String::from("COM");
    while current_key != &com {
        current_key = orbits.get(current_key).unwrap();
        my_path.insert(String::from(current_key), count);
        count += 1;
    }
    let mut current_key = "SAN";
    let mut distance = 0;
    while current_key != "COM" {
        current_key = orbits.get(current_key).unwrap();
        if my_path.contains_key(current_key) {

            println!("{}", distance + *my_path.get(current_key).unwrap());
            return
        }
        distance += 1;
    }
}