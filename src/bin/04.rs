advent_of_code::solution!(4);

fn how_many_xmas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> u64 {
    let mut count = 0;
    // check across first
    if j < grid[i].len() - 3
        && grid[i][j + 1] == 'M'
        && grid[i][j + 2] == 'A'
        && grid[i][j + 3] == 'S'
    {
        count += 1
    }
    // backwards
    if j > 2 && grid[i][j - 1] == 'M' && grid[i][j - 2] == 'A' && grid[i][j - 3] == 'S' {
        count += 1;
    }
    // up
    if i > 2 && grid[i - 1][j] == 'M' && grid[i - 2][j] == 'A' && grid[i - 3][j] == 'S' {
        count += 1;
    }
    // down
    if i < grid.len() - 3 && grid[i + 1][j] == 'M' && grid[i + 2][j] == 'A' && grid[i + 3][j] == 'S'
    {
        count += 1;
    }

    // diagonal up right
    if i > 2
        && j < grid[i].len() - 3
        && grid[i - 1][j + 1] == 'M'
        && grid[i - 2][j + 2] == 'A'
        && grid[i - 3][j + 3] == 'S'
    {
        count += 1;
    }
    // diagonal up left
    if i > 2
        && j > 2
        && grid[i - 1][j - 1] == 'M'
        && grid[i - 2][j - 2] == 'A'
        && grid[i - 3][j - 3] == 'S'
    {
        count += 1;
    }
    // diagonal down right
    if i < grid.len() - 3
        && j < grid[i].len() - 3
        && grid[i + 1][j + 1] == 'M'
        && grid[i + 2][j + 2] == 'A'
        && grid[i + 3][j + 3] == 'S'
    {
        count += 1;
    }
    // diagonal down left
    if i < grid.len() - 3
        && j > 2
        && grid[i + 1][j - 1] == 'M'
        && grid[i + 2][j - 2] == 'A'
        && grid[i + 3][j - 3] == 'S'
    {
        count += 1;
    }

    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut ret = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'X' {
                ret += how_many_xmas(&grid, i, j);
            }
        }
    }

    Some(ret)
}

fn is_x_mas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let ul = grid[i - 1][j - 1];
    let ur = grid[i - 1][j + 1];
    let dl = grid[i + 1][j - 1];
    let dr = grid[i + 1][j + 1];

    if ul == 'M' && dr == 'S' || ul == 'S' && dr == 'M' {
        if dl == 'M' && ur == 'S' || dl == 'S' && ur == 'M' {
            return true;
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ret = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if grid[i][j] == 'A' && is_x_mas(&grid, i, j) {
                // has to be an A now
                ret += 1;
            }
        }
    }

    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
