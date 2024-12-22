use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    let mut total_distance: u64 = 0;

    for line in input.lines() {
        let digits: Vec<i64> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();
        if let [l, r] = digits[..] {
            let left_idx = left.binary_search(&l).unwrap_or_else(|idx| idx);
            left.insert(left_idx, l);
            let right_idx = right.binary_search(&r).unwrap_or_else(|idx| idx);
            right.insert(right_idx, r);
        }
    }

    for (l, r) in left.iter().zip(right.iter()) {
        total_distance += (r - l).abs() as u64;
    }

    return Some(total_distance);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut left: HashMap<i64, i64> = HashMap::new();
    let mut right: HashMap<i64, i64> = HashMap::new();

    let mut total_distance: u64 = 0;

    for line in input.lines() {
        let digits: Vec<i64> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();
        if let [l, r] = digits[..] {
            *left.entry(l).or_insert(0) += 1;
            *right.entry(r).or_insert(0) += 1;
        }
    }

    for (num, occurances) in left.iter() {
        total_distance += (num * occurances * right.get(num).unwrap_or(&0)) as u64;
    }

    return Some(total_distance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
