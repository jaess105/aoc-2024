use std::fs;

use aoc_day::AocDay;
use day1::Day1;
use day2::Day2;

mod aoc_day;
mod day1;
mod day2;

fn main() {
    let aocs: Vec<Box<dyn AocDay>> = vec![Box::new(Day1::new()), Box::new(Day2::new())];

    for aoc in aocs {
        solve(&aoc);
    }
}

pub fn solve(aoc: &Box<dyn AocDay>) {
    let day_number = aoc.get_day_number();
    let day_file = aoc.get_file_path();
    let content = fs::read_to_string(&day_file).expect("Could not find input file!");

    let res = aoc.solve_a(content);
    println!("Result of Day {day_number} part a is {res}");

    let content = fs::read_to_string(&day_file).expect("Could not find input file!");
    let res = aoc.solve_b(content);
    println!("Result of Day {day_number} part b is {res}");
}
