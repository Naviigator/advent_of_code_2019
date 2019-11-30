extern crate rand;

use rand::Rng;

pub fn exec_part1() -> () {
    let hello = "Hello";
    let world = "world";
    let terminators = [".", "!", ";)"];
    let terminator = terminators[rand::thread_rng().gen_range(0, terminators.len())];
    println!("{0} {1}{2}", hello, world, terminator);
}
