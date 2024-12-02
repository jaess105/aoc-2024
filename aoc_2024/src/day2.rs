use std::fs;

use crate::aoc_day::AocDay;

pub struct Day2;

impl Day2 {
    pub const fn new() -> Self {
        Self
    }
}

impl AocDay for Day2 {
    fn get_file_path(&self) -> String {
        "resources/day02".to_string()
    }

    fn solve_a(&self, input: String) -> i32 {
        solve_a(input)
    }

    fn solve_b(&self, input: String) -> i32 {
        solve_b(input)
    }
}

pub fn solve() {
    const DAY: &str = "day2";
    const DAY1_FILE: &str = "resources/day02";
    let content = fs::read_to_string(DAY1_FILE).expect("Could not find input file!");

    let res = solve_a(content);
    println!("Result of {DAY} part a is {res}");

    let content = fs::read_to_string(DAY1_FILE).expect("Could not find input file!");
    let res = solve_b(content);
    println!("Result of {DAY} part b is {res}");
}

fn solve_b(input: String) -> i32 {
    let num_rows = parse_input(input);
    let mut safe_rows_count = 0;
    for row in num_rows {
        let diffs: Vec<i32> = calc_diffs(&row);
        if !is_safe(&diffs) && !any_subset_safe(row) {
            continue;
        }

        safe_rows_count += 1;
    }

    safe_rows_count
}

fn any_subset_safe(row: Vec<i32>) -> bool {
    (0..row.len()).any(|i| {
        let diffs = calc_diffs(&without_index(&row, i));
        is_safe(&diffs)
    })
}

fn without_index(v: &Vec<i32>, i: usize) -> Vec<i32> {
    v.iter()
        .enumerate()
        .filter_map(|(j, &x)| if j == i { None } else { Some(x) })
        .collect()
}

fn solve_a(input: String) -> i32 {
    let num_rows = parse_input(input);
    let mut safe_rows_count = 0;
    for row in num_rows {
        let diffs: Vec<i32> = calc_diffs(&row);
        if !is_safe(&diffs) {
            continue;
        }

        safe_rows_count += 1;
    }

    safe_rows_count
}

fn is_safe(diffs: &Vec<i32>) -> bool {
    let monotone = diffs.iter().all(|n| n < &0) || diffs.iter().all(|n| n > &0);
    if !monotone {
        return false;
    }
    let safe_range = diffs
        .iter()
        .map(|diff| i32::abs(*diff))
        .all(|diff| 0 < diff && diff <= 3);

    if !safe_range {
        return false;
    }
    true
}

fn calc_diffs(row: &Vec<i32>) -> Vec<i32> {
    row.windows(2).map(|win| win[0] - win[1]).collect()
}

fn parse_input(input: String) -> Vec<Vec<i32>> {
    let nums: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|num| i32::from_str_radix(num, 10).unwrap())
                .collect()
        })
        .collect();
    nums
}

#[cfg(test)]
mod tests {
    use super::{solve_a, solve_b};

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 4);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 2);
    }
}
