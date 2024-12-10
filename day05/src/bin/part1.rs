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

fn build_graph(update: &Update, rules: &[Rule]) -> HashMap<u32, Vec<u32>> {
    let update_pages: HashSet<u32> = update.pages.iter().copied().collect();
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();

    // Initialize graph for all pages in the update
    for &page in &update.pages {
        graph.entry(page).or_default();
    }

    // Add edges for applicable rules
    for rule in rules {
        if update_pages.contains(&rule.before) && update_pages.contains(&rule.after) {
            graph.entry(rule.before).or_default().push(rule.after);
        }
    }

    graph
}

fn has_cycle(
    graph: &HashMap<u32, Vec<u32>>,
    start: u32,
    visited: &mut HashSet<u32>,
    path: &mut HashSet<u32>,
) -> bool {
    if path.contains(&start) {
        return true;
    }
    if visited.contains(&start) {
        return false;
    }

    visited.insert(start);
    path.insert(start);

    if let Some(neighbors) = graph.get(&start) {
        for &next in neighbors {
            if has_cycle(graph, next, visited, path) {
                return true;
            }
        }
    }

    path.remove(&start);
    false
}

fn is_valid_order(update: &Update, rules: &[Rule]) -> bool {
    let graph = build_graph(update, rules);
    let positions: HashMap<u32, usize> = update
        .pages
        .iter()
        .enumerate()
        .map(|(i, &page)| (page, i))
        .collect();

    // Check for cycles first
    let mut visited = HashSet::new();
    let mut path = HashSet::new();
    for &page in &update.pages {
        if !visited.contains(&page) {
            if has_cycle(&graph, page, &mut visited, &mut path) {
                println!("Update {:?} has a cycle", update.pages);
                return false;
            }
        }
    }

    // Then check if all rules are satisfied by the current order
    let mut violations = Vec::new();
    for rule in rules {
        if positions.contains_key(&rule.before) && positions.contains_key(&rule.after) {
            if positions[&rule.before] >= positions[&rule.after] {
                violations.push(rule);
            }
        }
    }

    if !violations.is_empty() {
        println!("Update {:?} has violations:", update.pages);
        for rule in violations {
            println!("  Rule {}|{} violated", rule.before, rule.after);
        }
        return false;
    }

    true
}

fn get_middle_number(pages: &[u32]) -> u32 {
    let mid_idx = (pages.len() - 1) / 2;
    pages[mid_idx]
}

fn solve(input: &str) -> u32 {
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

    let mut valid_count = 0;
    let mut sum = 0;
    for (i, update) in updates.iter().enumerate() {
        if is_valid_order(update, &rules) {
            valid_count += 1;
            let middle = get_middle_number(&update.pages);
            println!(
                "Update {} is valid - {:?}, middle number: {}",
                i + 1,
                update.pages,
                middle
            );
            sum += middle;
        } else {
            println!("Update {} is invalid - {:?}", i + 1, update.pages);
        }
    }

    println!(
        "Found {} valid updates out of {}",
        valid_count,
        updates.len()
    );
    println!("Final sum: {}", sum);
    sum
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let result = solve(&input);
    println!("Sum of middle numbers from valid updates: {}", result);
}
