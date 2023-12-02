use std::fs;

use aoc23::cube_conundrum::solve;

#[cfg(not(tarpaulin_include))]
fn main() {
    match fs::read_to_string("input/day02_cube_conundrum.txt") {
        Ok(input) => println!("Answer: {}", solve(&input)),
        Err(err) => panic!("Cannot read input file: {err}"),
    }
}
