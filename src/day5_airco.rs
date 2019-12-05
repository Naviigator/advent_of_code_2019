use std::io::{BufReader, BufRead};
use std::fs::File;
//use crate::helper::print_if_sample;

pub fn exec_part1() -> () {
    //change per day
    let day_number = 5;
    //probably no need to change
    let sample_file_name = format!("src/input/day{}_sample.txt", day_number);
    run_part1(sample_file_name);

    //probably no need to change again
    let file_name = format!("src/input/day{}.txt", day_number);
    run_part1(file_name);
}

pub fn exec_part2() -> () {
    //change per day
    let day_number = 5;
    //probably no need to change
    let sample_file_name = format!("src/input/day{}_sample2.txt", day_number);
    run_part2(sample_file_name);

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
    println!("======================================================");
    let mut values: Vec<i32> = line_string.split(',').map(|x| x.parse().unwrap()).collect();
    let mut start_pos = 0 as usize;
    while start_pos < values.len() {
        let opcode = values[start_pos] % 100;
        let param1_mode = (values[start_pos] / 100) % 10;
        let param2_mode = (values[start_pos] / 1000) % 10;
        //let param3_mode = (values[start_pos]/10000)%10;

        if opcode == 99 {
            if is_sample {
                print_values(&values, start_pos);
            }
            println!("Halting");
            return;
        } else {
            println!("opcode: {}", opcode);
            let arg1 = values[start_pos + 1];
            if opcode == 3 {
                println!("inputting into {}...", arg1);
                values[arg1 as usize] = 1;//test UID
                start_pos += 2;
            } else if opcode == 4 {
                println!("Outputting... {}", get_value_for_mode(param1_mode, arg1, &values));
                start_pos += 2;
            } else {
                let arg2 = values[start_pos + 2];
                let arg3 = values[start_pos + 3];
                if opcode == 1 {
                    values[arg3 as usize] = get_value_for_mode(param1_mode, arg1, &values) + get_value_for_mode(param2_mode, arg2, &values);
                } else if opcode == 2 {
                    values[arg3 as usize] = get_value_for_mode(param1_mode, arg1, &values) * get_value_for_mode(param2_mode, arg2, &values);
                } else {
                    println!("Oh no, something went horribly wrong!");
                }
                start_pos += 4;
            }
        }
    }
}

fn get_value_for_mode(mode: i32, arg: i32, values: &Vec<i32>) -> i32 {
    if mode == 1 {
        return arg;
    }
    return values[arg as usize];
}

fn print_values(values: &Vec<i32>, start_pos: usize) {
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
    println!();
}

fn run_part2(file_name: String) {
    //probably leave this unchanged
    let file = File::open(&file_name).expect("unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_string = line.unwrap();
        do_part2(line_string, 5)
    }
}

fn do_part2(line_string: String, input: i32) -> () {
    println!("======================================================");
    let mut values: Vec<i32> = line_string.split(',').map(|x| x.parse().unwrap()).collect();
    let mut start_pos = 0 as usize;
    while start_pos < values.len() {
        let opcode = values[start_pos] % 100;
        if opcode == 99 {
            println!("Halting");
            return;
        }

        let param1_mode = (values[start_pos] / 100) % 10;
        let param2_mode = (values[start_pos] / 1000) % 10;

        let new_start_pos = handle_opcode(&mut values, &mut start_pos, opcode, param1_mode, param2_mode, input);
        start_pos = new_start_pos;
    }
}

fn handle_opcode(values: &mut Vec<i32>, start_pos: &usize, opcode: i32, param1_mode: i32, param2_mode: i32, input: i32) -> usize {
    let mut new_start_pos = start_pos + 1;//we have read the opcode at least
    let arg1 = values[start_pos + 1];
    let arg1_value = get_value_for_mode(param1_mode, arg1, &values);
    new_start_pos += 1;

    if opcode == 3 {
        println!("inputting {} into [{}]...", input, arg1);
        values[arg1 as usize] = input;
    } else if opcode == 4 {
        println!("Outputting... {}", arg1_value);
    } else {
        let arg2 = values[start_pos + 2];
        let arg2_value = get_value_for_mode(param2_mode, arg2, &values);
        new_start_pos += 1;
        if opcode == 5 {
            if arg1_value != 0 {
                new_start_pos = arg2_value as usize
            }
        } else if opcode == 6 {
            if arg1_value == 0 {
                new_start_pos = arg2_value as usize
            }
        } else {
            let arg3 = values[start_pos + 3];
            new_start_pos += 1;
            handle_3_args(values, opcode, arg1_value, arg2_value, arg3 as usize)
        }
    }
    return new_start_pos;
}

fn handle_3_args(values: &mut Vec<i32>, opcode: i32, arg1_value: i32, arg2_value: i32, arg3: usize) -> () {
    if opcode == 1 {
        values[arg3] = arg1_value + arg2_value;
    } else if opcode == 2 {
        values[arg3] = arg1_value * arg2_value;
    } else if opcode == 7 {
        values[arg3] = if arg1_value < arg2_value { 1 } else { 0 };
    } else if opcode == 8 {
        values[arg3] = if arg1_value == arg2_value { 1 } else { 0 };
    } else {
        panic!("Oh no, something went horribly wrong!");
    }
}