use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn exec_part1() -> () {
    //change per day
    let day_number = 8;
    //probably no need to change
    let sample_file_name = format!("src/input/day{}_sample.txt", day_number);
    run_part1(sample_file_name, 3, 2);

    //probably no need to change again
    let file_name = format!("src/input/day{}.txt", day_number);
    run_part1(file_name, 25, 6);
}

pub fn exec_part2() -> () {
    //change per day
    let day_number = 8;
    //probably no need to change
    let sample_file_name = format!("src/input/day{}_sample2.txt", day_number);
    run_part2(sample_file_name, 2, 2);

    //probably no need to change again
    let file_name = format!("src/input/day{}.txt", day_number);
    run_part2(file_name, 25, 6);
}

fn run_part1(file_name: String, width: usize, height: usize) {
    //probably leave this unchanged
    let file = File::open(&file_name).expect("unable to open file");
    let reader = BufReader::new(file);
    let mut colors: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line_string = line.unwrap();
        for c in line_string.chars() {
            colors.push(c.to_string().parse().unwrap());
        }
    }
    do_part1(&colors, width, height)
}

fn do_part1(colors: &Vec<i32>, width: usize, height: usize) -> () {
    let mut layers: Vec<Vec<i32>> = Vec::new();
    let layer_size = width * height;
    for index in 0..colors.len() {
        if index % layer_size == 0 {
            layers.push(Vec::new());
        }
        let current_layer = index / layer_size;
        layers[current_layer].push(*colors.get(index).unwrap());
    }

    let mut best_layer = 0;
    let mut best_layer_count = 1000000;
    for index in 0..layers.len() {
        let current_layer_count = layers[index].iter().filter(|x| x==&&0).count();
        println!("layer: {}, count: {}", index, current_layer_count);
        if current_layer_count < best_layer_count {
            best_layer_count = current_layer_count;
            best_layer = index;
        }
    }
    println!("best layer: {}", best_layer);
    let ones = layers[best_layer].iter().filter(|x| x==&&1).count();
    let twos = layers[best_layer].iter().filter(|x| x==&&2).count();
    println!("{}", ones * twos);
}

fn run_part2(file_name: String, width: usize, height: usize) {
    //probably leave this unchanged
    let file = File::open(&file_name).expect("unable to open file");
    let reader = BufReader::new(file);
    let mut colors: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line_string = line.unwrap();
        for c in line_string.chars() {
            colors.push(c.to_string().parse().unwrap());
        }
    }
    do_part2(&colors, width, height)
}

fn do_part2(colors: &Vec<i32>, width: usize, height: usize) -> () {
    let mut layers: Vec<Vec<i32>> = Vec::new();
    let layer_size = width * height;
    for index in 0..colors.len() {
        if index % layer_size == 0 {
            layers.push(Vec::new());
        }
        let current_layer = index / layer_size;
        layers[current_layer].push(*colors.get(index).unwrap());
    }

    let mut image: Vec<i32> = layers[0].iter().cloned().collect();
    for index in 1..layers.len() {
        for n in 0..image.len() {
            if image[n] == 2 {
                image[n] = layers[index][n];
            }
        }
    }
    for h in 0..height {
        for w in 0..width {
            let val = image[h*width + w];
            print!("{}", if val == 1 { "1" } else { " " });
        }
        println!();
    }
}