use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

struct ParsedInput {
    pairs: Vec<(u64, u64)>,
    levels: Vec<Vec<u64>>,
}

// Return dependency pairs and then Vec of Vec of u64
fn parse_input(input: &str) -> ParsedInput {
    let mut parsed_input: ParsedInput = ParsedInput {
        pairs: Vec::new(),
        levels: Vec::new(),
    };

    let mut all_lines = input.lines();
    for line in &mut all_lines {
        match line.split_once('|') {
            Some((left, right)) => {
                parsed_input
                    .pairs
                    .push((left.parse().unwrap(), right.parse().unwrap()));
            }
            None => break,
        }
    }

    for remaining_lines in all_lines {
        parsed_input.levels.push(
            remaining_lines
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    parsed_input
}

struct DAG {
    edges: HashMap<u64, HashSet<u64>>,
}

impl DAG {
    fn new() -> DAG {
        DAG {
            edges: HashMap::new(),
        }
    }

    fn set_edges(&mut self, pairs: &Vec<(u64, u64)>) {
        for (src, dst) in pairs {
            self.add_edge(*src, *dst);
        }
    }

    fn add_edge(&mut self, src: u64, dst: u64) {
        self.edges.entry(src).or_insert(HashSet::new()).insert(dst);
    }

    fn print_dag(&self) {
        for (src, dsts) in &self.edges {
            println!(
                "{} -> {}",
                src,
                dsts.iter()
                    .fold(String::new(), |acc, x| acc + ", " + &x.to_string())
            );
        }
    }
}

fn valid_level(level: &Vec<u64>, edge_map: &DAG) -> Option<u64> {
    // println!("Validating level: {}", level.iter().fold(String::new(), |acc, x| acc + ", " + &x.to_string()));
    let mut working_set = HashSet::<u64>::new();

    for digit in level.iter().rev() {
        if working_set.contains(digit) {
            // println!("Broken! This number {digit}\nWorking set: {}", working_set.iter().fold(String::new(), |acc, x| acc + ", " + &x.to_string()));
            return None;
        }
        match edge_map.edges.get(digit) {
            None => {}
            Some(deps) => {
                for dep in deps {
                    working_set.insert(*dep);
                }
            }
        }
        working_set.insert(*digit);
    }

    return Some(level[(level.len() - 1) / 2]);
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed_input = parse_input(input);
    let mut dag = DAG::new();
    dag.set_edges(&parsed_input.pairs);

    // DEBUG ONLY
    // dag.print_dag();

    let mut middle_sum = 0;
    for level in parsed_input.levels {
        match valid_level(&level, &dag) {
            Some(digit) => middle_sum += digit,
            None => continue,
        }
    }

    Some(middle_sum)
}

fn sort_level(mut level: Vec<u64>, dag: &DAG) -> u64 {
    // println!("");
    // println!("Sorting level: {}", level.iter().fold(String::new(), |acc, x| acc + ", " + &x.to_string()));
    for i in (0..level.len() - 1).rev() {
        // println!("Current level: {}", level.iter().fold(String::new(), |acc, x| acc + ", " + &x.to_string()));
        // println!("Trying to find where {} goes", level[i]);
        if valid_level(&level[i..].to_vec(), dag) == None {
            // only if its not sorted
            for j in (i + 1..level.len()).rev() {
                let mut curr_level_slice = level[i + 1..].to_vec();
                curr_level_slice.insert(j - i, level[i]);
                // println!("Trying level slice: {}", curr_level_slice.iter().fold(String::new(), |acc, x| acc + ", " + &x.to_string()));
                if valid_level(&curr_level_slice, dag) != None {
                    // if indeed sorted, reset level and break to keep going
                    level = level[..i].to_vec();
                    level.append(&mut curr_level_slice);
                    break;
                }
            }
        }
    }

    // println!("Sorted level: {}", level.iter().fold(String::new(), |acc, x| acc + ", " + &x.to_string()));

    level[(level.len() - 1) / 2]
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed_input = parse_input(input);
    let mut dag = DAG::new();
    dag.set_edges(&parsed_input.pairs);

    // dag.print_dag();

    Some(
        parsed_input
            .levels
            .iter()
            .filter_map(|x| {
                // println!("Validating level: {}", x.iter().fold(String::new(), |acc, x| acc + ", " + &x.to_string()));
                match valid_level(x, &dag) {
                    Some(_) => None,
                    None => Some(sort_level(x.to_vec(), &dag)),
                }
            })
            .fold(0, |acc, x| acc + x),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
