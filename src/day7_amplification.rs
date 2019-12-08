use std::io::{BufReader, BufRead};
use std::fs::File;
use permutohedron::heap_recursive;
//use crate::helper::print_if_sample;

pub fn exec_part1() -> () {
    //change per day
    let day_number = 7;
    //probably no need to change
    let sample_file_name = format!("src/input/day{}_sample.txt", day_number);
    run_part1(sample_file_name);

    //probably no need to change again
    let file_name = format!("src/input/day{}.txt", day_number);
    run_part1(file_name);
}

pub fn exec_part2() -> () {
    //change per day
    let day_number = 7;
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
    for line in reader.lines() {
        let line_string = line.unwrap();
        do_part1(line_string, 0)
    }
}

fn do_part1(line_string: String, input: i32) -> () {
    println!("======================================================");
    let mut data = [0, 1, 2, 3, 4];
    let mut permutations = Vec::new();
    heap_recursive(&mut data, |permutation| {
        permutations.push(permutation.to_vec())
    });
    let mut max_output = 0;

    println!("tmp: {}", line_string);
    for permutation in permutations.iter().cloned() {
        let mut inputs = Vec::new();
        for x in permutation {
            let input_vector = vec!(x);
            inputs.push(input_vector);
        }

        let mut amplifiers = Vec::new();
        let mut amplifier_ptr = Vec::new();
        for _i in 0..5 {
            let amplifier = line_string.split(',').map(|x| x.parse().unwrap()).collect();
            amplifiers.push(amplifier);
            amplifier_ptr.push(0);
        }

        inputs[0].push(input);
        let mut last_final_chain_value = 0;
        for current_machine in 0..5 {
            let outputs = run_that_machine(&mut inputs[current_machine], &mut amplifiers[current_machine], &mut amplifier_ptr[current_machine]);
            if outputs.is_empty() {
                panic!("aaaaaaaaaargh");
            } else {
                if current_machine == 4 {
                    last_final_chain_value = outputs[outputs.len() - 1];
                }
                if current_machine < 4 {
                    for x in outputs {
                        inputs[current_machine + 1].push(x);
                    }
                }
            }
        }
        max_output = if last_final_chain_value > max_output { last_final_chain_value } else { max_output };
    }
    println!("{}", max_output);
}

fn run_that_machine(inputs: &mut Vec<i32>, mut values: &mut Vec<i32>, start_pos: &mut usize) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    while *start_pos < values.len() {
        let opcode = values[*start_pos] % 100;
        if opcode == 99 {
            return output;
        } else if opcode == 3 && inputs.is_empty() {
            return output;
        }

        let param1_mode = (values[*start_pos] / 100) % 10;
        let param2_mode = (values[*start_pos] / 1000) % 10;

        let result = handle_opcode(&mut values, start_pos, opcode, param1_mode, param2_mode, inputs);
        *start_pos = result.0;
        if result.1 != Option::None {
            output.push(result.1.unwrap());
        }
    }
    return output;
}

fn get_value_for_mode(mode: i32, arg: i32, values: &Vec<i32>) -> i32 {
    if mode == 1 {
        return arg;
    }
    return values[arg as usize];
}

fn run_part2(file_name: String) {
    //probably leave this unchanged
    let file = File::open(&file_name).expect("unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_string = line.unwrap();
        do_part2(line_string, 0)
    }
}

fn do_part2(line_string: String, input: i32) -> () {
    println!("======================================================");
    let mut data = [5, 6, 7, 8, 9];
    let mut permutations = Vec::new();
    heap_recursive(&mut data, |permutation| {
        permutations.push(permutation.to_vec())
    });
    let mut max_output = 0;

    println!("tmp: {}", line_string);
    for permutation in permutations.iter().cloned() {
        let mut inputs = Vec::new();
        for x in permutation {
            let input_vector = vec!(x);
            inputs.push(input_vector);
        }

        let mut amplifiers = Vec::new();
        let mut amplifier_ptr = Vec::new();
        for _i in 0..5 {
            let amplifier = line_string.split(',').map(|x| x.parse().unwrap()).collect();
            amplifiers.push(amplifier);
            amplifier_ptr.push(0);
        }

        inputs[0].push(input);
        let mut last_final_chain_value = 0;
        let mut running = true;
        let mut current_machine = 0;
        while running {
            let outputs = run_that_machine(&mut inputs[current_machine], &mut amplifiers[current_machine], &mut amplifier_ptr[current_machine]);
            running = !outputs.is_empty();
            if running {
                if current_machine == 4 {
                    last_final_chain_value = outputs[outputs.len() - 1];
                }
                current_machine = (current_machine + 1) % 5;
                for x in outputs {
                    inputs[current_machine].push(x);
                }
            }
        }
        max_output = if last_final_chain_value > max_output { last_final_chain_value } else { max_output };
    }
    println!("{}", max_output);
}

fn handle_opcode(values: &mut Vec<i32>, start_pos: &usize, opcode: i32, param1_mode: i32, param2_mode: i32, inputs: &mut Vec<i32>) -> (usize, Option<i32>) {
    let mut output: Option<i32> = Option::None;
    let mut new_start_pos = start_pos + 1;//we have read the opcode at least
    let arg1 = values[start_pos + 1];
    let arg1_value = get_value_for_mode(param1_mode, arg1, &values);
    new_start_pos += 1;

    if opcode == 3 {
        values[arg1 as usize] = inputs[0];
        inputs.remove(0);
    } else if opcode == 4 {
        output = Option::Some(arg1_value);
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
    return (new_start_pos, output);
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