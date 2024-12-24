use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

fn parse_input(input: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<(i64, i64)>>) {
    let mut letter_locations: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .fold(Vec::<char>::new(), |mut acc, (y, c)| {
                    if c != '.' {
                        let curr_loc = (x as i64, y as i64);
                        letter_locations
                            .entry(c)
                            .or_insert(Vec::<(i64, i64)>::new())
                            .push(curr_loc);
                    }
                    acc.push(c);
                    acc
                })
        })
        .collect();

    (grid, letter_locations)
}

fn is_in_grid(grid: &Vec<Vec<char>>, location: &(i64, i64)) -> bool {
    location.0 >= 0
        && location.0 < grid.len() as i64
        && location.1 >= 0
        && location.1 < grid[0].len() as i64
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, letter_locations) = parse_input(input);
    let mut unique_locs: HashSet<(i64, i64)> = HashSet::new();

    for (c, locs) in letter_locations.iter() {
        if locs.len() < 2 {
            continue;
        }

        // println!("Looking for char: {}", c);

        for (i, loc_a) in locs.iter().take(locs.len() - 1).enumerate() {
            for loc_b in locs.iter().skip(i + 1) {
                // println!(
                //     "Checking between position 1: ({},{}) and position 2: ({}, {})",
                //     loc_a.0, loc_a.1, loc_b.0, loc_b.1
                // );

                let change_in_y = loc_a.1 - loc_b.1;
                let change_in_x = loc_a.0 - loc_b.0;

                // pos a + slope == pos b --> pos a - slope + pos_b + slope
                if (loc_a.0 + change_in_x, loc_a.1 + change_in_y) == *loc_b {
                    let p0 = (loc_a.0 - change_in_x, loc_a.1 - change_in_y);
                    let p3 = (loc_b.0 + change_in_x, loc_b.1 + change_in_y);

                    if is_in_grid(&grid, &p0) {
                        // println!("Yielding a node at: ({}, {})", p0.0, p0.1);
                        unique_locs.insert(p0);
                    }
                    if is_in_grid(&grid, &p3) {
                        // println!("Yielding a node at: ({}, {})", p3.0, p3.1);
                        unique_locs.insert(p3);
                    }
                } else {
                    let p0 = (loc_b.0 - change_in_x, loc_b.1 - change_in_y);
                    let p3 = (loc_a.0 + change_in_x, loc_a.1 + change_in_y);

                    if is_in_grid(&grid, &p0) {
                        // println!("Yielding a node at: ({}, {})", p0.0, p0.1);
                        unique_locs.insert(p0);
                    }
                    if is_in_grid(&grid, &p3) {
                        // println!("Yielding a node at: ({}, {})", p3.0, p3.1);
                        unique_locs.insert(p3);
                    }
                }
            }
        }
    }

    Some(unique_locs.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, letter_locations) = parse_input(input);
    let mut unique_locs: HashSet<(i64, i64)> = HashSet::new();

    for (_, locs) in letter_locations.iter() {
        if locs.len() < 2 {
            continue;
        }

        for (i, loc_a) in locs.iter().take(locs.len() - 1).enumerate() {
            for loc_b in locs.iter().skip(i + 1) {
                let change_in_y = loc_a.1 - loc_b.1;
                let change_in_x = loc_a.0 - loc_b.0;

                // try going left first (taking a point arbitrarily, subbing slope til out of bounds)
                let mut start = *loc_a;
                while is_in_grid(&grid, &start) {
                    unique_locs.insert(start);
                    start.0 -= change_in_x;
                    start.1 -= change_in_y;
                }
                let mut start = *loc_a;
                while is_in_grid(&grid, &start) {
                    unique_locs.insert(start);
                    start.0 += change_in_x;
                    start.1 += change_in_y;
                }
            }
        }
    }

    Some(unique_locs.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
