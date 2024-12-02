use std::collections::HashMap;

use regex::Regex;

use crate::aoc_day::AocDay;

pub struct Day1;

impl Day1 {
    pub const fn new() -> Self {
        Self
    }
}

impl AocDay for Day1 {
    fn get_file_path(&self) -> String {
        "resources/day01".to_string()
    }

    fn solve_a(&self, input: String) -> i32 {
        solve_a(input)
    }

    fn solve_b(&self, input: String) -> i32 {
        solve_b(input)
    }
}

fn solve_b(input: String) -> i32 {
    let two_nums_rows = parse_input(&input);

    let right_occurrences_count = two_nums_rows
        .iter()
        .map(|lr| lr[1])
        .fold(HashMap::new(), |mut agg, num| {
            let entry_ref = agg.entry(num).or_insert(0);
            *entry_ref += 1;

            agg
        });

    two_nums_rows
        .iter()
        .map(|lr| lr[0] * right_occurrences_count.get(&lr[0]).unwrap_or(&0))
        .sum()
}

fn solve_a(input: String) -> i32 {
    let two_nums_rows = parse_input(&input);

    let capacity = two_nums_rows.len();
    let mut l_nums = Vec::with_capacity(capacity);
    let mut r_nums = Vec::with_capacity(capacity);
    for lr_num in two_nums_rows {
        l_nums.push(lr_num[0]);
        r_nums.push(lr_num[1]);
    }

    l_nums.sort();
    r_nums.sort();

    l_nums
        .iter()
        .zip(r_nums.iter())
        .map(|(l, r)| i32::abs(l - r))
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r" +").unwrap();

    let two_nums_rows: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            re.split(line)
                .map(|num| i32::from_str_radix(num, 10).unwrap())
                .collect()
        })
        .collect();
    two_nums_rows
}

#[cfg(test)]
mod tests {
    use crate::day1::solve_b;

    use super::solve_a;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 31);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 11);
    }
}