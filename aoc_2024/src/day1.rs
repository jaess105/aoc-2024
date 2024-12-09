use std::collections::HashMap;

use regex::Regex;

use crate::aoc_day::AocDayData;

pub fn day1() -> AocDayData {
    AocDayData::new(1, "resources/day01".to_string(), solve_a, solve_b)
}

fn solve_b(input: String) -> i64 {
    let two_nums_rows = parse_input(&input);

    let right_occurrences_count =
        two_nums_rows
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
        .sum::<i32>() as i64
}

fn solve_a(input: String) -> i64 {
    let two_nums_rows = parse_input(&input);

    let (mut l_nums, mut r_nums): (Vec<i32>, Vec<i32>) =
        two_nums_rows.into_iter().map(|lr| (lr[0], lr[1])).unzip();

    l_nums.sort();
    r_nums.sort();

    l_nums
        .iter()
        .zip(r_nums.iter())
        .map(|(l, r)| i32::abs(l - r))
        .sum::<i32>() as i64
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
