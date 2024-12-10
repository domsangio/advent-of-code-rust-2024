advent_of_code::solution!(2);

#[derive(PartialEq)]
enum DIRECTION {
    INCREASING,
    DECREASING,
    UNSET,
}

fn is_safe(digits: &Vec<i32>) -> bool {
    let mut prev_dir = DIRECTION::UNSET;

    for i in 0..digits.len() - 1 {
        let diff = digits[i + 1] - digits[i];
        let new_dir = if diff > 0 {
            DIRECTION::INCREASING
        } else {
            DIRECTION::DECREASING
        };

        if diff.abs() > 3 || diff.abs() == 0 || prev_dir != DIRECTION::UNSET && prev_dir != new_dir
        {
            return false;
        } else if prev_dir == DIRECTION::UNSET {
            prev_dir = new_dir;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe: i32 = 0;

    for line in input.lines() {
        let digits: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        let mut prev_dir = DIRECTION::UNSET;

        for i in 0..digits.len() - 1 {
            let diff = digits[i + 1] - digits[i];
            let new_dir = if diff > 0 {
                DIRECTION::INCREASING
            } else {
                DIRECTION::DECREASING
            };

            if diff.abs() > 3
                || diff.abs() == 0
                || prev_dir != DIRECTION::UNSET && prev_dir != new_dir
            {
                safe -= 1;
                break;
            } else if prev_dir == DIRECTION::UNSET {
                prev_dir = new_dir;
            }
        }

        safe += 1;
    }

    Some(safe as u32)
}

// Brute force, probably a better way but who cares
pub fn part_two(input: &str) -> Option<u32> {
    let mut safe = 0;

    for line in input.lines() {
        let digits: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        for i in 0..digits.len() {
            let mut vec = digits.clone();
            vec.remove(i);
            if is_safe(&vec) {
                safe += 1;
                break;
            }
        }
    }

    Some(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
