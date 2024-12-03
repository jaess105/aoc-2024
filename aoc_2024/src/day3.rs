use std::sync::LazyLock;

use regex::{CaptureMatches, Captures, Regex};

use crate::aoc_day::AocDay;

pub struct Day3;

impl Day3 {
    pub const fn new() -> Self {
        Self
    }
}

impl AocDay for Day3 {
    fn get_file_path(&self) -> String {
        "resources/day03".to_string()
    }

    fn solve_a(&self, input: String) -> i32 {
        solve_a(input)
    }

    fn solve_b(&self, input: String) -> i32 {
        solve_b(input)
    }

    fn get_day_number(&self) -> u8 {
        3
    }
}

static NUM_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
static ACTION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:do\(\))|(?:don't\(\))").unwrap());

#[derive(PartialEq, Eq)]
enum Action {
    DO,
    DONT,
}

fn solve_b(input: String) -> i32 {
    let mut num_caps = NUM_RE.captures_iter(&input);
    let mut action_caps = ACTION_RE.captures_iter(&input);

    let mut num_cap = num_caps.next();
    let mut action_cap = action_caps.next();
    let mut current_action = Action::DO;

    let mut sum = 0;

    loop {
        if num_cap.is_none() {
            break;
        }
        let num_start = get_start(&num_cap);
        let action_start = get_start(&action_cap);

        if action_start.is_none() || num_start.unwrap() < action_start.unwrap() {
            if current_action == Action::DO {
                sum += unwrap_i32_at(&num_cap, 1) * unwrap_i32_at(&num_cap, 2);
            }

            num_cap = num_caps.next();
        } else {
            match unwrap_str_at(&action_cap, 0).as_str() {
                "do()" => current_action = Action::DO,
                "don't()" => current_action = Action::DONT,
                val => panic!("Matched an value that cannot be matched: {val}"),
            }

            action_cap = action_caps.next();
        }
    }
    return sum;

    fn unwrap_i32_at(capture: &Option<Captures<'_>>, i: usize) -> i32 {
        to_i32(&unwrap_str_at(capture, i))
    }

    fn unwrap_str_at(capture: &Option<Captures<'_>>, i: usize) -> String {
        capture
            .as_ref()
            .map(|cap| cap.get(i).unwrap().as_str())
            .unwrap()
            .to_string()
    }

    fn get_start(capture: &Option<Captures<'_>>) -> Option<usize> {
        capture.as_ref().map(|cap| cap.get(0).unwrap().start())
    }
}

fn solve_a(input: String) -> i32 {
    NUM_RE
        .captures_iter(&input)
        .map(|nums| (nums.get(1).unwrap().as_str(), nums.get(2).unwrap().as_str()))
        .map(|(first, second)| (to_i32(first), to_i32(second)))
        .map(|(first, second)| first * second)
        .sum::<i32>()
}

fn to_i32(s: &str) -> i32 {
    i32::from_str_radix(s, 10).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day3::ACTION_RE;

    use super::{solve_a, solve_b};

    const TEST_INPUT_B: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    const TEST_INPUT_A: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT_B.into());
        assert_eq!(result, 48);
    }

    #[test]
    fn test_action() {
        let was_match = ACTION_RE.is_match(TEST_INPUT_B);
        assert!(was_match);
    }

    #[test]
    fn test_action_regex_dont() {
        let was_match = ACTION_RE.is_match("don't()");
        assert!(was_match);
    }

    #[test]
    fn test_action_regex_do() {
        let was_match = ACTION_RE.is_match("do()");
        assert!(was_match);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT_A.into());
        assert_eq!(result, 161);
    }
}
