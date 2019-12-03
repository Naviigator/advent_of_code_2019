use std::io::{BufReader, BufRead};
use std::fs::File;
//use crate::helper::print_if_sample;

pub fn exec_part1() -> () {
    //change per day
    let day_number = 2;
    //probably no need to change
    let sample_file_name = format!("src/input/day{}_sample.txt", day_number);
    run_part1(sample_file_name);

    //probably no need to change again
    let file_name = format!("src/input/day{}.txt", day_number);
    run_part1(file_name);
}

pub fn exec_part2() -> () {
    //change per day
    let day_number = 2;

    //probably no need to change again
    let file_name = format!("src/input/day{}.txt", day_number);
    run_part2(file_name);
}

fn run_part1(file_name: String) {
    //probably leave this unchanged
    let is_sample = file_name.contains("sample");
    let file = File::open(&file_name).expect("unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_string = line.unwrap();
        do_part1(is_sample, line_string)
    }
}

fn do_part1(is_sample: bool, line_string: String) -> () {
    let mut values: Vec<i32> = line_string.split(',').map(|x| x.parse().unwrap()).collect();
    if !is_sample {
        values[1] = 12;
        values[2] = 2;
    }
    for i in (0..values.len()).step_by(4) {
        let start_pos = i as usize;
        let opcode = values[start_pos];

        if opcode == 99 {
            if is_sample {
                for j in 0..values.len() {
                    if j == start_pos {
                        print!("(");
                    }
                    print!("{}", values[j]);
                    if j == start_pos {
                        print!(")");
                    }
                    print!(",");
                }
                println!("");
            }
            println!("{0}result: {1}", if is_sample { "sample " } else { "" }, values[0]);
            return;
        } else {
            let arg1_pos = values[start_pos + 1] as usize;
            let arg2_pos = values[start_pos + 2] as usize;
            let output_pos = values[start_pos + 3] as usize;
            if opcode == 1 {
                values[output_pos] = values[arg1_pos] + values[arg2_pos];
            } else if opcode == 2 {
                values[output_pos] = values[arg1_pos] * values[arg2_pos];
            } else {
                println!("Oh no, something went horribly wrong!");
            }
        }
    }
}

fn run_part2(file_name: String) {
    //probably leave this unchanged
    let is_sample = file_name.contains("sample");
    let file = File::open(&file_name).expect("unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_string = line.unwrap();
        do_part2(is_sample, line_string)
    }
}

fn do_part2(is_sample: bool, line_string: String) -> () {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut values: Vec<i32> = line_string.split(',').map(|x| x.parse().unwrap()).collect();
            if !is_sample {
                values[1] = noun;
                values[2] = verb;
            }
            for i in (0..values.len()).step_by(4) {
                let start_pos = i as usize;
                let opcode = values[start_pos];

                if opcode == 99 {
                    if values[0] == 19690720 {
                        println!("yep! {}", 100 * noun + verb);
                        return;
                    }
                    println!("nope... {0}, {1}", noun, verb);
                    for j in 0..values.len() {
                        if j == start_pos {
                            print!("(");
                        }
                        print!("{}", values[j]);
                        if j == start_pos {
                            print!(")");
                        }
                        print!(",");
                    }
                    println!("");
                    break;
                } else {
                    let arg1_pos = values[start_pos + 1] as usize;
                    let arg2_pos = values[start_pos + 2] as usize;
                    let output_pos = values[start_pos + 3] as usize;
                    if opcode == 1 {
                        values[output_pos] = values[arg1_pos] + values[arg2_pos];
                    } else if opcode == 2 {
                        values[output_pos] = values[arg1_pos] * values[arg2_pos];
                    } else {
                        println!("Oh no, something went horribly wrong!");
                    }
                }
            }
        }
    }
}