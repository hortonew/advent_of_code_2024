use regex::Regex;

pub fn run() {
    let total = unscramble_and_sum("src/days/inputs/3.txt");
    println!("Day 3: mul(): {}", total);

    let total_2 = unscramble_and_sum_with_do_and_dont("src/days/inputs/3.txt");
    println!("Day 3: do_and_dont: {}", total_2);
}

fn unscramble_and_sum(file: &str) -> i32 {
    std::fs::read_to_string(file)
        .expect("File could not be parsed")
        .lines()
        .map(process_line)
        .sum::<i32>()
}

fn process_line(line: &str) -> i32 {
    // Define the regex to match mul(x, y)
    let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();

    // Iterate through all matches, parse and multiply numbers, and collect the results
    re.captures_iter(line)
        .map(|caps| {
            let x: i32 = caps[1].parse().expect("Not a number");
            let y: i32 = caps[2].parse().expect("Not a number");
            x * y
        })
        .sum()
}

fn unscramble_and_sum_with_do_and_dont(file: &str) -> i32 {
    let mut total = 0;
    let mut valid = true;
    let instructions = std::fs::read_to_string(file)
        .expect("File could not be parsed")
        .lines()
        .flat_map(process_line_do_dont) // Flatten all parsed commands
        .collect::<Vec<_>>();

    for instruction in instructions {
        if instruction == "don't()" {
            valid = false;
        } else if instruction == "do()" {
            valid = true;
        } else if valid && instruction.starts_with("mul(") {
            total += process_line(&instruction);
        }
    }
    total
}

fn process_line_do_dont(line: &str) -> Vec<String> {
    // Define the regex to match mul(x, y)
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut results = Vec::new();
    for caps in re.captures_iter(line) {
        results.push(caps[0].to_string());
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_example_a() {
        // 2*4 + 5*5 + 11*8 + 8*5 = 161
        assert_eq!(unscramble_and_sum("src/days/inputs/3_test.txt"), 161)
    }
    #[test]
    fn test_3_a() {
        assert_eq!(unscramble_and_sum("src/days/inputs/3.txt"), 181345830)
    }

    #[test]
    fn test_3_example_b() {
        // 2*4 + 8*5
        assert_eq!(unscramble_and_sum_with_do_and_dont("src/days/inputs/3_test_b.txt"), 48)
    }
    #[test]
    fn test_3_b() {
        assert_eq!(unscramble_and_sum_with_do_and_dont("src/days/inputs/3.txt"), 98729041)
    }
}
