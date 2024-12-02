pub fn run() {
    let num_of_reports_safe = num_of_reports_safe("src/days/inputs/2.txt");
    println!("Day 2: Safe Reports: {}", num_of_reports_safe);

    let reports_safe_with_pd = num_of_reports_safe_with_problem_dampener("src/days/inputs/2.txt");
    println!("Day 2: Safe Reports with PD: {}", reports_safe_with_pd);
}

fn num_of_reports_safe(file: &str) -> i32 {
    let input = std::fs::read_to_string(file).expect("Failed to read file");
    let lines: Vec<&str> = input.lines().collect();
    let mut total_safe = 0;

    for line in lines {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Failed to parse number"))
            .collect();

        if is_safe(&levels) {
            total_safe += 1;
            continue;
        }
    }
    total_safe
}

fn num_of_reports_safe_with_problem_dampener(file: &str) -> i32 {
    let input = std::fs::read_to_string(file).expect("Failed to read file");
    let lines: Vec<&str> = input.lines().collect();
    let mut total_safe = 0;

    for line in lines {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Failed to parse number"))
            .collect();

        if is_safe(&levels) {
            total_safe += 1;
            continue;
        }

        // Try removing each level and recheck safety
        let mut dampened_safe = false;
        for i in 0..levels.len() {
            // Create a new vector with the `i`-th level removed
            let mut modified_levels = levels.clone();
            modified_levels.remove(i);

            if is_safe(&modified_levels) {
                dampened_safe = true;
                break;
            }
        }

        if dampened_safe {
            total_safe += 1;
        }
    }
    total_safe
}

// Helper function to check if a report is safe
fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let increasing = levels[1] >= levels[0];

    for i in 1..levels.len() {
        let distance = (levels[i] - levels[i - 1]).abs();
        if (distance == 0 || distance > 3)
            || (increasing && levels[i] < levels[i - 1])
            || (!increasing && levels[i] > levels[i - 1])
        {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_of_reports_safe() {
        assert_eq!(num_of_reports_safe("src/days/inputs/2_test.txt"), 2);
    }

    #[test]
    fn test_num_of_reports_safe_with_problem_dampener() {
        assert_eq!(
            num_of_reports_safe_with_problem_dampener("src/days/inputs/2_test.txt"),
            4
        );
    }
}
