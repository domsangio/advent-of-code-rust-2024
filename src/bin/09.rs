advent_of_code::solution!(9);

const FREE_SPACE_ID: i64 = -1;
const DEBUG: bool = false;

#[derive(Clone, Copy)]
struct FileBlock {
    id: i64,
    length: i64, // idt we need location since it doesnt really matter
}

// sequence of ID + number of them, free space + number of them

fn parse_input(input: &str) -> Vec<FileBlock> {
    input
        .chars()
        .enumerate()
        .map(|(pos, c)| {
            // even positions == file blocks, odd free space
            if pos % 2 == 0 {
                FileBlock {
                    id: pos as i64 / 2,
                    length: c.to_digit(10).unwrap() as i64,
                }
            } else {
                FileBlock {
                    id: FREE_SPACE_ID as i64,
                    length: c.to_digit(10).unwrap() as i64,
                }
            }
        })
        .filter(|x| !(x.id == FREE_SPACE_ID && x.length == 0))
        .collect()
}

fn print_block_seq(files: &Vec<FileBlock>) {
    for fb in files.iter() {
        if fb.id == FREE_SPACE_ID {
            println!("FREESPACE: {}", fb.length);
        } else {
            println!("ID: {}, length: {}", fb.id, fb.length);
        }
    }
}

fn print_flat_list(files: &Vec<FileBlock>) {
    for fb in files.iter() {
        for _ in 0..fb.length {
            if fb.id == FREE_SPACE_ID {
                print!(".");
            } else {
                print!("{}", fb.id);
            }
        }
    }
    println!();
}

fn calculate_checksum(files: &Vec<FileBlock>) -> i64 {
    let mut initial_pos: i64 = 0;
    let mut ret: i64 = 0;
    for curr_block in files.iter() {
        if curr_block.id == FREE_SPACE_ID {
            initial_pos += curr_block.length;
            continue;
        }
        let partial_sum = (initial_pos * 2 + curr_block.length - 1) * curr_block.length / 2;
        ret += curr_block.id * partial_sum;
        initial_pos += curr_block.length;
    }

    ret
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut original_sequence = parse_input(input);
    let mut new_sequence: Vec<FileBlock> = Vec::new();

    // add left most block since always starting with one
    let mut left_pos = 1;
    new_sequence.push(original_sequence[0]);

    // drop rightmost block if its a free space
    let mut right_pos = original_sequence.len() - 1;
    if original_sequence[right_pos].id == FREE_SPACE_ID {
        right_pos -= 1;
    }

    while left_pos <= right_pos {
        // If left most block in original sequence is not free space, add to new sequence
        let left_block = original_sequence[left_pos];
        if left_block.id != FREE_SPACE_ID {
            new_sequence.push(left_block);
            left_pos += 1;
            continue;
        }

        // if right most block is free space skip it
        let right_block = original_sequence[right_pos];
        if right_block.id == FREE_SPACE_ID {
            right_pos -= 1;
            continue;
        }

        // at this point we have left block is free space, right block is not free space
        // case free space is bigger than right block
        // - add right block to new sequence
        // - deduce length of free space in original seq of LEFT
        // - deduct right pos by 2 because we skip free space
        if left_block.length > right_block.length {
            original_sequence[left_pos].length -= right_block.length;
            new_sequence.push(right_block);
            right_pos -= 1;
        } else if left_block.length == right_block.length {
            // case they are equal
            // - add right block to new seq
            // - inc left by 2 (since we know next is not free space)
            // - dec right by 2
            new_sequence.push(right_block);
            right_pos -= 1;
            left_pos += 1;
        } else {
            // left block is less than right block
            // - add right block but of length left block
            // - dec right block length by left
            // - inc left pos by 2
            new_sequence.push(FileBlock {
                id: right_block.id,
                length: left_block.length,
            });
            left_pos += 1;
            original_sequence[right_pos].length -= left_block.length;
        }
    }

    Some(calculate_checksum(&new_sequence) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut original_sequence = parse_input(input);

    // can skip first element
    let mut right_pos = original_sequence.len() - 1;
    while right_pos > 0 {
        let right_block = original_sequence[right_pos];
        if right_block.id == FREE_SPACE_ID {
            right_pos -= 1;
            continue;
        }

        for left_pos in 0..right_pos {
            let left_block = original_sequence[left_pos];
            if left_block.id == FREE_SPACE_ID && left_block.length >= right_block.length {
                if DEBUG {
                    println!("BEFORE");
                    print_flat_list(&original_sequence);
                }

                // insert this into this block
                // can insert a new block at left_pos that is == right_block, change right block id to free space
                // and reduce length of left_block (which is now pos + 1)
                original_sequence[right_pos].id = FREE_SPACE_ID;
                original_sequence[left_pos].length -= right_block.length;
                original_sequence.insert(left_pos, right_block);

                if left_block.length != right_block.length {
                    right_pos += 1; // increment because extended the list TODO maybe tho only if size wasnt equal
                }

                if DEBUG {
                    println!("AFTER");
                    print_flat_list(&original_sequence);
                    println!();
                }

                break;
            }
        }

        right_pos -= 1;
    }

    Some(calculate_checksum(&original_sequence) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
