use crate::aoc_day::AocDayData;
use crate::util::unwrap_to_i64;
use std::collections::HashSet;
use std::fmt::format;

pub fn day() -> AocDayData {
    AocDayData::new(7, "resources/day07".to_string(), solve_a, solve_b)
}

fn solve_b(input: String) -> i64 {
    let vec = parse_input(&input);
    check_all_possible_equations_con(vec)
}

fn check_all_possible_equations_con(vec: Vec<(i64, Vec<i64>)>) -> i64 {
    let mut possible_count = 0;
    for (target, nums) in vec {
        let results = nums.iter().fold(HashSet::new(), |mut agg, &num| {
            if agg.len() == 0 {
                agg.insert(num);
                agg
            } else {
                let mut new_agg = HashSet::new();
                for el in agg {
                    new_agg.insert(el * num);
                    new_agg.insert(el + num);
                    new_agg.insert(unwrap_to_i64(&format!("{el}{num}")));
                }
                new_agg
            }
        });
        if results.contains(&target) {
            possible_count += target;
        }
    }

    possible_count
}

fn solve_a(input: String) -> i64 {
    let vec = parse_input(&input);
    check_all_possible_equations(vec)
}

fn check_all_possible_equations(vec: Vec<(i64, Vec<i64>)>) -> i64 {
    let mut possible_count = 0;
    for (target, nums) in vec {
        let results = nums.iter().fold(HashSet::new(), |mut agg, &num| {
            if agg.len() == 0 {
                agg.insert(num);
                agg
            } else {
                let mut new_agg = HashSet::new();
                for el in agg {
                    new_agg.insert(el * num);
                    new_agg.insert(el + num);
                }
                new_agg
            }
        });
        if results.contains(&target) {
            possible_count += target;
        }
    }

    possible_count
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| line.split(':'))
        .map(|mut content| (content.next(), content.next()))
        .map(|(target, nums)| {
            (
                unwrap_to_i64(target.unwrap()),
                nums.unwrap()
                    .split(" ")
                    .filter(|&s| !s.is_empty())
                    .map(unwrap_to_i64)
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 11387);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 3749);
    }
}
