use std::collections::HashMap;

mod day0_hello_world;
mod day1_fuel_calc;
mod day2_intcode;
mod day3_crossing;
mod day4_passwords;
mod day5_airco;
mod day6_orbit;
mod helper;

fn main() {
    let day_to_execute = 6;
    let part_to_execute = 2;

    let mut things_to_execute: HashMap<(i32, i32), fn() -> ()> = HashMap::new();
    things_to_execute.insert((0, 1), day0_hello_world::exec_part1);
    things_to_execute.insert((1, 1), day1_fuel_calc::exec_part1);
    things_to_execute.insert((1, 2), day1_fuel_calc::exec_part2);
    things_to_execute.insert((2, 1), day2_intcode::exec_part1);
    things_to_execute.insert((2, 2), day2_intcode::exec_part2);
    things_to_execute.insert((3, 1), day3_crossing::exec_part1);
    things_to_execute.insert((3, 2), day3_crossing::exec_part2);
    things_to_execute.insert((4, 1), day4_passwords::exec_part1);
    things_to_execute.insert((4, 2), day4_passwords::exec_part2);
    things_to_execute.insert((5, 1), day5_airco::exec_part1);
    things_to_execute.insert((5, 2), day5_airco::exec_part2);
    things_to_execute.insert((6, 1), day6_orbit::exec_part1);
    things_to_execute.insert((6, 2), day6_orbit::exec_part2);

    let thing_to_execute = things_to_execute.get(&(day_to_execute, part_to_execute));
    if thing_to_execute.is_some() {
        thing_to_execute.unwrap()();
    }
}