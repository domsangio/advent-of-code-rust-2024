advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mul_match = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;

    for m in mul_match.captures_iter(input) {
        let a = m[1].parse::<u32>().unwrap();
        let b = m[2].parse::<u32>().unwrap();

        total += a * b;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    // So slow bruh, maybe scan for m then do the match ?
    let pattern_match = Regex::new(r"do\(\)|don\'t\(\)|mul\(\d+,\d+\)").unwrap();

    let mut total = 0;
    let mut enabled = true;

    for m in pattern_match.find_iter(input) {
        match m.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            mul => {
                if enabled {
                    total += part_one(mul).unwrap();
                }
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
