use std::fs;

#[derive(Debug, Clone)]
struct File {
    id: usize,
    size: usize,
}

fn parse_disk_map(input: &str) -> Vec<Option<usize>> {
    let numbers: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut result = Vec::new();
    let mut file_id = 0;

    for (i, &size) in numbers.iter().enumerate() {
        if i % 2 == 0 {
            // File block
            for _ in 0..size {
                result.push(Some(file_id));
            }
            if size > 0 {
                file_id += 1;
            }
        } else {
            // Space block
            for _ in 0..size {
                result.push(None);
            }
        }
    }

    result
}

fn get_files_info(blocks: &[Option<usize>]) -> Vec<(usize, usize, usize)> {
    let mut files = Vec::new();
    let mut current_id = None;
    let mut start_pos = 0;
    let mut size = 0;

    for (i, &block) in blocks.iter().enumerate() {
        match (current_id, block) {
            (None, Some(id)) => {
                current_id = Some(id);
                start_pos = i;
                size = 1;
            }
            (Some(curr_id), Some(id)) if curr_id == id => {
                size += 1;
            }
            (Some(curr_id), _) => {
                files.push((curr_id, start_pos, size));
                current_id = block;
                start_pos = i;
                size = if block.is_some() { 1 } else { 0 };
            }
            (None, None) => {}
        }
    }
    if let Some(id) = current_id {
        files.push((id, start_pos, size));
    }

    files
}

fn find_free_space(
    blocks: &[Option<usize>],
    start_limit: usize,
    size_needed: usize,
) -> Option<usize> {
    let mut current_free = 0;
    let mut free_start = 0;

    for (pos, &block) in blocks.iter().enumerate() {
        if pos >= start_limit {
            break;
        }

        if block.is_none() {
            if current_free == 0 {
                free_start = pos;
            }
            current_free += 1;
            if current_free >= size_needed {
                return Some(free_start);
            }
        } else {
            current_free = 0;
        }
    }
    None
}

fn compact_disk(blocks: &mut Vec<Option<usize>>) {
    let files = get_files_info(blocks);

    // Process files in order of decreasing ID
    for &(file_id, start_pos, file_size) in files.iter().rev() {
        // Find leftmost suitable free space
        if let Some(new_pos) = find_free_space(blocks, start_pos, file_size) {
            // Clear old position
            for i in start_pos..start_pos + file_size {
                blocks[i] = None;
            }
            // Place in new position
            for i in new_pos..new_pos + file_size {
                blocks[i] = Some(file_id);
            }
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
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let mut blocks = parse_disk_map(&input);
    compact_disk(&mut blocks);
    let checksum = calculate_checksum(&blocks);
    println!("Filesystem checksum: {}", checksum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "2333133121414131402";
        let mut blocks = parse_disk_map(input);
        compact_disk(&mut blocks);
        let checksum = calculate_checksum(&blocks);
        assert_eq!(checksum, 2858);
    }

    #[test]
    fn test_visualization() {
        let input = "2333133121414131402";
        let mut blocks = parse_disk_map(input);
        println!("Initial:");
        for block in &blocks {
            match block {
                Some(id) => print!("{}", id),
                None => print!("."),
            }
        }
        println!();

        compact_disk(&mut blocks);
        println!("After compaction:");
        for block in &blocks {
            match block {
                Some(id) => print!("{}", id),
                None => print!("."),
            }
        }
        println!();
    }
}
