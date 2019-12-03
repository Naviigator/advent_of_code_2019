use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::{HashSet, HashMap};
//use crate::helper::print_if_sample;

pub fn exec_part1() -> () {
    //change per day
    let day_number = 3;
    //probably no need to change
    let sample_file_name = format!("src/input/day{}_sample.txt", day_number);
    run_part1(sample_file_name);

    //probably no need to change again
    let file_name = format!("src/input/day{}.txt", day_number);
    run_part1(file_name);
}

pub fn exec_part2() -> () {
    //change per day
    let day_number = 3;

    //probably no need to change
    let sample_file_name = format!("src/input/day{}_sample.txt", day_number);
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
    let mut inputs: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line_string = line.unwrap();
        inputs.push(line_string);
    }
    for i in (0..inputs.len()).step_by(2) {
        if i + 1 < inputs.len() {
            do_part1(is_sample, &inputs[i], &inputs[i + 1]);
        }
    }
}

fn do_part1(_is_sample: bool, inp1: &String, inp2: &String) -> () {
    let places1: HashSet<(i32, i32)> = str2set(inp1.split(',').collect());
    let places2: HashSet<(i32, i32)> = str2set(inp2.split(',').collect());
    let joint_places = places1.iter().
        filter(|x| places2.contains(x)).
        cloned().
        collect::<HashSet<(i32, i32)>>();

    let mut best_match: (i32, i32) = (10000000, 10000000);
    for x in joint_places {
        if x.0.abs() + x.1.abs() < best_match.0.abs() + best_match.1.abs() {
            best_match = x;
        }
    }
    println!("{0}, {1} = {2}", best_match.0, best_match.1, best_match.0.abs() + best_match.1.abs());
}

fn str2set(inp1_values: Vec<&str>) -> HashSet<(i32, i32)> {
    let mut places: HashSet<(i32, i32)> = HashSet::new();
    let mut pos = (0, 0);
    for x in inp1_values {
        let mut direction = x.chars().nth(0).unwrap();
        let distance = x[1..].parse().unwrap();
        let mut speed = 1;
        if direction == 'L' {
            speed = -1;
            direction = 'R';
        } else if direction == 'U' {
            speed = -1;
            direction = 'D';
        }

        if direction == 'R' {
            for _i in 0..distance {
                pos = (pos.0 + speed, pos.1);
                places.insert(pos);
            }
        } else if direction == 'D' {
            for _i in 0..distance {
                pos = (pos.0, pos.1 + speed);
                places.insert(pos);
            }
        }
    }
    return places;
}

fn run_part2(file_name: String) {
    //probably leave this unchanged
    let is_sample = file_name.contains("sample");
    let file = File::open(&file_name).expect("unable to open file");
    let reader = BufReader::new(file);
    let mut inputs: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line_string = line.unwrap();
        inputs.push(line_string);
    }
    for i in (0..inputs.len()).step_by(2) {
        if i + 1 < inputs.len() {
            do_part2(is_sample, &inputs[i], &inputs[i + 1]);
        }
    }
}

fn do_part2(_is_sample: bool, inp1: &String, inp2: &String) -> () {
    let places1: HashMap<(i32, i32), i32> = str2map(inp1.split(',').collect());
    let places2: HashMap<(i32, i32), i32> = str2map(inp2.split(',').collect());
    let joint_places = places1.
        iter().
        map(|(k, _v)| k).
        filter(|x| places2.contains_key(x)).
        cloned().
        collect::<HashSet<(i32, i32)>>();

    let mut best_match = 1000000000;
    for x in joint_places {
        let possible_match = places1[&x] + places2[&x];
        if possible_match < best_match {
            best_match = possible_match;
        }
    }
    println!("{0}", best_match);
}

fn str2map(inp1_values: Vec<&str>) -> HashMap<(i32, i32), i32> {
    let mut places: HashMap<(i32, i32), i32> = HashMap::new();
    let mut pos = (0, 0);
    let mut distance_travelled = 0;
    for x in inp1_values {
        let mut direction = x.chars().nth(0).unwrap();
        let distance = x[1..].parse().unwrap();
        let mut speed = 1;
        if direction == 'L' {
            speed = -1;
            direction = 'R';
        } else if direction == 'U' {
            speed = -1;
            direction = 'D';
        }

        let mut x_speed = 0;
        let mut y_speed = 0;

        if direction == 'R' {
            x_speed = speed;
        } else if direction == 'D' {
            y_speed = speed;
        }
        for _i in 0..distance {
            pos = (pos.0 + x_speed, pos.1 + y_speed);
            distance_travelled += 1;
            if !places.contains_key(&pos) {
                places.insert(pos, distance_travelled);
            }
        }
    }
    return places;
}