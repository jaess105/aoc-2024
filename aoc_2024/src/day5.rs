use std::collections::{HashMap, HashSet};

use crate::{aoc_day::AocDayData, util::unwrap_to_i32};

pub fn day() -> AocDayData {
    AocDayData::new(5, "resources/day05".to_string(), solve_a, solve_b)
}

fn solve_b(input: String) -> i32 {
    let (rules, prints) = pares_input(&input);
    let rule_map: HashMap<i32, HashSet<i32>> = generate_rules(rules);

    let mut valid_middle_page_sum = 0;
    'line_check: for line in prints
        .iter()
        .map(|&line| line.split(",").filter(|&el| !el.is_empty()))
    {
        let mut already_printed = Vec::new();
        let line_nums: Vec<_> = line.into_iter().map(|num| unwrap_to_i32(num)).collect();
        for num in line_nums.iter() {
            // If there are no successors, the num is always valid.
            if let Some(successors) = rule_map.get(num) {
                for pred in already_printed.iter() {
                    // the line check should be continued, when there was a successor in the already printed numbers.
                    // As this makes the line invalid
                    if successors.contains(pred) {
                        continue 'line_check;
                    }
                }
            }

            already_printed.push(*num);
        }
        // If we reach this, there was no successor printed before a predecessor, thus the line was valid
        let len = line_nums.len();
        assert!(len >= 3, "Not long enough line: {:?}", line_nums);

        valid_middle_page_sum += line_nums[(len - 1) / 2]
    }

    valid_middle_page_sum
}

fn generate_rules(rules: Vec<&str>) -> HashMap<i32, HashSet<i32>> {
    rules
        .iter()
        .map(|&line| line.trim().split_once("|").unwrap())
        .map(|(pred, suc)| (unwrap_to_i32(pred), unwrap_to_i32(suc)))
        .fold(HashMap::new(), |mut agg, (pred, suc)| {
            let set = agg.entry(pred).or_insert(HashSet::new());

            set.insert(suc);

            agg
        })
}

fn solve_a(input: String) -> i32 {
    let (rules, prints) = pares_input(&input);
    let rule_map: HashMap<i32, HashSet<i32>> = generate_rules(rules);

    let mut valid_middle_page_sum = 0;
    'line_check: for line in prints
        .iter()
        .map(|&line| line.split(",").filter(|&el| !el.is_empty()))
    {
        let mut already_printed = Vec::new();
        let line_nums: Vec<_> = line.into_iter().map(|num| unwrap_to_i32(num)).collect();
        for num in line_nums.iter() {
            // If there are no successors, the num is always valid.
            if let Some(successors) = rule_map.get(num) {
                for pred in already_printed.iter() {
                    // the line check should be continued, when there was a successor in the already printed numbers.
                    // As this makes the line invalid
                    if successors.contains(pred) {
                        continue 'line_check;
                    }
                }
            }

            already_printed.push(*num);
        }
        // If we reach this, there was no successor printed before a predecessor, thus the line was valid
        let len = line_nums.len();
        assert!(len >= 3, "Not long enough line: {:?}", line_nums);

        valid_middle_page_sum += line_nums[(len - 1) / 2]
    }

    valid_middle_page_sum
}

fn pares_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let lines: Vec<&str> = input.lines().collect();
    let split_idx = lines.iter().position(|&s| s == "").unwrap();
    let (rules, prints) = lines.split_at(split_idx);
    // the first entry is the empty line;
    let prints = prints[1..].to_vec();
    (rules.to_vec(), prints.to_vec())
}

#[cfg(test)]
mod tests {
    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 9);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 143);
    }
}
