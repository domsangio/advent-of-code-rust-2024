use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum DIR {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

fn in_bounds(width: i32, height: i32, pos: &(i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height
}

fn turn(dir: &DIR) -> DIR {
    match dir {
        DIR::LEFT => DIR::UP,
        DIR::UP => DIR::RIGHT,
        DIR::RIGHT => DIR::DOWN,
        DIR::DOWN => DIR::LEFT,
    }
}

fn get_next_pos(dir: &DIR, pos: &(i32, i32)) -> (i32, i32) {
    match dir {
        DIR::LEFT => (pos.0, pos.1 - 1),
        DIR::RIGHT => (pos.0, pos.1 + 1),
        DIR::UP => (pos.0 - 1, pos.1),
        DIR::DOWN => (pos.0 + 1, pos.1),
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, DIR, (i32, i32)) {
    let mut pos = (-1, -1);
    let mut dir = DIR::DOWN;
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    match c {
                        '^' | '>' | 'v' | '<' => {
                            pos = (row as i32, col as i32);
                            dir = match c {
                                '^' => DIR::UP,
                                '>' => DIR::RIGHT,
                                'v' => DIR::DOWN,
                                '<' | _ => DIR::LEFT,
                            };
                        }
                        _ => (),
                    }
                    c
                })
                .collect()
        })
        .collect();

    (grid, dir, pos)
}

fn get_visited_pos(
    start_dir: &DIR,
    start_pos: &(i32, i32),
    grid: &Vec<Vec<char>>,
) -> HashSet<(i32, i32)> {
    let width = grid.len();
    let height = grid[0].len();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut dir = *start_dir;
    let mut pos = start_pos.to_owned();

    while in_bounds(width as i32, height as i32, &pos) {
        // if weve never been here increment and add to visited
        if !visited.contains(&pos) {
            visited.insert(pos);
        }

        // move the pos
        let next_pos = get_next_pos(&dir, &pos);

        // turn if we hit an obstacle
        if in_bounds(width as i32, height as i32, &next_pos)
            && grid[next_pos.0 as usize][next_pos.1 as usize] == '#'
        {
            dir = turn(&dir);
            pos = get_next_pos(&dir, &pos);
        } else {
            // otherwise move there
            pos = next_pos;
        }
    }

    visited
}

// breaks if in a loop or out of bounds
fn get_visited_pos_and_dir(
    start_dir: &DIR,
    start_pos: &(i32, i32),
    grid: &Vec<Vec<char>>,
) -> Option<HashSet<(DIR, i32, i32)>> {
    let width = grid.len();
    let height = grid[0].len();

    let mut visited: HashSet<(DIR, i32, i32)> = HashSet::new();
    let mut dir = *start_dir;
    let mut pos = start_pos.to_owned();

    // use less recursion to speed up
    while in_bounds(width as i32, height as i32, &pos) {
        let curr_pos = (dir, pos.0, pos.1);

        // if weve been exactly here before we are in a loop
        if !visited.insert(curr_pos) {
            return Some(visited);
        }

        // move the pos
        let next_pos = get_next_pos(&dir, &pos);

        // turn if we hit an obstacle
        if in_bounds(width as i32, height as i32, &next_pos)
            && grid[next_pos.0 as usize][next_pos.1 as usize] == '#'
        {
            dir = turn(&dir);
            pos = get_next_pos(&dir, &pos);
        } else {
            // otherwise move there
            pos = next_pos;
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, dir, pos) = parse_input(input);
    Some(get_visited_pos(&dir, &pos, &grid).len() as u32)
}

fn debug_loop(l: &HashSet<(DIR, i32, i32)>) {
    // TODO idek how to debug this ?
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, start_dir, start_pos) = parse_input(input);
    let path = get_visited_pos(&start_dir, &start_pos, &grid);
    let mut tampered_grid = grid.clone();

    Some(
        path.iter()
            .filter(|pos| {
                if **pos == start_pos {
                    return false;
                }
                // add an obstacle here, run if we hit a loop, remove the obstacle (know its on path so not obstacle)
                tampered_grid[pos.0 as usize][pos.1 as usize] = '#';
                let f = match get_visited_pos_and_dir(&start_dir, &start_pos, &tampered_grid) {
                    Some(l) => {
                        debug_loop(&l);
                        true
                    }
                    None => false,
                };
                tampered_grid[pos.0 as usize][pos.1 as usize] = '.';
                f
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
