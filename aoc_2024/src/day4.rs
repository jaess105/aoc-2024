use std::iter;

use crate::aoc_day::AocDayData;

pub fn day4() -> AocDayData {
    AocDayData::new(4, "resources/day04".to_string(), solve_a, solve_b)
}

fn solve_b(input: String) -> i64 {
    let matrix: Vec<Vec<char>> = parse_input(input);

    let starts: Vec<(usize, usize)> = find_position_of(&matrix, 'A');

    starts
        .iter()
        .map(|idx| count_x_mas(&matrix, *idx))
        .sum::<i32>() as i64
}

fn count_x_mas(m: &Vec<Vec<char>>, (i, j): (usize, usize)) -> i32 {
    let jp1 = if let Some(inbounds) = m.get(i).map(|row| row.len() > j + 1) {
        if inbounds {
            Some(j + 1)
        } else {
            None
        }
    } else {
        None
    };
    let ip1 = if i + 1 < m.len() { Some(i + 1) } else { None };
    let im1 = i.checked_sub(1);
    let jm1 = j.checked_sub(1);

    if ip1.is_none() || im1.is_none() || jp1.is_none() || jm1.is_none() {
        return 0;
    }

    let nw = m[im1.unwrap()][jp1.unwrap()];
    let ne = m[im1.unwrap()][jm1.unwrap()];
    let se = m[ip1.unwrap()][jm1.unwrap()];
    let sw = m[ip1.unwrap()][jp1.unwrap()];

    if nw == 'M' && ne == 'M' && sw == 'S' && se == 'S'
        || ne == 'M' && se == 'M' && sw == 'S' && nw == 'S'
        || se == 'M' && sw == 'M' && nw == 'S' && ne == 'S'
        || sw == 'M' && nw == 'M' && ne == 'S' && se == 'S'
    {
        1
    } else {
        0
    }
}

fn solve_a(input: String) -> i64 {
    let matrix: Vec<Vec<char>> = parse_input(input);

    let starts: Vec<(usize, usize)> = find_position_of(&matrix, 'X');

    starts
        .iter()
        .map(|idx| count_xmas(&matrix, *idx))
        .sum::<i32>() as i64
}

fn count_xmas(input: &Vec<Vec<char>>, (i, j): (usize, usize)) -> i32 {
    // horizontal
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
    let mut count = 0;

    let j_pos_valid = j + 3 < input.len();
    let j_neg_valid = j.checked_sub(3).is_some();
    let i_pos_valid = i + 3 < input.len();
    let i_neg_valid = i.checked_sub(3).is_some();

    let directions: Vec<Vec<(usize, usize)>> = vec![
        // right to left
        if_zip_to_vec(j_pos_valid, || (iter::repeat(i)), || (j..j + 4)),
        // left to right
        if_zip_to_vec(j_neg_valid, || (iter::repeat(i)), || ((j - 3..=j).rev())),
        // up to down
        if_zip_to_vec(i_pos_valid, || (i..i + 4), || (iter::repeat(j))),
        // down to up
        if_zip_to_vec(i_neg_valid, || ((i - 3..=i).rev()), || (iter::repeat(j))),
        // south east
        if_zip_to_vec(i_pos_valid && j_pos_valid, || (i..i + 4), || (j..j + 4)),
        // south west
        if_zip_to_vec(
            i_pos_valid && j_neg_valid,
            || (i..i + 4),
            || ((j - 3..=j).rev()),
        ),
        // north west
        if_zip_to_vec(
            i_neg_valid && j_neg_valid,
            || (i - 3..=i).rev(),
            || (j - 3..=j).rev(),
        ),
        // north east
        if_zip_to_vec(
            i_neg_valid && j_pos_valid,
            || (i - 3..=i).rev(),
            || j..j + 4,
        ),
    ]
    .into_iter()
    .filter_map(|x| x)
    .collect();

    'outer: for it in directions {
        for (idx, (x, y)) in it.into_iter().enumerate() {
            if let Some(Some(c)) = input.get(x).map(|row| row.get(y)) {
                if *c == XMAS[idx] {
                    continue;
                }
            }

            // if it didn't match or there was another problem, just break the inner loop.
            continue 'outer;
        }
        count += 1;
    }

    return count;

    fn if_zip_to_vec<I1, I2, F1, F2>(valid: bool, col1: F1, col2: F2) -> Option<Vec<(usize, usize)>>
    where
        I1: Iterator<Item = usize>,
        I2: Iterator<Item = usize>,
        F1: Fn() -> I1,
        F2: Fn() -> I2,
    {
        if valid {
            Some(col1().zip(col2()).collect())
        } else {
            None
        }
    }
}

fn find_position_of(matrix: &Vec<Vec<char>>, needle: char) -> Vec<(usize, usize)> {
    matrix
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, c)| if *c == needle { Some((i, j)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {

    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 9);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 18);
    }
}
