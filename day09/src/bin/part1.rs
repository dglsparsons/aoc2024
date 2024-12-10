use std::fs;

#[derive(Debug)]
struct File {
    id: usize,
    size: usize,
}

fn parse_disk_map(input: &str) -> Vec<(Option<File>, usize)> {
    let numbers: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut blocks = Vec::new();
    let mut file_id = 0;

    // Parse alternating file and space lengths
    for (i, &size) in numbers.iter().enumerate() {
        if i % 2 == 0 {
            // File block
            if size > 0 {
                blocks.push((Some(File { id: file_id, size }), size));
                file_id += 1;
            }
        } else {
            // Space block
            blocks.push((None, size));
        }
    }

    blocks
}

fn expand_to_individual_blocks(blocks: &[(Option<File>, usize)]) -> Vec<Option<usize>> {
    let mut result = Vec::new();

    for (file_opt, size) in blocks {
        for _ in 0..*size {
            match file_opt {
                Some(file) => result.push(Some(file.id)),
                None => result.push(None),
            }
        }
    }

    result
}

fn compact_disk(blocks: &mut Vec<Option<usize>>) {
    let len = blocks.len();

    // Keep moving files from right to left until no more moves are possible
    loop {
        let mut made_move = false;

        // Find rightmost file
        for i in (0..len).rev() {
            if blocks[i].is_some() {
                // Find leftmost empty space
                for j in 0..i {
                    if blocks[j].is_none() {
                        // Move the file block
                        blocks[j] = blocks[i];
                        blocks[i] = None;
                        made_move = true;
                        break;
                    }
                }
                if made_move {
                    break;
                }
            }
        }

        if !made_move {
            break;
        }
    }
}

fn calculate_checksum(blocks: &[Option<usize>]) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, &file_id)| file_id.map(|id| pos * id))
        .sum()
}

fn main() {
    // Read input from file
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    // Parse the disk map
    let blocks = parse_disk_map(&input);

    // Expand into individual blocks
    let mut expanded_blocks = expand_to_individual_blocks(&blocks);

    // Compact the disk
    compact_disk(&mut expanded_blocks);

    // Calculate and print the checksum
    let checksum = calculate_checksum(&expanded_blocks);
    println!("Filesystem checksum: {}", checksum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "2333133121414131402";
        let blocks = parse_disk_map(input);
        let mut expanded = expand_to_individual_blocks(&blocks);
        compact_disk(&mut expanded);
        let checksum = calculate_checksum(&expanded);
        assert_eq!(checksum, 1928);
    }
}
