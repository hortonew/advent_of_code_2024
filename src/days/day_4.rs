pub fn run() {
    let search_results = word_search("src/days/inputs/4.txt");
    println!("Day 4: XMAS: {}", search_results);

    let x_mas_search = word_search_x_mas("src/days/inputs/4.txt");
    println!("Day 4: X-MAS: {}", x_mas_search);
}

fn word_search(file: &str) -> i32 {
    // convert lines to vectors
    let input = std::fs::read_to_string(file).expect("File doesn't exist");
    let grid: Vec<&str> = input.lines().collect();

    let word = "XMAS";
    let mut total_matches = search_grid(&grid, word);
    let word: String = word.chars().rev().collect();
    total_matches += search_grid(&grid, &word);
    total_matches as i32
}

fn word_search_x_mas(file: &str) -> i32 {
    let input = std::fs::read_to_string(file).expect("File doesn't exist");
    let grid: Vec<&str> = input.lines().collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_matches = 0;

    for r in 0..rows - 2 {
        for c in 0..cols - 2 {
            // Check diagonals directly from the grid
            if is_x_mas(&grid[r..r + 3].iter().map(|line| &line[c..c + 3]).collect::<Vec<_>>()) {
                total_matches += 1;
            }
        }
    }
    total_matches
}

fn is_x_mas(subgrid: &[&str]) -> bool {
    // Extract diagonal elements
    let top_left_to_bottom_right = (
        subgrid[0].chars().nth(0).unwrap(),
        subgrid[1].chars().nth(1).unwrap(),
        subgrid[2].chars().nth(2).unwrap(),
    );
    let top_right_to_bottom_left = (
        subgrid[0].chars().nth(2).unwrap(),
        subgrid[1].chars().nth(1).unwrap(),
        subgrid[2].chars().nth(0).unwrap(),
    );

    // Define the valid "X" patterns
    let mas = ('M', 'A', 'S');
    let sam = ('S', 'A', 'M');

    // Both diagonals must match one of the valid patterns
    [mas, sam].contains(&top_left_to_bottom_right) && [mas, sam].contains(&top_right_to_bottom_left)
}

fn count_overlapping_substrings(haystack: &str, needle: &str) -> usize {
    // Counts overlapping substrings of `needle` in `haystack`.
    let mut count = 0;
    let mut start = 0;

    while let Some(pos) = haystack[start..].find(needle) {
        count += 1;
        start += pos + 1; // Move one character forward to allow overlap
    }

    count
}

fn search_grid(grid: &[&str], word: &str) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut total = 0;

    // Search horizontally
    total += grid
        .iter()
        .map(|row| count_overlapping_substrings(row, word))
        .sum::<usize>();

    // Search vertically
    total += (0..cols)
        .map(|col| {
            let column: String = (0..rows).map(|row| grid[row].chars().nth(col).unwrap()).collect();
            count_overlapping_substrings(&column, word)
        })
        .sum::<usize>();

    // Search diagonally
    total += extract_diagonals(grid)
        .iter()
        .map(|diag| count_overlapping_substrings(diag, word))
        .sum::<usize>();

    total
}

fn extract_diagonals(grid: &[&str]) -> Vec<String> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut diagonals = Vec::new();

    // Top-left to bottom-right (↘)
    for d in 0..(rows + cols - 1) {
        let diagonal: String = (0..=d)
            .filter_map(|i| {
                let j = d - i;
                if i < rows && j < cols {
                    Some(grid[i].chars().nth(j).unwrap())
                } else {
                    None
                }
            })
            .collect();
        diagonals.push(diagonal);
    }

    // Top-right to bottom-left (↙)
    for d in 0..(rows + cols - 1) {
        let diagonal: String = (0..=d)
            .filter_map(|i| {
                let j = cols.checked_sub(1 + (d - i));
                if let Some(j) = j {
                    if i < rows {
                        return Some(grid[i].chars().nth(j).unwrap());
                    }
                }
                None
            })
            .collect();
        diagonals.push(diagonal);
    }

    diagonals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4_example() {
        assert_eq!(word_search("src/days/inputs/4_test.txt"), 18)
    }

    #[test]
    fn test_4() {
        assert_eq!(word_search("src/days/inputs/4.txt"), 2336)
    }

    #[test]
    fn test_4_example_2() {
        assert_eq!(word_search_x_mas("src/days/inputs/4_test.txt"), 9)
    }
    #[test]
    fn test_4_2() {
        assert_eq!(word_search_x_mas("src/days/inputs/4.txt"), 1831)
    }
}
