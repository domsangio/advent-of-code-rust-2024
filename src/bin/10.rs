advent_of_code::solution!(10);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn is_in_bounds(grid: &Vec<Vec<u8>>, x: i16, y: i16)

fn get_valid_neighbors(grid: &Vec<Vec<u8>>) -> HashMap<(usize, usize), u8> {
    for i in 0..grid.len() {
        for j in grid[0].len() {

        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let topo = parse_input(input);



    None
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
