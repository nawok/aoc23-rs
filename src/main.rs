use std::fs;

use aoc23::trebuchet::get_sum_calibration_values;

#[cfg(not(tarpaulin_include))]
fn main() {
    match fs::read_to_string("input/trebuchet.txt") {
        Ok(input) => println!("Answer: {}", get_sum_calibration_values(&input)),
        Err(err) => panic!("Cannot read input file: {err}"),
    }
}
