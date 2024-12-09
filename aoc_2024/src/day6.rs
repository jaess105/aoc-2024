use crate::aoc_day::AocDayData;
use core::panic;
use std::collections::HashSet;

pub fn day() -> AocDayData {
    AocDayData::new(6, "resources/day06".to_string(), solve_a, solve_b)
}

fn solve_b(input: String) -> i64 {
    0
}

fn solve_a(input: String) -> i64 {
    let vec: Vec<Vec<char>> = parse_input(&input);
    let start = get_start_pos(&vec);
    if start.is_none() {
        panic!("No starting position found!");
    }

    let visited_pos = generate_all_positions(start.unwrap(), vec);

    visited_pos.len() as i64
}

fn get_bounds(vec: &Vec<Vec<char>>) -> (i32, i32) {
    (vec.len() as i32, vec[0].len() as i32)
}

fn get_start_pos(vec: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut start: Option<(usize, usize)> = None;

    for (i, row) in vec.iter().enumerate() {
        for (j, &el) in row.iter().enumerate() {
            if el == '^' {
                let _ = start.insert((i, j));
            }
        }
    }
    start
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn generate_all_positions(mut pos: (usize, usize), vec: Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let (x_max, y_max) = get_bounds(&vec);
    let mut visited_pos = HashSet::new();
    visited_pos.insert(pos);
    let mut orientation = Orientation::North;

    loop {
        let (x, y) = (pos.0 as i32, pos.1 as i32);
        let (x_add, y_add) = orientation.step();
        let (new_x, new_y) = (x.checked_add(x_add), y.checked_add(y_add));
        if new_x.is_none() || new_x.unwrap() >= x_max || new_y.is_none() || new_y.unwrap() >= y_max
        {
            break;
        }

        let (new_x, new_y) = (new_x.unwrap() as usize, new_y.unwrap() as usize);
        if vec[new_x][new_y] == '#' {
            // turn right and handle next step in next iteration
            orientation = orientation.turn_right();
            continue;
        }

        pos = (new_x, new_y);
        visited_pos.insert(pos);
    }
    visited_pos
}

enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    pub fn turn_right(&self) -> Self {
        match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }

    pub fn step(&self) -> (i32, i32) {
        match self {
            Orientation::North => (-1, 0),
            Orientation::East => (0, 1),
            Orientation::South => (1, 0),
            Orientation::West => (0, -1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_b() {
        todo!();
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 123);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 41);
    }
}
