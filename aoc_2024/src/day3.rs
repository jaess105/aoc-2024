use std::sync::LazyLock;

use regex::{Captures, Regex};

use crate::{aoc_day::AocDayData, util::unwrap_to_i32};

pub fn day3() -> AocDayData {
    AocDayData::new(3, "resources/day03".to_string(), solve_a, solve_b)
}

static NUM_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
static INSTRUCTION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:mul\((\d+),(\d+)\))|(?:do\(\))|(?:don't\(\))").unwrap());

#[derive(PartialEq, Eq, Clone, Copy)]
enum Instruction {
    DO,
    DONT,
    MUL(i32, i32),
}

fn solve_b(input: String) -> i64 {
    let result = INSTRUCTION_RE
        .captures_iter(&input)
        .map(|m| match m.get(0).unwrap().as_str() {
            "do()" => Instruction::DO,
            "don't()" => Instruction::DONT,
            _ => Instruction::MUL(unwrap_i32_at(&m, 1), unwrap_i32_at(&m, 2)),
        })
        .fold(
            (Instruction::DO, 0_i32),
            |(curr_inst, sum), new_inst| match new_inst {
                Instruction::DO | Instruction::DONT => (new_inst, sum),
                Instruction::MUL(..) if curr_inst == Instruction::DONT => (curr_inst, sum),
                Instruction::MUL(first, second) => (curr_inst, sum + first * second),
            },
        );

    return result.1 as i64;

    fn unwrap_i32_at(capture: &Captures<'_>, i: usize) -> i32 {
        unwrap_to_i32(capture.get(i).unwrap().as_str())
    }
}

fn solve_a(input: String) -> i64 {
    NUM_RE
        .captures_iter(&input)
        .map(|nums| (nums.get(1).unwrap().as_str(), nums.get(2).unwrap().as_str()))
        .map(|(first, second)| (unwrap_to_i32(first), unwrap_to_i32(second)))
        .map(|(first, second)| first * second)
        .sum::<i32>() as i64
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
