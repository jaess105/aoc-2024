use crate::{aoc_day::AocDayData, util::unwrap_to_i32};
use std::collections::{HashMap, HashSet};

pub fn day() -> AocDayData {
    AocDayData::new(5, "resources/day05".to_string(), solve_a, solve_b)
}

fn solve_b(input: String) -> i64 {
    let (rules, prints): (Vec<&str>, Vec<Vec<i32>>) = pares_input(&input);
    let rule_map: HashMap<i32, HashSet<i32>> = generate_rules(rules);

    prints
        .into_iter()
        .filter(|line_nums| !is_valid(&rule_map, line_nums))
        .map(|line_nums| line_nums.iter().fold(vec![], order_correctly(&rule_map)))
        .map(get_middle_value)
        .sum::<i32>() as i64
}

/// Orders a line by inserting an element before the first occurring successor, or appends it,
/// if none of the present elements were predecessors.
fn order_correctly<'a>(
    rule_map: &'a HashMap<i32, HashSet<i32>>,
) -> impl Fn(Vec<i32>, &i32) -> Vec<i32> + 'a {
    move |mut agg, num| {
        for (i, pred) in agg.iter().enumerate() {
            if let Some(successors) = rule_map.get(num) {
                if successors.contains(pred) {
                    agg.insert(i, *num);
                    return agg;
                }
            }
        }
        agg.push(*num);
        agg
    }
}

fn solve_a(input: String) -> i64 {
    let (rules, prints): (Vec<&str>, Vec<Vec<i32>>) = pares_input(&input);
    let rule_map: HashMap<i32, HashSet<i32>> = generate_rules(rules);

    prints
        .into_iter()
        .filter(|line_nums| is_valid(&rule_map, line_nums))
        .map(get_middle_value)
        .sum::<i32>() as i64
}

fn get_middle_value(line_nums: Vec<i32>) -> i32 {
    line_nums[(line_nums.len() - 1) / 2]
}
fn is_valid(rule_map: &HashMap<i32, HashSet<i32>>, line_nums: &Vec<i32>) -> bool {
    line_nums
        .iter()
        .fold(Some(Vec::new()), |mut opt_vec, num| {
            if let Some(mut predecessors) = opt_vec.take() {
                // If there are no successors, the num is always valid.
                if let Some(successors) = rule_map.get(num) {
                    if predecessors.iter().any(|pred| successors.contains(pred)) {
                        return None;
                    }
                }
                predecessors.push(*num);
                return Some(predecessors);
            } else {
                None
            }
        })
        .is_some()
}

fn generate_rules(rules: Vec<&str>) -> HashMap<i32, HashSet<i32>> {
    rules
        .iter()
        .map(|&line| line.trim().split_once("|").unwrap())
        .map(|(pred, suc)| (unwrap_to_i32(pred), unwrap_to_i32(suc)))
        .fold(HashMap::new(), |mut agg, (pred, suc)| {
            agg.entry(pred).or_insert(HashSet::new()).insert(suc);
            agg
        })
}

fn pares_input(input: &str) -> (Vec<&str>, Vec<Vec<i32>>) {
    let lines: Vec<&str> = input.lines().collect();
    let split_idx = lines.iter().position(|&s| s == "").unwrap();
    let (rules, prints) = lines.split_at(split_idx);
    // the first entry is the empty line;
    let prints = prints[1..].to_vec();
    (rules.to_vec(), prepare_printed_lines(prints))
}

/// Use the print vector of strings and make it to a vector of vectors of integers.
/// The strings themselves are currently the lines, and they are converted to i32s.
fn prepare_printed_lines(prints: Vec<&str>) -> Vec<Vec<i32>> {
    prints
        .iter()
        .map(|&line| line.split(",").filter(|&el| !el.is_empty()))
        .map(|line| line.into_iter().map(|num| unwrap_to_i32(num)).collect())
        .collect()
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
        assert_eq!(result, 123);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 143);
    }
}
