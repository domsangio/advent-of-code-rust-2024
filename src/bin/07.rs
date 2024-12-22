use std::{collections::HashSet, thread::current};

advent_of_code::solution!(7);

fn parse_line(line: &str) -> (i64, Vec<i64>) {
    line.split_once(':')
        .map(|(target, rest_of_line)| {
            (
                target.parse::<i64>().unwrap(),
                rest_of_line
                    .trim()
                    .split(" ")
                    .map(|c| c.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .unwrap()
}

fn test_line(target: &i64, options: &Vec<i64>) -> Option<i64> {
    // do this with a fold maybe ?
    let mut currently_available: HashSet<i64> = HashSet::new();

    currently_available.insert(options[0]);

    for num in options.iter().skip(1) {
        if currently_available.is_empty() {
            return None;
        }

        let mut next_available: HashSet<i64> = HashSet::new();

        for current in currently_available {
            if current * num <= *target {
                next_available.insert(current * num);
            }
            if current + num <= *target {
                next_available.insert(current + num);
            }
        }

        currently_available = next_available;
    }

    match currently_available.contains(target) {
        true => Some(*target),
        false => None,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(parse_line)
            .map(|(target, options)| test_line(&target, &options))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap() as u64)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
