extern crate rand;

use std::io::{BufReader, BufRead};
use std::fs::File;

fn get_fuel_cost(weight: i32) -> i32 {
    return weight / 3 - 2;
}

pub fn exec_part1() -> () {
    //change per day
    let day_number = 1;
    //probably no need to change
    let sample_file_name = format!("src/input/day{}_sample.txt", day_number);
    let sample_file = File::open(sample_file_name).expect("unable to open sample file");
    let sample_reader = BufReader::new(sample_file);

    for line in sample_reader.lines() {
        //change for sample with verbose output
        let nr = line.unwrap().parse().unwrap();
        println!("{}: {}", nr, get_fuel_cost(nr));
    }

    //probably no need to change again
    let file_name = format!("src/input/day{}.txt", day_number);
    let file = File::open(file_name).expect("unable to open sample file");
    let reader = BufReader::new(file);

    let mut total_cost = 0;
    for line in reader.lines() {
        //work your actual magic
        let nr = line.unwrap().parse().unwrap();

        total_cost += get_fuel_cost(nr);
    }
    println!("result: {0}", total_cost);
}

pub fn exec_part2() -> () {
    //change per day
    let day_number = 1;
    //probably no need to change
    let sample_file_name = format!("src/input/day{}_sample.txt", day_number);
    run_part2(sample_file_name);

    //probably no need to change again
    let file_name = format!("src/input/day{}.txt", day_number);
    run_part2(file_name);
}

fn run_part2(file_name: String) {
    //probably leave this unchanged
    let file = File::open(&file_name).expect("unable to open sample file");
    let reader = BufReader::new(file);
    let is_sample = file_name.contains("sample");

    //start making changes!
    let mut total_cost = 0;
    for line in reader.lines() {
        //change for sample with verbose output
        let nr = line.unwrap().parse().unwrap();
        let cost = get_fuel_cost(nr);
        total_cost += cost;
        print_if_sample(is_sample, format!("{}: {}", nr, cost));

        let mut cost_for_fuel_weight = get_fuel_cost(cost);
        while cost_for_fuel_weight > 0 {
            print_if_sample(is_sample, format!("{}", cost_for_fuel_weight));
            total_cost += cost_for_fuel_weight;
            cost_for_fuel_weight = get_fuel_cost(cost_for_fuel_weight)
        }
    }
    println!("{0}result: {1}", if is_sample {"sample "} else {""} ,total_cost);
}

fn print_if_sample(is_sample: bool, message: String) {
    if is_sample {
        println!("{}", message);
    }
}