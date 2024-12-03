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

#[derive(PartialEq, Eq, Clone, Copy)]
enum Action {
    DO,
    DONT,
    MUL(i32, i32),
}

impl Action {
    fn multimply(&self) -> i32 {
        match self {
            Action::DO | Action::DONT => 0,
            Action::MUL(first, second) => *first * *second,
        }
    }
}

fn solve_b(input: String) -> i32 {
    let regex = Regex::new(r"(?:mul\((\d+),(\d+)\))|(?:do\(\))|(?:don't\(\))").unwrap();
    let result = regex
        .captures_iter(&input)
        .map(|m| match m.get(0).unwrap().as_str() {
            "do()" => Action::DO,
            "don't()" => Action::DONT,
            _ => Action::MUL(unwrap_i32_at(&m, 1), unwrap_i32_at(&m, 2)),
        })
        .fold((Action::DO, 0_i32), |agg, next| match next {
            Action::DO | Action::DONT => (next, agg.1),
            _ => {
                if agg.0 == Action::DONT {
                    agg
                } else {
                    (agg.0, agg.1 + next.multimply())
                }
            }
        });

    return result.1;

    fn unwrap_i32_at(capture: &Captures<'_>, i: usize) -> i32 {
        to_i32(capture.get(i).unwrap().as_str())
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
    fn test_a() {
        let result = solve_a(TEST_INPUT_A.into());
        assert_eq!(result, 161);
    }
}
