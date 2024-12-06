use std::collections::HashMap;
use std::fs;

pub fn run() {
    let result = calculate_middle_sum("src/days/inputs/5.txt");
    println!("Day 5: Total: {}", result.0);
    println!("Day 5: Incorrect Total: {}", result.1);
}

fn parse_rules<'a>(rules: &'a [&'a str]) -> Vec<(&'a str, &'a str)> {
    rules
        .iter()
        .map(|rule| {
            let parts: Vec<&str> = rule.split('|').collect();
            (parts[0], parts[1])
        })
        .collect()
}

fn is_valid_sequence(sequence: &[&str], rules: &[(&str, &str)]) -> bool {
    // Map elements to their indices in the sequence
    let mut position_map: HashMap<&str, usize> = HashMap::new();
    for (i, &element) in sequence.iter().enumerate() {
        position_map.insert(element, i);
    }

    // Check each rule
    for &(from, to) in rules {
        if let (Some(&from_idx), Some(&to_idx)) = (position_map.get(from), position_map.get(to)) {
            if from_idx >= to_idx {
                return false; // Rule violated
            }
        }
    }

    true
}

fn reorder_sequence<'a>(sequence: &'a [&'a str], rules: &'a [(&'a str, &'a str)]) -> Vec<&'a str> {
    let mut pages: Vec<&str> = sequence.to_vec();

    // Sort pages based on the rules
    pages.sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            std::cmp::Ordering::Less // a should come before b
        } else if rules.contains(&(*b, *a)) {
            std::cmp::Ordering::Greater // b should come before a
        } else {
            std::cmp::Ordering::Equal // No specific order
        }
    });

    pages
}

fn calculate_middle_sum(file: &str) -> (i32, i32) {
    // Read and parse the input file
    let input = fs::read_to_string(file).expect("File doesn't exist");
    let mut sections = input.split("\n\n");

    // Parse the rules from the first section
    let first_section = sections.next().expect("Missing rules section");
    let rules: Vec<&str> = first_section.lines().collect();
    let parsed_rules = parse_rules(&rules);

    // Parse the sequences from the second section
    let second_section = sections.next().expect("Missing sequences section");
    let sequences: Vec<Vec<&str>> = second_section.lines().map(|line| line.split(',').collect()).collect();

    // Validate sequences and calculate totals
    let mut total = 0;
    let mut incorrect_total = 0;
    for sequence in &sequences {
        if is_valid_sequence(sequence, &parsed_rules) {
            // Valid sequence: calculate middle value
            let middle_index = sequence.len() / 2;
            total += sequence[middle_index].parse::<i32>().unwrap();
        } else {
            // Invalid sequence: reorder and calculate middle value
            let reordered_sequence = reorder_sequence(sequence, &parsed_rules);
            let middle_index = reordered_sequence.len() / 2;
            incorrect_total += reordered_sequence[middle_index].parse::<i32>().unwrap();
        }
    }

    (total, incorrect_total)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_5_example() {
        assert_eq!(calculate_middle_sum("src/days/inputs/5_test.txt").0, 143);
    }
    #[test]
    fn test_5() {
        assert_eq!(calculate_middle_sum("src/days/inputs/5.txt").0, 5091);
    }

    #[test]
    fn test_5_example_2() {
        assert_eq!(calculate_middle_sum("src/days/inputs/5_test.txt").1, 123);
    }
    #[test]
    fn test_5_2() {
        assert_eq!(calculate_middle_sum("src/days/inputs/5.txt").1, 4681);
    }
}
