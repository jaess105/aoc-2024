use crate::aoc_day::AocDayData;
use std::collections::HashSet;
use std::usize;

pub fn day() -> AocDayData {
    AocDayData::new(9, "resources/day09".to_string(), solve_a, solve_b)
}

fn solve_b(input: String) -> i64 {
    let mut spaces = parse_input(input);
    let mut handled_ids = HashSet::new();

    loop {
        spaces = merge_space(&mut spaces);

        let last_taken_pos = match last_unhandled(&spaces, &handled_ids) {
            Some(pos) => pos,
            None => break,
        };
        let last_taken = spaces[last_taken_pos];
        handled_ids.insert(last_taken.id().unwrap());

        let free_pos = spaces.iter().enumerate().find_map(|(i, s)| {
            if s.is_taken() || s.size() < last_taken.size() || i > last_taken_pos {
                None
            } else {
                Some(i)
            }
        });
        if free_pos.is_none() {
            continue;
        }

        let free_pos = free_pos.unwrap();
        let free = spaces[free_pos];

        let diff = free.size() as i64 - last_taken.size() as i64;
        match diff {
            // equal size
            0 => {
                // they can simply switch due to the same space.
                spaces[last_taken_pos] = free;
                spaces[free_pos] = last_taken;
            }
            // more free space than needed
            diff if diff > 0 => {
                // new free space where the taken element was removed
                spaces[last_taken_pos] = Space::Free(last_taken.size());
                // insert the remaining free space at the old position
                spaces[free_pos] = Space::Free(diff as usize);
                // calling insert will shift everything else to the right, including the reintroduced free space.
                spaces.insert(free_pos, last_taken);
            }
            // less free space than needed. Is not possible anymore.
            _ => panic!("Invalid diff {}", diff),
        };
    }

    check_sum(spaces)
}

fn merge_space(spaces: &mut Vec<Space>) -> Vec<Space> {
    spaces.iter().fold(Vec::new(), |mut acc, el| {
        // We have to copy here to not hold a reference onto an element of the acc.
        if let Some(last) = acc.last().map(|last| *last) {
            match last {
                Space::Taken(size, id) if el.is_taken() && el.id().unwrap() == id => {
                    // pop the last merge both as they have the same id
                    acc.pop();
                    acc.push(Space::Taken(size + el.size(), id));
                    return acc;
                }
                Space::Free(size) if !el.is_taken() => {
                    // both are free, pop the last and merge both into one
                    acc.pop();
                    acc.push(Space::Free(size + el.size()));
                    return acc;
                }
                _ => {}
            }
        }

        acc.push(*el);
        acc
    })
}

fn solve_a(input: String) -> i64 {
    let mut spaces = parse_input(input);

    loop {
        let first_free_pos = first_free(&spaces);
        let last_taken_pos = match last_unhandled(&spaces, &HashSet::new()) {
            Some(pos) => pos,
            None => break,
        };
        if last_taken_pos < first_free_pos {
            break;
        }

        let first_free = spaces[first_free_pos];
        let last_taken = spaces[last_taken_pos];

        let diff = first_free.size() as i64 - last_taken.size() as i64;
        match diff {
            // equal size
            0 => {
                // they can simply switch due to the same space.
                spaces[last_taken_pos] = first_free;
                spaces[first_free_pos] = last_taken;
            }
            // less free space than needed
            diff if diff < 0 => {
                // free space is completely taken over by id element and the diff remains at the end of the list.
                let last_taken_id = last_taken.id().unwrap();
                // insert in the beginning as much as can be
                spaces[first_free_pos] = Space::Taken(first_free.size(), last_taken_id);
                // move the free space to the old position of the taken
                spaces[last_taken_pos] = Space::Free(first_free.size());
                // insert the difference that was remaining from the taken space to shift the free space
                // to the right.
                spaces.insert(
                    last_taken_pos,
                    Space::Taken(diff.abs() as usize, last_taken_id),
                );
            }
            // more free space than needed
            diff if diff > 0 => {
                // new free space where the taken element was removed
                spaces[last_taken_pos] = Space::Free(last_taken.size());
                // insert the remaining free space at the old position
                spaces[first_free_pos] = Space::Free(diff as usize);
                // calling insert will shift everything else to the right, including the reintroduced free space.
                spaces.insert(first_free_pos, last_taken);
            }
            _ => panic!("How can something be not less, not more and not equal to 0..."),
        };
    }

    check_sum(spaces)
}

fn parse_input(input: String) -> Vec<Space> {
    input
        .split("")
        .filter(|&s| s != "")
        .enumerate()
        .map(|(i, s)| {
            let space = usize::from_str_radix(s, 10).unwrap();
            if i % 2 == 0 {
                Space::Taken(space, (i / 2) as u32)
            } else {
                Space::Free(space)
            }
        })
        .filter(|el| el.size() != 0)
        .collect::<Vec<Space>>()
}

fn check_sum(spaces: Vec<Space>) -> i64 {
    spaces
        .iter()
        .fold((0, 0), |(pos, mut acc), s| {
            if let Space::Taken(size, id) = s {
                for i in pos..pos + size {
                    acc += (i as i64) * (*id as i64)
                }
                (pos + size, acc)
            } else {
                (pos + s.size(), acc)
            }
        })
        .1
}

fn first_free(spaces: &Vec<Space>) -> usize {
    spaces
        .iter()
        .enumerate()
        .find_map(|(i, space)| if !space.is_taken() { Some(i) } else { None })
        .unwrap()
}

fn last_unhandled(spaces: &Vec<Space>, handled_ids: &HashSet<u32>) -> Option<usize> {
    spaces
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, space)| space.is_taken())
        .filter(|(_, space)| !handled_ids.contains(&space.id().unwrap()))
        .find_map(|(i, s)| if s.is_taken() { Some(i) } else { None })
}

/// First argument is always the size of the block, the second on taken, is the Id placed in that block.
#[derive(Copy, Clone, Debug)]
enum Space {
    Free(usize),
    Taken(usize, u32),
}

impl Space {
    pub fn size(&self) -> usize {
        match self {
            Space::Free(size) => *size,
            Space::Taken(size, _) => *size,
        }
    }

    pub fn is_taken(&self) -> bool {
        match self {
            Space::Free(_) => false,
            Space::Taken(_, _) => true,
        }
    }

    pub fn id(&self) -> Option<u32> {
        match self {
            Space::Free(_) => None,
            Space::Taken(_, id) => Some(*id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 2858);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 1928);
    }
}
