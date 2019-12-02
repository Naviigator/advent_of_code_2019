use std::collections::HashMap;

mod day0_hello_world;
mod day1_fuel_calc;
mod day2_intcode;
mod helper;

fn main() {
    let day_to_execute = 2;
    let part_to_execute = 2;

    let mut things_to_execute: HashMap<(i32, i32), fn() -> ()> = HashMap::new();
    things_to_execute.insert((0, 1), day0_hello_world::exec_part1);
    things_to_execute.insert((1, 1), day1_fuel_calc::exec_part1);
    things_to_execute.insert((1, 2), day1_fuel_calc::exec_part2);
    things_to_execute.insert((2, 1), day2_intcode::exec_part1);
    things_to_execute.insert((2, 2), day2_intcode::exec_part2);

    let thing_to_execute = things_to_execute.get(&(day_to_execute, part_to_execute));
    if thing_to_execute.is_some() {
        thing_to_execute.unwrap()();
    }
}
