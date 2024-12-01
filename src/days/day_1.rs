pub fn run() {
    let (left_vec, right_vec) = process_file("src/days/inputs/1.txt");

    // 1a
    let total_distance = calculate_total_distance(&left_vec, &right_vec);
    println!("Total distance: {}", total_distance);

    // 1b
    let similarity_score = calculate_similarity_score(&left_vec, &right_vec);
    println!("Similarity score: {}", similarity_score);
}

fn process_file(file: &str) -> (std::vec::Vec<i32>, std::vec::Vec<i32>) {
    let input = std::fs::read_to_string(file).expect("Failed to read file");
    let lines: Vec<&str> = input.lines().collect();
    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();

    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Failed to parse number"))
            .collect();

        if numbers.len() == 2 {
            left_vec.push(numbers[0]);
            right_vec.push(numbers[1]);
        }
    }
    left_vec.sort();
    right_vec.sort();
    (left_vec, right_vec)
}

fn calculate_total_distance(left_vec: &[i32], right_vec: &[i32]) -> i32 {
    let mut total_distance = 0;
    for i in 0..left_vec.len() {
        let distance = right_vec[i] - left_vec[i];
        total_distance += distance.abs();
    }
    total_distance
}

fn calculate_similarity_score(left_vec: &[i32], right_vec: &[i32]) -> i32 {
    let mut similarity_score = 0;
    // for every number in the left_vec, find how many times it appears in the right vec
    // then multiple the number by the number of times it appears and add it to similarity score
    for &left_num in left_vec {
        let count = right_vec.iter().filter(|&num| *num == left_num).count();
        similarity_score += left_num * count as i32;
    }
    similarity_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_distance() {
        let (left_vec, right_vec) = process_file("src/days/inputs/1_test.txt");
        let total_distance = calculate_total_distance(&left_vec, &right_vec);
        assert_eq!(total_distance, 11);
    }

    #[test]
    fn test_calculate_similarity_score() {
        let (left_vec, right_vec) = process_file("src/days/inputs/1_test.txt");
        let similarity_score = calculate_similarity_score(&left_vec, &right_vec);
        assert_eq!(similarity_score, 31);
    }
}
