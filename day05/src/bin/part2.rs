use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 2 {
            return Err("Invalid rule format".to_string());
        }

        let before = parts[0].parse().map_err(|_| "Invalid number")?;
        let after = parts[1].parse().map_err(|_| "Invalid number")?;

        Ok(Rule { before, after })
    }
}

#[derive(Debug)]
struct Update {
    pages: Vec<u32>,
}

impl FromStr for Update {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pages = s
            .split(',')
            .map(|n| n.parse().map_err(|_| "Invalid number"))
            .collect::<Result<Vec<u32>, _>>()?;
        Ok(Update { pages })
    }
}

fn build_graph(update: &Update, rules: &[Rule]) -> (HashMap<u32, Vec<u32>>, HashMap<u32, usize>) {
    let update_pages: HashSet<u32> = update.pages.iter().copied().collect();
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();

    // Initialize graph and in-degree for all pages
    for &page in &update.pages {
        graph.entry(page).or_default();
        in_degree.entry(page).or_insert(0);
    }

    // Add edges and update in-degrees
    for rule in rules {
        if update_pages.contains(&rule.before) && update_pages.contains(&rule.after) {
            graph.entry(rule.before).or_default().push(rule.after);
            *in_degree.entry(rule.after).or_insert(0) += 1;
        }
    }

    (graph, in_degree)
}

fn topological_sort(update: &Update, rules: &[Rule]) -> Option<Vec<u32>> {
    let (graph, mut in_degree) = build_graph(update, rules);
    let mut sorted = Vec::new();
    let mut queue = VecDeque::new();

    // Find all nodes with 0 in-degree
    for &page in &update.pages {
        if in_degree[&page] == 0 {
            queue.push_back(page);
        }
    }

    // Process queue
    while let Some(current) = queue.pop_front() {
        sorted.push(current);

        if let Some(neighbors) = graph.get(&current) {
            for &next in neighbors {
                *in_degree.get_mut(&next).unwrap() -= 1;
                if in_degree[&next] == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    if sorted.len() == update.pages.len() {
        Some(sorted)
    } else {
        None
    }
}

fn is_valid_order(update: &Update, rules: &[Rule]) -> bool {
    let positions: HashMap<u32, usize> = update
        .pages
        .iter()
        .enumerate()
        .map(|(i, &page)| (page, i))
        .collect();

    for rule in rules {
        if positions.contains_key(&rule.before) && positions.contains_key(&rule.after) {
            if positions[&rule.before] >= positions[&rule.after] {
                return false;
            }
        }
    }

    true
}

fn get_middle_number(pages: &[u32]) -> u32 {
    let mid_idx = (pages.len() - 1) / 2;
    pages[mid_idx]
}

fn solve_part2(input: &str) -> u32 {
    let mut sections = input.trim().split("\n\n");

    let rules: Vec<Rule> = sections
        .next()
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse().ok())
        .collect();

    let updates: Vec<Update> = sections
        .next()
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse().ok())
        .collect();

    println!(
        "Processing {} rules and {} updates",
        rules.len(),
        updates.len()
    );

    let mut sum = 0;
    let mut fixed_count = 0;

    for (i, update) in updates.iter().enumerate() {
        if !is_valid_order(update, &rules) {
            if let Some(sorted_pages) = topological_sort(update, &rules) {
                fixed_count += 1;
                let middle = get_middle_number(&sorted_pages);
                println!(
                    "Fixed update {}: {:?} -> {:?}, middle: {}",
                    i + 1,
                    update.pages,
                    sorted_pages,
                    middle
                );
                sum += middle;
            } else {
                println!(
                    "Warning: Update {} cannot be sorted (cycle detected)",
                    i + 1
                );
            }
        }
    }

    println!("Fixed {} invalid updates", fixed_count);
    println!("Final sum of middle numbers from fixed updates: {}", sum);
    sum
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let result = solve_part2(&input);
    println!("Sum of middle numbers from fixed updates: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(solve_part2(input), 123);
    }
}
